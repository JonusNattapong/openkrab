//! tts — Text-to-Speech abstraction.
//! Ported from `openclaw/src/tts/` (Phase 5).
//!
//! Provides platform-aware TTS so the agent can speak replies on the host OS.
//! On Windows: PowerShell `Add-Type`/`SpeakAsync`.
//! On macOS: `say` command.
//! On Linux: `espeak-ng` or `piper`.

use anyhow::anyhow;
use anyhow::Result;
use std::process::Command;

// ─── Backend enum ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TtsBackend {
    /// ElevenLabs cloud TTS API.
    ElevenLabs,
    /// Windows SAPI via PowerShell.
    WindowsSapi,
    /// macOS built-in `say` command.
    MacosSay,
    /// `espeak-ng` on Linux.
    Espeak,
    /// No-op / silent.
    Silent,
}

impl TtsBackend {
    /// Detect the best available backend for the current platform.
    pub fn detect() -> Self {
        if std::env::var("ELEVENLABS_API_KEY")
            .ok()
            .map(|v| !v.trim().is_empty())
            .unwrap_or(false)
        {
            return TtsBackend::ElevenLabs;
        }

        #[cfg(target_os = "windows")]
        return TtsBackend::WindowsSapi;

        #[cfg(target_os = "macos")]
        return TtsBackend::MacosSay;

        #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
        {
            if which_exists("espeak-ng") {
                TtsBackend::Espeak
            } else {
                TtsBackend::Silent
            }
        }
    }
}

fn which_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

// ─── TTS speaker ──────────────────────────────────────────────────────────────

/// High-level TTS interface.
pub struct TtsSpeaker {
    pub backend: TtsBackend,
    pub rate: Option<i32>,
    pub voice: Option<String>,
}

impl TtsSpeaker {
    pub fn new() -> Self {
        Self {
            backend: TtsBackend::detect(),
            rate: None,
            voice: None,
        }
    }

    pub fn with_backend(mut self, backend: TtsBackend) -> Self {
        self.backend = backend;
        self
    }

    pub fn with_rate(mut self, rate: i32) -> Self {
        self.rate = Some(rate);
        self
    }

    pub fn with_voice(mut self, voice: impl Into<String>) -> Self {
        self.voice = Some(voice.into());
        self
    }

    /// Speak the given text using the detected/configured backend.
    pub fn speak(&self, text: &str) -> Result<()> {
        let safe = sanitize_text(text);
        match &self.backend {
            TtsBackend::ElevenLabs => self.speak_elevenlabs(&safe),
            TtsBackend::WindowsSapi => self.speak_windows(&safe),
            TtsBackend::MacosSay => self.speak_macos(&safe),
            TtsBackend::Espeak => self.speak_espeak(&safe),
            TtsBackend::Silent => Ok(()),
        }
    }

    fn speak_windows(&self, text: &str) -> Result<()> {
        let rate = self.rate.unwrap_or(0);
        let script = format!(
            "Add-Type -AssemblyName System.Speech; \
             $s = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
             $s.Rate = {}; \
             $s.Speak([System.Runtime.InteropServices.RuntimeEnvironment]::GetRuntimeDirectory() | Out-Null; '{}');",
            rate,
            text.replace('\'', "''")
        );
        // Simplified invocation (avoids the complex escape above)
        let ps_cmd = format!(
            "Add-Type -AssemblyName System.Speech; $s=New-Object System.Speech.Synthesis.SpeechSynthesizer; $s.Rate={}; $s.Speak('{}')",
            rate,
            text.replace('\'', " ")
        );
        Command::new("powershell")
            .args(["-NoProfile", "-Command", &ps_cmd])
            .status()?;
        Ok(())
    }

    fn speak_macos(&self, text: &str) -> Result<()> {
        let mut cmd = Command::new("say");
        if let Some(ref voice) = self.voice {
            cmd.args(["-v", voice]);
        }
        if let Some(rate) = self.rate {
            cmd.args(["-r", &rate.to_string()]);
        }
        cmd.arg(text).status()?;
        Ok(())
    }

    fn speak_espeak(&self, text: &str) -> Result<()> {
        let mut cmd = Command::new("espeak-ng");
        if let Some(rate) = self.rate {
            cmd.args(["-s", &rate.to_string()]);
        }
        cmd.arg(text).status()?;
        Ok(())
    }

    fn speak_elevenlabs(&self, text: &str) -> Result<()> {
        let api_key = std::env::var("ELEVENLABS_API_KEY")
            .map_err(|_| anyhow!("ELEVENLABS_API_KEY is not set"))?;
        let voice_id = std::env::var("ELEVENLABS_VOICE_ID")
            .ok()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or_else(|| "EXAVITQu4vr4xnSDxMaL".to_string());
        let model_id = std::env::var("ELEVENLABS_MODEL_ID")
            .ok()
            .filter(|v| !v.trim().is_empty())
            .unwrap_or_else(|| "eleven_multilingual_v2".to_string());

        let url = format!(
            "https://api.elevenlabs.io/v1/text-to-speech/{voice_id}?output_format=mp3_44100_128"
        );

        let body = serde_json::json!({
            "text": text,
            "model_id": model_id,
        });

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(url)
            .header("xi-api-key", api_key)
            .header("accept", "audio/mpeg")
            .json(&body)
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(anyhow!("elevenlabs request failed ({}): {}", status, body));
        }

        let bytes = response.bytes()?;
        let mut path = std::env::temp_dir();
        path.push(format!("krabkrab-tts-{}.mp3", uuid::Uuid::new_v4()));
        std::fs::write(&path, &bytes)?;

        play_audio_file(&path)?;
        let _ = std::fs::remove_file(path);
        Ok(())
    }
}

fn play_audio_file(path: &std::path::Path) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        Command::new("afplay").arg(path).status()?;
        return Ok(());
    }

    #[cfg(target_os = "windows")]
    {
        let ffplay = Command::new("ffplay")
            .args(["-nodisp", "-autoexit", "-loglevel", "quiet"])
            .arg(path)
            .status();
        if ffplay.is_ok() {
            return Ok(());
        }

        let script = format!(
            "$w = New-Object -ComObject WMPlayer.OCX; \
             $m = $w.newMedia('{}'); \
             $w.currentPlaylist.appendItem($m) | Out-Null; \
             $w.controls.play(); \
             while ($w.playState -ne 1) {{ Start-Sleep -Milliseconds 200 }}",
            path.display()
        );
        Command::new("powershell")
            .args(["-NoProfile", "-Command", &script])
            .status()?;
        return Ok(());
    }

    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        let status = Command::new("ffplay")
            .args(["-nodisp", "-autoexit", "-loglevel", "quiet"])
            .arg(path)
            .status();
        if status.is_ok() {
            return Ok(());
        }
        let status = Command::new("mpg123").arg(path).status();
        if status.is_ok() {
            return Ok(());
        }
        Err(anyhow!("no audio player found for ElevenLabs mp3 output"))
    }
}

impl Default for TtsSpeaker {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Text sanitiser ───────────────────────────────────────────────────────────

/// Remove characters that could break shell invocations.
pub fn sanitize_text(input: &str) -> String {
    // Strip markdown formatting and dangerous shell chars
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '`' | '"' | '$' | '\0' => out.push(' '),
            _ => out.push(ch),
        }
    }
    // collapse excessive whitespace
    let mut prev_space = false;
    let mut clean = String::with_capacity(out.len());
    for ch in out.chars() {
        if ch.is_whitespace() {
            if !prev_space {
                clean.push(' ');
            }
            prev_space = true;
        } else {
            clean.push(ch);
            prev_space = false;
        }
    }
    clean.trim().to_string()
}

/// Best-effort estimate of speaking duration for a given text (seconds).
/// Assumes ~150 words per minute average.
pub fn estimate_duration_secs(text: &str, wpm: Option<f32>) -> f32 {
    let words = text.split_whitespace().count() as f32;
    let rate = wpm.unwrap_or(150.0).max(1.0);
    words / rate * 60.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_removes_backticks() {
        // Backticks become spaces; leading/trailing spaces are trimmed.
        assert_eq!(sanitize_text("`hello`"), "hello");
        // Dollar signs become spaces; internal space is collapsed.
        assert_eq!(sanitize_text("hello $world"), "hello world");
        // Double-quotes become spaces.
        assert_eq!(sanitize_text("say \"hi\""), "say hi");
    }

    #[test]
    fn estimate_duration() {
        // 150 words at 150 wpm → ~60 seconds
        let text = "word ".repeat(150);
        let dur = estimate_duration_secs(text.trim(), None);
        assert!((dur - 60.0).abs() < 1.0, "dur={}", dur);
    }

    #[test]
    fn detect_backend_is_not_panicky() {
        let _b = TtsBackend::detect();
    }

    #[test]
    fn speaker_builder() {
        let s = TtsSpeaker::new().with_rate(120).with_voice("Samantha");
        assert_eq!(s.rate, Some(120));
        assert_eq!(s.voice.as_deref(), Some("Samantha"));
    }
}
