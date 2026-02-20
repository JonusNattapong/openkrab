#[derive(Debug, Clone, Copy)]
pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
}

pub const IMAGE_REDUCE_QUALITY_STEPS: [u8; 6] = [85, 75, 65, 55, 45, 35];

pub fn build_image_resize_side_grid(max_side: u32, side_start: u32) -> Vec<u32> {
    let candidates = [side_start, 1800, 1600, 1400, 1200, 1000, 800];
    let mut result: Vec<u32> = candidates
        .iter()
        .map(|&v| v.min(max_side))
        .filter(|&v| v > 0)
        .collect();
    result.sort_by(|a, b| b.cmp(a));
    result.dedup();
    result
}

fn read_jpeg_exif_orientation(buffer: &[u8]) -> Option<u8> {
    if buffer.len() < 2 || buffer[0] != 0xFF || buffer[1] != 0xD8 {
        return None;
    }

    let mut offset = 2;
    while offset < buffer.len() - 4 {
        if buffer[offset] != 0xFF {
            offset += 1;
            continue;
        }

        let marker = buffer[offset + 1];

        if marker == 0xFF {
            offset += 1;
            continue;
        }

        if marker == 0xE1 {
            let exif_start = offset + 4;

            if buffer.len() > exif_start + 6
                && &buffer[exif_start..exif_start + 4] == b"Exif"
                && buffer[exif_start + 4] == 0
                && buffer[exif_start + 5] == 0
            {
                let tiff_start = exif_start + 6;
                if buffer.len() < tiff_start + 8 {
                    return None;
                }

                let byte_order = &buffer[tiff_start..tiff_start + 2];
                let is_little_endian = byte_order == b"II";

                let read_u16 = |pos: usize| -> u16 {
                    if is_little_endian {
                        u16::from_le_bytes([buffer[pos], buffer[pos + 1]])
                    } else {
                        u16::from_be_bytes([buffer[pos], buffer[pos + 1]])
                    }
                };

                let read_u32 = |pos: usize| -> u32 {
                    if is_little_endian {
                        u32::from_le_bytes([
                            buffer[pos],
                            buffer[pos + 1],
                            buffer[pos + 2],
                            buffer[pos + 3],
                        ])
                    } else {
                        u32::from_be_bytes([
                            buffer[pos],
                            buffer[pos + 1],
                            buffer[pos + 2],
                            buffer[pos + 3],
                        ])
                    }
                };

                let ifd0_offset = read_u32(tiff_start + 4);
                let ifd0_start = tiff_start + ifd0_offset as usize;

                if buffer.len() < ifd0_start + 2 {
                    return None;
                }

                let num_entries = read_u16(ifd0_start) as usize;

                for i in 0..num_entries {
                    let entry_offset = ifd0_start + 2 + i * 12;
                    if buffer.len() < entry_offset + 12 {
                        break;
                    }

                    let tag = read_u16(entry_offset);
                    if tag == 0x0112 {
                        let value = read_u16(entry_offset + 8) as u8;
                        if value >= 1 && value <= 8 {
                            return Some(value);
                        }
                    }
                }
            }
            return None;
        }

        if (0xE0..=0xEF).contains(&marker) {
            let segment_length =
                u16::from_be_bytes([buffer[offset + 2], buffer[offset + 3]]) as usize;
            offset += 2 + segment_length;
            continue;
        }

        if marker == 0xC0 || marker == 0xDA {
            break;
        }

        offset += 1;
    }

    None
}

pub fn get_image_metadata(buffer: &[u8]) -> Option<ImageMetadata> {
    if buffer.len() < 24 {
        return None;
    }

    if &buffer[0..2] == b"\xff\xd8" {
        let mut offset = 2;
        while offset < buffer.len() - 8 {
            if buffer[offset] != 0xFF {
                offset += 1;
                continue;
            }

            let marker = buffer[offset + 1];

            if marker == 0xFF {
                offset += 1;
                continue;
            }

            if marker == 0xC0 || marker == 0xC2 {
                let seg_start = offset + 4;
                if buffer.len() >= seg_start + 6 {
                    let height = u16::from_be_bytes([buffer[seg_start + 1], buffer[seg_start + 2]]);
                    let width = u16::from_be_bytes([buffer[seg_start + 3], buffer[seg_start + 4]]);
                    return Some(ImageMetadata {
                        width: width as u32,
                        height: height as u32,
                    });
                }
            }

            if (0xE0..=0xEF).contains(&marker) {
                let segment_length =
                    u16::from_be_bytes([buffer[offset + 2], buffer[offset + 3]]) as usize;
                offset += 2 + segment_length;
                continue;
            }

            offset += 1;
        }
    }

    if &buffer[0..8] == b"\x89PNG\r\n\x1a\n" {
        let chunk_start = 8;
        if buffer.len() >= chunk_start + 24 && &buffer[chunk_start + 4..chunk_start + 8] == b"IHDR"
        {
            let width = u32::from_be_bytes([
                buffer[chunk_start + 8],
                buffer[chunk_start + 9],
                buffer[chunk_start + 10],
                buffer[chunk_start + 11],
            ]);
            let height = u32::from_be_bytes([
                buffer[chunk_start + 12],
                buffer[chunk_start + 13],
                buffer[chunk_start + 14],
                buffer[chunk_start + 15],
            ]);
            return Some(ImageMetadata { width, height });
        }
    }

    if buffer.len() >= 12 && (&buffer[0..4] == b"GIF8") {
        let width = u16::from_le_bytes([buffer[6], buffer[7]]) as u32;
        let height = u16::from_le_bytes([buffer[8], buffer[9]]) as u32;
        return Some(ImageMetadata { width, height });
    }

    if buffer.len() >= 16 && &buffer[4..8] == b"ftyp" {
        let mut offset = 0;
        while offset < buffer.len() - 8 {
            let box_size = u32::from_be_bytes([
                buffer[offset],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
            ]) as usize;
            let box_type = &buffer[offset + 4..offset + 8];

            if box_type == b"avc1" || box_type == b"hev1" || box_type == b"hvc1" {
                let width_offset = offset + 32;
                let height_offset = offset + 34;
                if buffer.len() >= height_offset + 2 {
                    let width =
                        u16::from_be_bytes([buffer[width_offset], buffer[width_offset + 1]]) as u32;
                    let height =
                        u16::from_be_bytes([buffer[height_offset], buffer[height_offset + 1]])
                            as u32;
                    return Some(ImageMetadata { width, height });
                }
            }

            if box_size == 0 {
                break;
            }
            offset += box_size;
        }
    }

    None
}

pub fn normalize_exif_orientation(buffer: &[u8]) -> Vec<u8> {
    let orientation = read_jpeg_exif_orientation(buffer);

    match orientation {
        Some(1) | None => buffer.to_vec(),
        Some(_) => buffer.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_image_resize_side_grid() {
        let grid = build_image_resize_side_grid(2048, 2048);
        assert!(!grid.is_empty());
        assert_eq!(grid[0], 2048);
    }

    #[test]
    fn test_get_image_metadata_png() {
        let png_header = [
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x0A,
        ];
        let meta = get_image_metadata(&png_header);
        assert!(meta.is_some());
        let meta = meta.unwrap();
        assert_eq!(meta.width, 16);
        assert_eq!(meta.height, 10);
    }

    #[test]
    fn test_get_image_metadata_gif() {
        let gif_header = b"GIF89a\x10\x00\x0a\x00";
        let meta = get_image_metadata(gif_header);
        assert!(meta.is_some());
        let meta = meta.unwrap();
        assert_eq!(meta.width, 16);
        assert_eq!(meta.height, 10);
    }
}
