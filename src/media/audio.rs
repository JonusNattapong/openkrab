use once_cell::sync::Lazy;
use std::collections::HashSet;

pub static TELEGRAM_VOICE_AUDIO_EXTENSIONS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert(".oga");
    set.insert(".ogg");
    set.insert(".opus");
    set.insert(".mp3");
    set.insert(".m4a");
    set
});

pub static TELEGRAM_VOICE_MIME_TYPES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    set.insert("audio/ogg");
    set.insert("audio/opus");
    set.insert("audio/mpeg");
    set.insert("audio/mp3");
    set.insert("audio/mp4");
    set.insert("audio/x-m4a");
    set.insert("audio/m4a");
    set
});

pub fn is_telegram_voice_compatible_audio(
    content_type: Option<&str>,
    file_name: Option<&str>,
) -> bool {
    if let Some(mime) = crate::media::mime::normalize_mime(content_type) {
        if TELEGRAM_VOICE_MIME_TYPES.contains(mime) {
            return true;
        }
    }

    if let Some(name) = file_name.map(|s| s.trim()).filter(|s| !s.is_empty()) {
        if let Some(ext) = crate::media::mime::get_file_extension(Some(name)) {
            return TELEGRAM_VOICE_AUDIO_EXTENSIONS.contains(ext);
        }
    }

    false
}

pub fn is_voice_compatible_audio(content_type: Option<&str>, file_name: Option<&str>) -> bool {
    is_telegram_voice_compatible_audio(content_type, file_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telegram_voice_mime_types() {
        assert!(is_telegram_voice_compatible_audio(Some("audio/ogg"), None));
        assert!(is_telegram_voice_compatible_audio(Some("audio/mpeg"), None));
        assert!(is_telegram_voice_compatible_audio(None, Some("audio.mp3")));
        assert!(is_telegram_voice_compatible_audio(None, Some("voice.oga")));
    }

    #[test]
    fn test_non_voice_types() {
        assert!(!is_telegram_voice_compatible_audio(Some("audio/wav"), None));
        assert!(!is_telegram_voice_compatible_audio(None, Some("audio.wav")));
    }
}
