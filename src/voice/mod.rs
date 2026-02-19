use anyhow::{anyhow, bail, Result};
use base64::Engine;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AudioStats {
    pub sample_rate: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
    pub duration_ms: u64,
    pub rms_dbfs: f32,
    pub peak_dbfs: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VoiceAction {
    Ignore,
    Wake,
    Talk,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VoiceDecision {
    pub action: VoiceAction,
    pub reason: String,
}

pub fn analyze_wav_pcm16(bytes: &[u8]) -> Result<AudioStats> {
    if bytes.len() < 44 {
        bail!("wav payload too small");
    }
    if &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        bail!("unsupported audio format: expected RIFF/WAVE");
    }

    let mut idx = 12usize;
    let mut channels = 0u16;
    let mut sample_rate = 0u32;
    let mut bits_per_sample = 0u16;
    let mut data_start = 0usize;
    let mut data_len = 0usize;

    while idx + 8 <= bytes.len() {
        let chunk_id = &bytes[idx..idx + 4];
        let chunk_size = u32::from_le_bytes([
            bytes[idx + 4],
            bytes[idx + 5],
            bytes[idx + 6],
            bytes[idx + 7],
        ]) as usize;
        let content_start = idx + 8;
        let content_end = content_start.saturating_add(chunk_size);
        if content_end > bytes.len() {
            break;
        }

        if chunk_id == b"fmt " {
            if chunk_size < 16 {
                bail!("invalid fmt chunk");
            }
            let audio_format = u16::from_le_bytes([bytes[content_start], bytes[content_start + 1]]);
            if audio_format != 1 {
                bail!("only PCM WAV is supported");
            }
            channels = u16::from_le_bytes([bytes[content_start + 2], bytes[content_start + 3]]);
            sample_rate = u32::from_le_bytes([
                bytes[content_start + 4],
                bytes[content_start + 5],
                bytes[content_start + 6],
                bytes[content_start + 7],
            ]);
            bits_per_sample =
                u16::from_le_bytes([bytes[content_start + 14], bytes[content_start + 15]]);
        } else if chunk_id == b"data" {
            data_start = content_start;
            data_len = chunk_size;
            break;
        }

        idx = content_end + (chunk_size % 2);
    }

    if channels == 0 || sample_rate == 0 || bits_per_sample == 0 || data_len == 0 {
        bail!("missing WAV fmt/data chunks");
    }
    if bits_per_sample != 16 {
        bail!("only 16-bit PCM WAV is supported");
    }
    if data_start + data_len > bytes.len() {
        bail!("invalid WAV data chunk bounds");
    }

    let data = &bytes[data_start..data_start + data_len];
    let sample_count = data.len() / 2;
    if sample_count == 0 {
        bail!("empty audio data");
    }

    let mut sum_squares = 0f64;
    let mut peak = 0f64;
    for i in 0..sample_count {
        let off = i * 2;
        let s = i16::from_le_bytes([data[off], data[off + 1]]) as f64 / i16::MAX as f64;
        let abs = s.abs();
        if abs > peak {
            peak = abs;
        }
        sum_squares += s * s;
    }

    let rms = (sum_squares / sample_count as f64).sqrt();
    let rms_dbfs = if rms > 0.0 {
        (20.0 * rms.log10()) as f32
    } else {
        -120.0
    };
    let peak_dbfs = if peak > 0.0 {
        (20.0 * peak.log10()) as f32
    } else {
        -120.0
    };

    let frames = sample_count as u64 / channels as u64;
    let duration_ms = if sample_rate == 0 {
        0
    } else {
        ((frames as f64 / sample_rate as f64) * 1000.0) as u64
    };

    Ok(AudioStats {
        sample_rate,
        channels,
        bits_per_sample,
        duration_ms,
        rms_dbfs,
        peak_dbfs,
    })
}

pub fn detect_wake_or_talk(transcript: &str, wake_phrase: &str, is_awake: bool) -> VoiceDecision {
    let normalized = normalize_text(transcript);
    let wake = normalize_text(wake_phrase);

    if normalized.is_empty() {
        return VoiceDecision {
            action: VoiceAction::Ignore,
            reason: "empty transcript".to_string(),
        };
    }

    if !wake.is_empty() && normalized.contains(&wake) {
        return VoiceDecision {
            action: VoiceAction::Wake,
            reason: "wake phrase detected".to_string(),
        };
    }

    if is_awake {
        return VoiceDecision {
            action: VoiceAction::Talk,
            reason: "session already awake".to_string(),
        };
    }

    VoiceDecision {
        action: VoiceAction::Ignore,
        reason: "wake phrase not detected".to_string(),
    }
}

fn normalize_text(text: &str) -> String {
    text.to_ascii_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c.is_ascii_whitespace() {
                c
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn decode_audio_payload(payload: &serde_json::Value) -> Result<Vec<u8>> {
    if let Some(path) = payload.get("wav_path").and_then(|v| v.as_str()) {
        let bytes = std::fs::read(path).map_err(|e| anyhow!("failed reading wav_path: {e}"))?;
        return Ok(bytes);
    }
    if let Some(b64) = payload.get("wav_base64").and_then(|v| v.as_str()) {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(b64)
            .map_err(|e| anyhow!("invalid wav_base64: {e}"))?;
        return Ok(bytes);
    }
    bail!("voice analyze_audio requires payload.wav_path or payload.wav_base64")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_wav_silence() -> Vec<u8> {
        let sample_rate = 16000u32;
        let channels = 1u16;
        let bits = 16u16;
        let samples = 160u32;
        let data_size = (samples * 2) as u32;

        let mut out = Vec::new();
        out.extend_from_slice(b"RIFF");
        out.extend_from_slice(&(36 + data_size).to_le_bytes());
        out.extend_from_slice(b"WAVE");
        out.extend_from_slice(b"fmt ");
        out.extend_from_slice(&16u32.to_le_bytes());
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&channels.to_le_bytes());
        out.extend_from_slice(&sample_rate.to_le_bytes());
        let byte_rate = sample_rate * channels as u32 * (bits as u32 / 8);
        out.extend_from_slice(&byte_rate.to_le_bytes());
        let block_align = channels * (bits / 8);
        out.extend_from_slice(&block_align.to_le_bytes());
        out.extend_from_slice(&bits.to_le_bytes());
        out.extend_from_slice(b"data");
        out.extend_from_slice(&data_size.to_le_bytes());
        out.resize(out.len() + data_size as usize, 0);
        out
    }

    #[test]
    fn wav_analysis_works() {
        let bytes = test_wav_silence();
        let stats = analyze_wav_pcm16(&bytes).unwrap();
        assert_eq!(stats.sample_rate, 16000);
        assert_eq!(stats.channels, 1);
        assert_eq!(stats.bits_per_sample, 16);
        assert!(stats.duration_ms > 0);
    }

    #[test]
    fn wake_detection_works() {
        let d = detect_wake_or_talk("hey krabkrab what time is it", "hey krabkrab", false);
        assert_eq!(d.action, VoiceAction::Wake);

        let d2 = detect_wake_or_talk("tell me weather", "hey krabkrab", true);
        assert_eq!(d2.action, VoiceAction::Talk);
    }
}
