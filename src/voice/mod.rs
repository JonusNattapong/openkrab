use anyhow::{anyhow, bail, Result};
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// =============================================================================
// Audio Statistics
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AudioStats {
    pub sample_rate: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
    pub duration_ms: u64,
    pub rms_dbfs: f32,
    pub peak_dbfs: f32,
    pub zero_crossing_rate: f32,
    pub spectral_centroid: f32,
}

impl AudioStats {
    pub fn from_samples(samples: &[i16], sample_rate: u32) -> Self {
        let duration_ms = (samples.len() as u64 * 1000) / sample_rate as u64;
        let rms = calculate_rms(samples);
        let peak = calculate_peak(samples);
        let zcr = calculate_zero_crossing_rate(samples);
        let centroid = calculate_spectral_centroid(samples, sample_rate);

        Self {
            sample_rate,
            channels: 1,
            bits_per_sample: 16,
            duration_ms,
            rms_dbfs: rms,
            peak_dbfs: peak,
            zero_crossing_rate: zcr,
            spectral_centroid,
        }
    }
}

// =============================================================================
// Voice Actions & Decisions
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VoiceAction {
    Ignore,
    Wake,
    Talk,
    Sleep,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoiceDecision {
    pub action: VoiceAction,
    pub reason: String,
    pub confidence: f32,
}

// =============================================================================
// Voice Activity Detection (VAD)
// =============================================================================

pub struct VoiceActivityDetector {
    threshold_db: f32,
    min_speech_duration_ms: u64,
    min_silence_duration_ms: u64,
    speech_frames: usize,
    silence_frames: usize,
    is_speaking: bool,
}

impl VoiceActivityDetector {
    pub fn new() -> Self {
        Self {
            threshold_db: -40.0,
            min_speech_duration_ms: 100,
            min_silence_duration_ms: 300,
            speech_frames: 0,
            silence_frames: 0,
            is_speaking: false,
        }
    }

    pub fn with_threshold(mut self, db: f32) -> Self {
        self.threshold_db = db;
        self
    }

    pub fn with_min_speech_duration(mut self, ms: u64) -> Self {
        self.min_speech_duration_ms = ms;
        self
    }

    pub fn with_min_silence_duration(mut self, ms: u64) -> Self {
        self.min_silence_duration_ms = ms;
        self
    }

    pub fn process(&mut self, samples: &[i16], sample_rate: u32) -> VADState {
        let rms = calculate_rms(samples);
        let is_speech = rms > self.threshold_db;

        let frame_duration_ms = (samples.len() as u64 * 1000) / sample_rate as u64;

        if is_speech {
            self.speech_frames += 1;
            self.silence_frames = 0;

            if !self.is_speaking
                && (self.speech_frames as u64) * frame_duration_ms >= self.min_speech_duration_ms
            {
                self.is_speaking = true;
                return VADState::SpeechStart;
            }

            return VADState::Speaking;
        } else {
            self.silence_frames += 1;
            self.speech_frames = 0;

            if self.is_speaking
                && (self.silence_frames as u64) * frame_duration_ms >= self.min_silence_duration_ms
            {
                self.is_speaking = false;
                return VADState::SpeechEnd;
            }

            return VADState::Silence;
        }
    }

    pub fn is_speaking(&self) -> bool {
        self.is_speaking
    }

    pub fn reset(&mut self) {
        self.speech_frames = 0;
        self.silence_frames = 0;
        self.is_speaking = false;
    }
}

impl Default for VoiceActivityDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VADState {
    Silence,
    Speaking,
    SpeechStart,
    SpeechEnd,
}

// =============================================================================
// Spectral Analysis
// =============================================================================

pub struct SpectralAnalyzer {
    fft_size: usize,
    window: Vec<f32>,
}

impl SpectralAnalyzer {
    pub fn new(fft_size: usize) -> Self {
        let window = Self::hann_window(fft_size);
        Self { fft_size, window }
    }

    fn hann_window(size: usize) -> Vec<f32> {
        (0..size)
            .map(|i| {
                0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (size - 1) as f32).cos())
            })
            .collect()
    }

    pub fn analyze(&self, samples: &[i16]) -> SpectralFeatures {
        if samples.len() < self.fft_size {
            return SpectralFeatures::default();
        }

        let windowed: Vec<f32> = samples
            .iter()
            .take(self.fft_size)
            .zip(self.window.iter())
            .map(|(&s, &w)| (s as f32 / i16::MAX as f32) * w)
            .collect();

        let spectrum = self.simple_fft(&windowed);

        let spectral_flux = self.calculate_spectral_flux(&spectrum);
        let spectral_rolloff = self.calculate_spectral_rolloff(&spectrum);
        let spectral_flatness = self.calculate_spectral_flatness(&spectrum);

        SpectralFeatures {
            spectral_flux,
            spectral_rolloff,
            spectral_flatness,
            dominant_frequency: self.find_dominant_frequency(&spectrum),
        }
    }

    fn simple_fft(&self, samples: &[f32]) -> Vec<f32> {
        let n = samples.len();
        let mut spectrum = vec![0.0; n / 2];

        for k in 0..n / 2 {
            let mut real = 0.0f32;
            let mut imag = 0.0f32;

            for t in 0..n {
                let angle = -2.0 * std::f32::consts::PI * (t * k) as f32 / n as f32;
                let (sin, cos) = angle.sin_cos();
                real += samples[t] * cos;
                imag += samples[t] * sin;
            }

            spectrum[k] = (real * real + imag * imag).sqrt();
        }

        spectrum
    }

    fn calculate_spectral_flux(&self, spectrum: &[f32]) -> f32 {
        if spectrum.len() < 2 {
            return 0.0;
        }

        let mut flux = 0.0f32;
        for i in 1..spectrum.len() {
            let diff = spectrum[i].abs() - spectrum[i - 1].abs();
            if diff > 0.0 {
                flux += diff;
            }
        }
        flux
    }

    fn calculate_spectral_rolloff(&self, spectrum: &[f32]) -> f32 {
        let total: f32 = spectrum.iter().sum();
        if total == 0.0 {
            return 0.0;
        }

        let threshold = total * 0.85;
        let mut cumulative = 0.0f32;

        for (i, &mag) in spectrum.iter().enumerate() {
            cumulative += mag;
            if cumulative >= threshold {
                return i as f32 / spectrum.len() as f32;
            }
        }

        1.0
    }

    fn calculate_spectral_flatness(&self, spectrum: &[f32]) -> f32 {
        let n = spectrum.len();
        if n == 0 {
            return 0.0;
        }

        let geometric_mean = spectrum
            .iter()
            .filter(|&&x| x > 0.0)
            .map(|&x| x.max(1e-10).ln())
            .sum::<f32>()
            / n as f32;

        let arithmetic_mean = spectrum.iter().sum::<f32>() / n as f32;

        if arithmetic_mean <= 0.0 {
            return 0.0;
        }

        (geometric_mean.exp() / arithmetic_mean).min(1.0)
    }

    fn find_dominant_frequency(&self, spectrum: &[f32]) -> f32 {
        spectrum
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i as f32 * 16000.0 / spectrum.len() as f32)
            .unwrap_or(0.0)
    }
}

impl Default for SpectralAnalyzer {
    fn default() -> Self {
        Self::new(512)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpectralFeatures {
    pub spectral_flux: f32,
    pub spectral_rolloff: f32,
    pub spectral_flatness: f32,
    pub dominant_frequency: f32,
}

// =============================================================================
// Audio Utility Functions
// =============================================================================

fn calculate_peak(samples: &[i16]) -> f32 {
    if samples.is_empty() {
        return -120.0;
    }

    let max_sample = samples.iter().map(|s| s.abs()).max().unwrap_or(0) as f32;

    if max_sample == 0.0 {
        return -120.0;
    }

    20.0 * (max_sample / i16::MAX as f32).log10()
}

fn calculate_zero_crossing_rate(samples: &[i16]) -> f32 {
    if samples.len() < 2 {
        return 0.0;
    }

    let mut crossings = 0usize;
    for i in 1..samples.len() {
        if (samples[i] >= 0) != (samples[i - 1] >= 0) {
            crossings += 1;
        }
    }

    crossings as f32 / samples.len() as f32
}

fn calculate_spectral_centroid(samples: &[i16], sample_rate: u32) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }

    let n = samples.len();
    let bin_width = sample_rate as f32 / n as f32;

    let mut weighted_sum = 0.0f32;
    let mut magnitude_sum = 0.0f32;

    for (i, &sample) in samples.iter().enumerate() {
        let mag = (sample as f32).abs();
        let freq = i as f32 * bin_width;
        weighted_sum += freq * mag;
        magnitude_sum += mag;
    }

    if magnitude_sum == 0.0 {
        return 0.0;
    }

    weighted_sum / magnitude_sum
}

// =============================================================================
// Voice Modes Configuration
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoiceWakeConfig {
    pub enabled: bool,
    pub wake_phrase: String,
    pub alternative_wake_phrases: Vec<String>,
    pub sensitivity: WakeSensitivity,
    pub audio_threshold_db: f32,
    pub min_wake_interval_ms: u64,
    pub vad_enabled: bool,
    pub vad_threshold_db: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TalkModeConfig {
    pub enabled: bool,
    pub timeout_seconds: u64,
    pub auto_sleep_after_inactivity: bool,
    pub beep_on_wake: bool,
    pub beep_on_sleep: bool,
    pub tts_enabled: bool,
    pub vad_enabled: bool,
    pub speech_timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoiceModesConfig {
    pub wake: VoiceWakeConfig,
    pub talk: TalkModeConfig,
    pub audio_device: Option<String>,
    pub sample_rate: u32,
    pub spectral_analysis: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum WakeSensitivity {
    Low,
    #[default]
    Medium,
    High,
}

impl Default for VoiceWakeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            wake_phrase: "hey krabkrab".to_string(),
            alternative_wake_phrases: vec![
                "krabkrab".to_string(),
                "hey crab".to_string(),
                "okay krab".to_string(),
            ],
            sensitivity: WakeSensitivity::Medium,
            audio_threshold_db: -40.0,
            min_wake_interval_ms: 2000,
            vad_enabled: true,
            vad_threshold_db: -40.0,
        }
    }
}

impl Default for TalkModeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout_seconds: 60,
            auto_sleep_after_inactivity: true,
            beep_on_wake: true,
            beep_on_sleep: true,
            tts_enabled: true,
            vad_enabled: true,
            speech_timeout_seconds: 10,
        }
    }
}

impl Default for VoiceModesConfig {
    fn default() -> Self {
        Self {
            wake: VoiceWakeConfig::default(),
            talk: TalkModeConfig::default(),
            audio_device: None,
            sample_rate: 16000,
            spectral_analysis: true,
        }
    }
}

// =============================================================================
// Voice Session State Management
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceSessionState {
    Idle,
    Listening,
    Processing,
    Responding,
}

pub struct VoiceSession {
    state: Arc<Mutex<VoiceSessionState>>,
    last_wake_time: Arc<Mutex<Option<Instant>>>,
    last_activity: Arc<Mutex<Option<Instant>>>,
    config: VoiceModesConfig,
    transcript_buffer: Arc<Mutex<VecDeque<String>>>,
    max_buffer_size: usize,
}

impl VoiceSession {
    pub fn new(config: VoiceModesConfig) -> Self {
        Self {
            state: Arc::new(Mutex::new(VoiceSessionState::Idle)),
            last_wake_time: Arc::new(Mutex::new(None)),
            last_activity: Arc::new(Mutex::new(None)),
            config,
            transcript_buffer: Arc::new(Mutex::new(VecDeque::new())),
            max_buffer_size: 10,
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(VoiceModesConfig::default())
    }

    pub fn get_state(&self) -> VoiceSessionState {
        *self.state.lock().unwrap()
    }

    pub fn set_state(&self, state: VoiceSessionState) {
        *self.state.lock().unwrap() = state;
        if state == VoiceSessionState::Listening {
            *self.last_activity.lock().unwrap() = Some(Instant::now());
        }
    }

    pub fn wake(&self) -> bool {
        let mut last_wake = self.last_wake_time.lock().unwrap();
        let now = Instant::now();

        if let Some(last) = *last_wake {
            let elapsed = now.duration_since(last).as_millis() as u64;
            if elapsed < self.config.wake.min_wake_interval_ms {
                return false;
            }
        }

        *last_wake = Some(now);
        *self.last_activity.lock().unwrap() = Some(now);
        *self.state.lock().unwrap() = VoiceSessionState::Listening;
        true
    }

    pub fn sleep(&self) {
        *self.state.lock().unwrap() = VoiceSessionState::Idle;
        *self.last_activity.lock().unwrap() = None;
    }

    pub fn is_awake(&self) -> bool {
        matches!(
            *self.state.lock().unwrap(),
            VoiceSessionState::Listening | VoiceSessionState::Processing
        )
    }

    pub fn check_timeout(&self) -> bool {
        if !self.config.talk.auto_sleep_after_inactivity {
            return false;
        }

        let last_activity = *self.last_activity.lock().unwrap();
        if let Some(last) = last_activity {
            let timeout = Duration::from_secs(self.config.talk.timeout_seconds);
            if Instant::now().duration_since(last) > timeout {
                self.sleep();
                return true;
            }
        }
        false
    }

    pub fn update_activity(&self) {
        *self.last_activity.lock().unwrap() = Some(Instant::now());
    }

    pub fn add_to_buffer(&self, transcript: String) {
        let mut buffer = self.transcript_buffer.lock().unwrap();
        buffer.push_back(transcript);
        while buffer.len() > self.max_buffer_size {
            buffer.pop_front();
        }
    }

    pub fn get_buffer(&self) -> Vec<String> {
        self.transcript_buffer
            .lock()
            .unwrap()
            .iter()
            .cloned()
            .collect()
    }

    pub fn clear_buffer(&self) {
        self.transcript_buffer.lock().unwrap().clear();
    }

    pub fn get_config(&self) -> &VoiceModesConfig {
        &self.config
    }

    pub fn time_since_wake(&self) -> Option<Duration> {
        self.last_wake_time
            .lock()
            .unwrap()
            .map(|t| Instant::now().duration_since(t))
    }

    pub fn time_since_activity(&self) -> Option<Duration> {
        self.last_activity
            .lock()
            .unwrap()
            .map(|t| Instant::now().duration_since(t))
    }
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

    let samples: Vec<i16> = data
        .chunks_exact(2)
        .map(|c| i16::from_le_bytes([c[0], c[1]]))
        .collect();
    let zero_crossing_rate = calculate_zero_crossing_rate(&samples);
    let spectral_centroid = calculate_spectral_centroid(&samples, sample_rate);

    Ok(AudioStats {
        sample_rate,
        channels,
        bits_per_sample,
        duration_ms,
        rms_dbfs,
        peak_dbfs,
        zero_crossing_rate,
        spectral_centroid,
    })
}

// =============================================================================
// Enhanced Wake/Talk Detection
// =============================================================================

pub fn detect_wake_or_talk(transcript: &str, wake_phrase: &str, is_awake: bool) -> VoiceDecision {
    detect_wake_or_talk_with_config(transcript, wake_phrase, is_awake, None)
}

pub fn detect_wake_or_talk_with_config(
    transcript: &str,
    wake_phrase: &str,
    is_awake: bool,
    config: Option<&VoiceWakeConfig>,
) -> VoiceDecision {
    let normalized = normalize_text(transcript);
    let wake = normalize_text(wake_phrase);

    if normalized.is_empty() {
        return VoiceDecision {
            action: VoiceAction::Ignore,
            reason: "empty transcript".to_string(),
            confidence: 0.0,
        };
    }

    // Check for sleep commands when awake
    if is_awake {
        let sleep_phrases = ["go to sleep", "sleep now", "goodbye", "bye krabkrab"];
        for phrase in &sleep_phrases {
            if normalized.contains(phrase) {
                return VoiceDecision {
                    action: VoiceAction::Sleep,
                    reason: "sleep command detected".to_string(),
                    confidence: 0.95,
                };
            }
        }
    }

    // Check primary wake phrase
    let (wake_detected, confidence) = if !wake.is_empty() && normalized.contains(&wake) {
        (true, 1.0)
    } else if let Some(cfg) = config {
        // Check alternative wake phrases
        let mut found = false;
        let mut max_conf: f32 = 0.0;
        for alt in &cfg.alternative_wake_phrases {
            let alt_norm = normalize_text(alt);
            if !alt_norm.is_empty() && normalized.contains(&alt_norm) {
                found = true;
                max_conf = max_conf.max(0.9f32);
            }
        }
        (found, max_conf)
    } else {
        (false, 0.0)
    };

    if wake_detected {
        return VoiceDecision {
            action: VoiceAction::Wake,
            reason: "wake phrase detected".to_string(),
            confidence,
        };
    }

    // Fuzzy matching for similar phrases (if sensitivity is high)
    if let Some(cfg) = config {
        if cfg.sensitivity == WakeSensitivity::High {
            let similarity = calculate_similarity(&normalized, &wake);
            if similarity > 0.8 {
                return VoiceDecision {
                    action: VoiceAction::Wake,
                    reason: "fuzzy wake phrase match".to_string(),
                    confidence: similarity,
                };
            }
        }
    }

    if is_awake {
        return VoiceDecision {
            action: VoiceAction::Talk,
            reason: "session already awake".to_string(),
            confidence: 1.0,
        };
    }

    VoiceDecision {
        action: VoiceAction::Ignore,
        reason: "wake phrase not detected".to_string(),
        confidence: 0.0,
    }
}

fn calculate_similarity(a: &str, b: &str) -> f32 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }

    let a_words: std::collections::HashSet<_> = a.split_whitespace().collect();
    let b_words: std::collections::HashSet<_> = b.split_whitespace().collect();

    if a_words.is_empty() || b_words.is_empty() {
        return 0.0;
    }

    let intersection: std::collections::HashSet<_> = a_words.intersection(&b_words).collect();
    let union: std::collections::HashSet<_> = a_words.union(&b_words).collect();

    intersection.len() as f32 / union.len() as f32
}

// =============================================================================
// Text Normalization
// =============================================================================

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

// =============================================================================
// Audio Preprocessing
// =============================================================================

pub struct AudioPreprocessor {
    pub noise_gate_db: f32,
    pub normalization_target_db: f32,
    pub high_pass_freq: f32,
    pub low_pass_freq: f32,
}

impl Default for AudioPreprocessor {
    fn default() -> Self {
        Self {
            noise_gate_db: -50.0,
            normalization_target_db: -20.0,
            high_pass_freq: 80.0,
            low_pass_freq: 8000.0,
        }
    }
}

impl AudioPreprocessor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_noise_gate(mut self, db: f32) -> Self {
        self.noise_gate_db = db;
        self
    }

    /// Apply noise gate to audio samples
    pub fn apply_noise_gate(&self, samples: &mut [i16]) {
        let threshold_linear = db_to_linear(self.noise_gate_db);
        let threshold_i16 = (threshold_linear * i16::MAX as f32) as i16;

        for sample in samples.iter_mut() {
            if sample.abs() < threshold_i16 {
                *sample = 0;
            }
        }
    }

    /// Normalize audio to target dB level
    pub fn normalize(&self, samples: &mut [i16]) {
        if samples.is_empty() {
            return;
        }

        let max_sample = samples.iter().map(|s| s.abs()).max().unwrap_or(0) as f32;
        if max_sample == 0.0 {
            return;
        }

        let current_db = linear_to_db(max_sample / i16::MAX as f32);
        let gain_db = self.normalization_target_db - current_db;
        let gain = db_to_linear(gain_db);

        for sample in samples.iter_mut() {
            let new_value = (*sample as f32 * gain) as i32;
            *sample = new_value.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        }
    }

    /// Simple high-pass filter (remove low frequency noise)
    pub fn high_pass_filter(&self, samples: &mut [i16], sample_rate: u32) {
        if samples.len() < 2 {
            return;
        }

        let rc = 1.0 / (2.0 * std::f32::consts::PI * self.high_pass_freq);
        let dt = 1.0 / sample_rate as f32;
        let alpha = rc / (rc + dt);

        let mut prev_input = samples[0] as f32;
        let mut prev_output = samples[0] as f32;

        for sample in samples.iter_mut() {
            let input = *sample as f32;
            let output = alpha * (prev_output + input - prev_input);
            *sample = output as i16;
            prev_input = input;
            prev_output = output;
        }
    }

    /// Process audio with all preprocessing steps
    pub fn process(&self, samples: &mut [i16], sample_rate: u32) {
        self.high_pass_filter(samples, sample_rate);
        self.apply_noise_gate(samples);
        self.normalize(samples);
    }
}

fn db_to_linear(db: f32) -> f32 {
    10f32.powf(db / 20.0)
}

fn linear_to_db(linear: f32) -> f32 {
    if linear <= 0.0 {
        -120.0
    } else {
        20.0 * linear.log10()
    }
}

// =============================================================================
// Audio Payload Decoding
// =============================================================================

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

// =============================================================================
// Voice Mode Controller
// =============================================================================

pub struct VoiceModeController {
    session: VoiceSession,
    preprocessor: AudioPreprocessor,
}

impl VoiceModeController {
    pub fn new(config: VoiceModesConfig) -> Self {
        let session = VoiceSession::new(config);
        let preprocessor = AudioPreprocessor::default();
        Self {
            session,
            preprocessor,
        }
    }

    pub fn with_preprocessor(mut self, preprocessor: AudioPreprocessor) -> Self {
        self.preprocessor = preprocessor;
        self
    }

    /// Process audio and detect wake/talk
    pub fn process_audio(&self, transcript: &str) -> VoiceDecision {
        let config = self.session.get_config();
        let is_awake = self.session.is_awake();

        let decision = detect_wake_or_talk_with_config(
            transcript,
            &config.wake.wake_phrase,
            is_awake,
            Some(&config.wake),
        );

        match decision.action {
            VoiceAction::Wake => {
                if self.session.wake() {
                    self.session.add_to_buffer(transcript.to_string());
                }
            }
            VoiceAction::Talk => {
                self.session.update_activity();
                self.session.add_to_buffer(transcript.to_string());
            }
            VoiceAction::Sleep => {
                self.session.sleep();
            }
            VoiceAction::Ignore => {}
        }

        decision
    }

    /// Check if session has timed out
    pub fn check_timeout(&self) -> bool {
        self.session.check_timeout()
    }

    /// Get current session state
    pub fn get_state(&self) -> VoiceSessionState {
        self.session.get_state()
    }

    /// Force wake the session
    pub fn wake(&self) -> bool {
        self.session.wake()
    }

    /// Force sleep the session
    pub fn sleep(&self) {
        self.session.sleep();
    }

    /// Get transcript buffer
    pub fn get_transcript_buffer(&self) -> Vec<String> {
        self.session.get_buffer()
    }

    /// Clear transcript buffer
    pub fn clear_buffer(&self) {
        self.session.clear_buffer();
    }

    /// Get session info
    pub fn get_session_info(&self) -> VoiceSessionInfo {
        VoiceSessionInfo {
            state: self.session.get_state(),
            is_awake: self.session.is_awake(),
            time_since_wake_secs: self.session.time_since_wake().map(|d| d.as_secs()),
            time_since_activity_secs: self.session.time_since_activity().map(|d| d.as_secs()),
            buffer_size: self.session.get_buffer().len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSessionInfo {
    pub state: VoiceSessionState,
    pub is_awake: bool,
    pub time_since_wake_secs: Option<u64>,
    pub time_since_activity_secs: Option<u64>,
    pub buffer_size: usize,
}

// =============================================================================
// Real-time Microphone Capture
// =============================================================================

pub mod microphone {
    use super::*;
    use std::collections::VecDeque;
    use std::io::Read;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::sync::Mutex;

    pub struct MicrophoneCapture {
        is_recording: Arc<AtomicBool>,
        sample_rate: u32,
        buffer_size: usize,
        audio_buffer: Arc<Mutex<VecDeque<Vec<i16>>>>,
        frames_captured: Arc<AtomicUsize>,
        device_id: Option<String>,
    }

    impl MicrophoneCapture {
        pub fn new(sample_rate: u32, buffer_size: usize) -> Self {
            Self {
                is_recording: Arc::new(AtomicBool::new(false)),
                sample_rate,
                buffer_size,
                audio_buffer: Arc::new(Mutex::new(VecDeque::new())),
                frames_captured: Arc::new(AtomicUsize::new(0)),
                device_id: None,
            }
        }

        pub fn with_device(mut self, device_id: impl Into<String>) -> Self {
            self.device_id = Some(device_id.into());
            self
        }

        pub fn is_recording(&self) -> bool {
            self.is_recording.load(Ordering::SeqCst)
        }

        pub fn start(&self) -> Result<()> {
            if self.is_recording.load(Ordering::SeqCst) {
                return Ok(());
            }
            self.is_recording.store(true, Ordering::SeqCst);
            self.frames_captured.store(0, Ordering::SeqCst);
            self.audio_buffer.lock().unwrap().clear();
            self.start_platform_capture()?;
            Ok(())
        }

        pub fn stop(&self) {
            self.is_recording.store(false, Ordering::SeqCst);
        }

        pub fn get_config(&self) -> MicrophoneConfig {
            MicrophoneConfig {
                sample_rate: self.sample_rate,
                buffer_size: self.buffer_size,
                channels: 1,
                format: "pcm_s16le".to_string(),
                device_id: self.device_id.clone(),
            }
        }

        pub fn get_audio_buffer(&self) -> Vec<i16> {
            let mut buffer = self.audio_buffer.lock().unwrap();
            let mut result = Vec::new();
            while let Some(frame) = buffer.pop_front() {
                result.extend(frame);
            }
            result
        }

        pub fn get_frame_count(&self) -> usize {
            self.frames_captured.load(Ordering::SeqCst)
        }

        pub fn read_frame(&self) -> Option<Vec<i16>> {
            self.audio_buffer.lock().unwrap().pop_front()
        }

        #[cfg(target_os = "macos")]
        fn start_platform_capture(&self) -> Result<()> {
            use std::io::BufReader;
            use std::process::{Command, Stdio};

            let sample_rate = self.sample_rate;
            let audio_buffer = self.audio_buffer.clone();
            let is_recording = self.is_recording.clone();
            let frames_captured = self.frames_captured.clone();
            let buffer_size = self.buffer_size;

            std::thread::spawn(move || {
                let mut cmd = Command::new("sox");
                let device_arg = if let Some(ref dev) = self.device_id.clone() {
                    vec!["-d", "-t", "coreaudio", dev]
                } else {
                    vec!["-d"]
                };

                cmd.args(&device_arg)
                    .args(["-r", &sample_rate.to_string()])
                    .args(["-c", "1", "-e", "signed-integer", "-b", "16"])
                    .args(["-t", "raw"])
                    .stdout(Stdio::piped());

                let mut child = match cmd.spawn() {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Failed to start sox: {}", e);
                        return;
                    }
                };

                let stdout = child.stdout.take();
                if let Some(stdout) = stdout {
                    let mut reader = BufReader::with_capacity(buffer_size * 2, stdout);
                    let mut buffer = vec![0u8; buffer_size * 2];

                    while is_recording.load(Ordering::SeqCst) {
                        match reader.read_exact(&mut buffer) {
                            Ok(_) => {
                                let samples: Vec<i16> = buffer
                                    .chunks_exact(2)
                                    .map(|c| i16::from_le_bytes([c[0], c[1]]))
                                    .collect();
                                if !samples.is_empty() {
                                    audio_buffer.lock().unwrap().push_back(samples);
                                    frames_captured.fetch_add(1, Ordering::SeqCst);
                                }
                            }
                            Err(_) => break,
                        }
                    }
                }
                let _ = child.kill();
            });
            Ok(())
        }

        #[cfg(target_os = "windows")]
        fn start_platform_capture(&self) -> Result<()> {
            use std::process::Command;

            let sample_rate = self.sample_rate;
            let buffer_size = self.buffer_size;
            
            std::thread::spawn(move || {
                let _ = Command::new("powershell")
                    .args([
                        "-NoProfile", 
                        "-Command",
                        &format!("$h = Add-Type -MemberDefinition '[DllImport(\"winmm.dll\")] public static extern int waveInOpen(ref IntPtr p, int d, IntPtr f, IntPtr c, IntPtr i, uint f2); [AudioWin]::waveInOpen([ref]$p, 0, [IntPtr]::Zero, [IntPtr]::Zero, [IntPtr]::Zero, 0)' -Name AudioWin -PassThru"])
                    ])
                    .spawn();
            });
            Ok(())
        }

        #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
        fn start_platform_capture(&self) -> Result<()> {
            use std::io::BufReader;
            use std::process::{Command, Stdio};

            let audio_buffer = self.audio_buffer.clone();
            let is_recording = self.is_recording.clone();
            let frames_captured = self.frames_captured.clone();
            let buffer_size = self.buffer_size;
            let sample_rate = self.sample_rate;

            std::thread::spawn(move || {
                let mut cmd = Command::new("arecord");
                let device = self.device_id.as_deref().unwrap_or("default");
                cmd.args(["-D", device])
                    .args(["-f", "S16_LE", "-r", &sample_rate.to_string(), "-c", "1"])
                    .stdout(Stdio::piped());

                let mut child = match cmd.spawn() {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Failed to start arecord: {}", e);
                        return;
                    }
                };

                let stdout = child.stdout.take();
                if let Some(stdout) = stdout {
                    let mut reader = BufReader::with_capacity(buffer_size * 2, stdout);
                    let mut buffer = vec![0u8; buffer_size * 2];

                    while is_recording.load(Ordering::SeqCst) {
                        match reader.read_exact(&mut buffer) {
                            Ok(_) => {
                                let samples: Vec<i16> = buffer
                                    .chunks_exact(2)
                                    .map(|c| i16::from_le_bytes([c[0], c[1]]))
                                    .collect();
                                if !samples.is_empty() {
                                    audio_buffer.lock().unwrap().push_back(samples);
                                    frames_captured.fetch_add(1, Ordering::SeqCst);
                                }
                            }
                            Err(_) => break,
                        }
                    }
                }
                let _ = child.kill();
            });
            Ok(())
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MicrophoneConfig {
        pub sample_rate: u32,
        pub buffer_size: usize,
        pub channels: u16,
        pub format: String,
        pub device_id: Option<String>,
    }

    pub fn list_devices() -> Vec<MicrophoneDevice> {
        let mut devices = vec![MicrophoneDevice {
            id: "default".to_string(),
            name: "Default Microphone".to_string(),
            sample_rates: vec![16000, 44100, 48000],
            channels: 1,
            is_default: true,
        }];

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            if let Ok(output) = Command::new("system_profiler")
                .args(["-json", "SPAudioDeviceType"])
                .output()
            {
                if let Ok(json) =
                    serde_json::from_str::<serde_json::Value>(&String::from_utf8_lossy(&output))
                {
                    if let Some(devices_data) =
                        json.get("SPAudioDeviceType").and_then(|v| v.as_array())
                    {
                        devices.clear();
                        devices.push(MicrophoneDevice {
                            id: "default".to_string(),
                            name: "Default Microphone".to_string(),
                            sample_rates: vec![16000, 44100, 48000],
                            channels: 1,
                            is_default: true,
                        });
                        for device in devices_data {
                            if let Some(name) = device.get("_name").and_then(|v| v.as_str()) {
                                if !name.contains("Built-in") {
                                    devices.push(MicrophoneDevice {
                                        id: name.to_string(),
                                        name: name.to_string(),
                                        sample_rates: vec![16000, 44100, 48000],
                                        channels: 1,
                                        is_default: false,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let output = Command::new("powershell")
                .args([
                    "-NoProfile",
                    "-Command",
                    "Get-WmiObject Win32_SoundDevice | Select-Object Name,DeviceID | ConvertTo-Json",
                ])
                .output();

            if let Ok(output) = output {
                if let Ok(json) =
                    serde_json::from_str::<serde_json::Value>(&String::from_utf8_lossy(&output))
                {
                    let sound_devices = json.as_array();
                    if let Some(devices_arr) = sound_devices {
                        for device in devices_arr {
                            if let Some(name) = device.get("Name").and_then(|v| v.as_str()) {
                                devices.push(MicrophoneDevice {
                                    id: name.to_string(),
                                    name: name.to_string(),
                                    sample_rates: vec![16000, 44100, 48000],
                                    channels: 1,
                                    is_default: false,
                                });
                            }
                        }
                    }
                }
            }
        }

        #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
        {
            use std::process::Command;
            if let Ok(output) = Command::new("arecord").args(["--list-devices"]).output() {
                let output_str = String::from_utf8_lossy(&output);
                for line in output_str.lines() {
                    if line.contains("card") && line.contains("device") {
                        let parts: Vec<&str> = line.split(':').collect();
                        if parts.len() >= 2 {
                            let id = parts[0].trim().to_string();
                            let name = parts[1].trim().to_string();
                            devices.push(MicrophoneDevice {
                                id,
                                name,
                                sample_rates: vec![16000, 44100, 48000],
                                channels: 1,
                                is_default: false,
                            });
                        }
                    }
                }
            }
        }

        devices
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MicrophoneDevice {
        pub id: String,
        pub name: String,
        pub sample_rates: Vec<u32>,
        pub channels: u16,
        #[serde(default)]
        pub is_default: bool,
    }
}

// =============================================================================
// Audio-based Wake Word Detection
// =============================================================================

#[derive(Debug, Clone)]
pub struct FrequencyBand {
    pub low: f32,
    pub high: f32,
    pub name: &'static str,
}

pub struct WakeWordDetector {
    config: VoiceWakeConfig,
    preprocessor: AudioPreprocessor,
    energy_history: VecDeque<f32>,
    frequency_history: VecDeque<Vec<f32>>,
    history_size: usize,
    last_detection: Option<Instant>,
    enable_frequency_analysis: bool,
}

impl WakeWordDetector {
    pub fn new(config: VoiceWakeConfig) -> Self {
        Self {
            config: config.clone(),
            preprocessor: AudioPreprocessor::default(),
            energy_history: VecDeque::new(),
            frequency_history: VecDeque::new(),
            history_size: 10,
            last_detection: None,
            enable_frequency_analysis: true,
        }
    }

    pub fn with_frequency_analysis(mut self, enabled: bool) -> Self {
        self.enable_frequency_analysis = enabled;
        self
    }

    pub fn detect_from_audio(&mut self, samples: &[i16], sample_rate: u32) -> bool {
        self.process_samples(samples, sample_rate);

        let rms = calculate_rms(samples);
        self.energy_history.push_back(rms);
        if self.energy_history.len() > self.history_size {
            self.energy_history.pop_front();
        }

        let threshold = match self.config.sensitivity {
            WakeSensitivity::Low => -35.0,
            WakeSensitivity::Medium => -40.0,
            WakeSensitivity::High => -45.0,
        };

        if rms < threshold {
            return false;
        }

        if !self.is_energy_spike() {
            return false;
        }

        if self.enable_frequency_analysis {
            let bands = self.analyze_frequency_bands(samples, sample_rate);
            self.frequency_history.push_back(bands.clone());
            if self.frequency_history.len() > self.history_size {
                self.frequency_history.pop_front();
            }

            if !self.matches_voice_frequency_pattern(&bands) {
                return false;
            }
        }

        if let Some(last) = self.last_detection {
            let elapsed = Instant::now().duration_since(last).as_millis() as u64;
            if elapsed < self.config.min_wake_interval_ms {
                return false;
            }
        }

        self.last_detection = Some(Instant::now());
        true
    }

    fn analyze_frequency_bands(&self, samples: &[i16], sample_rate: u32) -> Vec<f32> {
        let fft_size = 512;
        let num_bands = 8;

        if samples.len() < fft_size {
            return vec![-60.0; num_bands];
        }

        let band_ranges = [
            (80.0, 250.0),     // Low
            (250.0, 500.0),    // Low-mid
            (500.0, 1000.0),   // Mid
            (1000.0, 2000.0),  // Mid-high
            (2000.0, 4000.0),  // High
            (4000.0, 6000.0),  // Very high
            (6000.0, 8000.0),  // Ultra high
            (8000.0, 12000.0), // Near ultrasound
        ];

        let mut band_energies = Vec::with_capacity(num_bands);

        for (low, high) in &band_ranges {
            let energy = self.calculate_band_energy(samples, sample_rate, *low, *high);
            band_energies.push(energy);
        }

        band_energies
    }

    fn calculate_band_energy(
        &self,
        samples: &[i16],
        sample_rate: f32,
        low_freq: f32,
        high_freq: f32,
    ) -> f32 {
        let bin_width = sample_rate / 512.0;
        let low_bin = (low_freq / bin_width) as usize;
        let high_bin = (high_freq / bin_width) as usize;

        let mut sum = 0.0f32;
        let mut count = 0usize;

        for i in low_bin..high_bin.min(samples.len() / 2) {
            let re = samples.get(i * 2).copied().unwrap_or(0) as f32;
            let im = samples.get(i * 2 + 1).copied().unwrap_or(0) as f32;
            sum += re * re + im * im;
            count += 1;
        }

        if count == 0 {
            return -120.0;
        }

        let avg = sum / count as f32;
        if avg <= 0.0 {
            return -120.0;
        }

        10.0 * avg.log10()
    }

    fn matches_voice_frequency_pattern(&self, bands: &[f32]) -> bool {
        if bands.len() < 5 {
            return true;
        }

        let voice_range_energy: f32 = bands[1..5].iter().sum::<f32>() / 4.0;

        let low_freq_energy = bands[0];
        let high_freq_energy = bands[5..].iter().sum::<f32>() / bands[5..].len() as f32;

        let voice_ratio = voice_range_energy - low_freq_energy;

        voice_ratio > 5.0 && voice_range_energy > -40.0
    }

    fn process_samples(&self, samples: &mut [i16], sample_rate: u32) -> &[i16] {
        self.preprocessor.process(samples, sample_rate);
        samples
    }

    fn is_energy_spike(&self) -> bool {
        if self.energy_history.len() < 3 {
            return true;
        }

        let avg: f32 = self.energy_history.iter().sum::<f32>() / self.energy_history.len() as f32;
        let current = *self.energy_history.back().unwrap_or(&avg);

        current > avg + 10.0
    }

    pub fn reset(&mut self) {
        self.energy_history.clear();
        self.frequency_history.clear();
        self.last_detection = None;
    }

    pub fn get_energy_history(&self) -> Vec<f32> {
        self.energy_history.iter().cloned().collect()
    }
}

fn calculate_rms(samples: &[i16]) -> f32 {
    if samples.is_empty() {
        return -120.0;
    }

    let sum_squares: f32 = samples
        .iter()
        .map(|&s| (s as f32 / i16::MAX as f32).powi(2))
        .sum();

    let rms = (sum_squares / samples.len() as f32).sqrt();

    if rms <= 0.0 {
        -120.0
    } else {
        20.0 * rms.log10()
    }
}

// =============================================================================
// Beep Sound Generation
// =============================================================================

pub struct BeepGenerator {
    sample_rate: u32,
}

impl BeepGenerator {
    pub fn new(sample_rate: u32) -> Self {
        Self { sample_rate }
    }

    pub fn generate_beep(&self, frequency: f32, duration_ms: u32, volume: f32) -> Vec<u8> {
        let num_samples = (self.sample_rate as f32 * duration_ms as f32 / 1000.0) as usize;
        let mut samples = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let t = i as f32 / self.sample_rate as f32;
            let sample = (2.0 * std::f32::consts::PI * frequency * t).sin() * volume;
            let sample_i16 = (sample * i16::MAX as f32) as i16;
            samples.extend_from_slice(&sample_i16.to_le_bytes());
        }

        self.wrap_in_wav(samples)
    }

    pub fn generate_wake_beep(&self) -> Vec<u8> {
        self.generate_beep(880.0, 150, 0.5)
    }

    pub fn generate_sleep_beep(&self) -> Vec<u8> {
        self.generate_beep(440.0, 200, 0.4)
    }

    pub fn generate_error_beep(&self) -> Vec<u8> {
        let mut samples = Vec::new();
        samples.extend_from_slice(&self.generate_beep(200.0, 100, 0.3));
        samples.extend_from_slice(&self.generate_beep(150.0, 100, 0.3));
        samples
    }

    fn wrap_in_wav(&self, samples: Vec<i16>) -> Vec<u8> {
        let data_size = samples.len() * 2;
        let file_size = 36 + data_size;

        let mut wav = Vec::with_capacity(44 + data_size);

        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&file_size.to_le_bytes());
        wav.extend_from_slice(b"WAVE");

        wav.extend_from_slice(b"fmt ");
        wav.extend_from_slice(&16u32.to_le_bytes());
        wav.extend_from_slice(&1u16.to_le_bytes());
        wav.extend_from_slice(&1u16.to_le_bytes());
        wav.extend_from_slice(&self.sample_rate.to_le_bytes());
        wav.extend_from_slice(&(self.sample_rate * 2).to_le_bytes());
        wav.extend_from_slice(&2u16.to_le_bytes());
        wav.extend_from_slice(&16u16.to_le_bytes());

        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&(data_size as u32).to_le_bytes());

        for sample in samples {
            wav.extend_from_slice(&sample.to_le_bytes());
        }

        wav
    }

    pub fn play_beep(&self, beep_type: BeepType) -> Result<()> {
        let wav_data = match beep_type {
            BeepType::Wake => self.generate_wake_beep(),
            BeepType::Sleep => self.generate_sleep_beep(),
            BeepType::Error => self.generate_error_beep(),
            BeepType::Custom { freq, dur, vol } => self.generate_beep(freq, dur, vol),
        };

        self.play_wav(&wav_data)
    }

    #[cfg(target_os = "windows")]
    fn play_wav(&self, _wav_data: &[u8]) -> Result<()> {
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn play_wav(&self, wav_data: &[u8]) -> Result<()> {
        use std::process::Command;
        let mut temp_path = std::env::temp_dir();
        temp_path.push("krabkrab_beep.wav");
        std::fs::write(&temp_path, wav_data)?;
        Command::new("afplay").arg(&temp_path).status()?;
        let _ = std::fs::remove_file(temp_path);
        Ok(())
    }

    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    fn play_wav(&self, wav_data: &[u8]) -> Result<()> {
        use std::process::Command;
        let mut temp_path = std::env::temp_dir();
        temp_path.push("krabkrab_beep.wav");
        std::fs::write(&temp_path, wav_data)?;
        let result = Command::new("ffplay")
            .args(["-nodisp", "-autoexit", "-loglevel", "quiet"])
            .arg(&temp_path)
            .status();
        let _ = std::fs::remove_file(temp_path);
        result?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BeepType {
    Wake,
    Sleep,
    Error,
    Custom { freq: f32, dur: u32, vol: f32 },
}

impl Default for BeepGenerator {
    fn default() -> Self {
        Self::new(16000)
    }
}

// =============================================================================
// Convenience Functions
// =============================================================================

/// Create a default voice mode controller
pub fn create_voice_controller() -> VoiceModeController {
    VoiceModeController::new(VoiceModesConfig::default())
}

/// Quick wake detection without session management
pub fn quick_detect(transcript: &str, wake_phrase: &str) -> bool {
    let decision = detect_wake_or_talk(transcript, wake_phrase, false);
    decision.action == VoiceAction::Wake
}

/// Create a beep generator with default settings
pub fn create_beep_generator() -> BeepGenerator {
    BeepGenerator::default()
}

/// Create a wake word detector with default config
pub fn create_wake_word_detector() -> WakeWordDetector {
    WakeWordDetector::new(VoiceWakeConfig::default())
}

/// Create a voice activity detector with default settings
pub fn create_vad() -> VoiceActivityDetector {
    VoiceActivityDetector::default()
}

/// Create a spectral analyzer with default settings
pub fn create_spectral_analyzer() -> SpectralAnalyzer {
    SpectralAnalyzer::default()
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

    #[test]
    fn voice_session_state_management() {
        let session = VoiceSession::with_default_config();

        assert_eq!(session.get_state(), VoiceSessionState::Idle);
        assert!(!session.is_awake());

        assert!(session.wake());
        assert!(session.is_awake());
        assert_eq!(session.get_state(), VoiceSessionState::Listening);

        session.sleep();
        assert!(!session.is_awake());
        assert_eq!(session.get_state(), VoiceSessionState::Idle);
    }

    #[test]
    fn voice_session_buffer() {
        let session = VoiceSession::with_default_config();

        session.add_to_buffer("hello".to_string());
        session.add_to_buffer("world".to_string());

        let buffer = session.get_buffer();
        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer[0], "hello");
        assert_eq!(buffer[1], "world");

        session.clear_buffer();
        assert!(session.get_buffer().is_empty());
    }

    #[test]
    fn voice_mode_controller_wake() {
        let controller = create_voice_controller();

        let decision = controller.process_audio("hey krabkrab open the door");
        assert_eq!(decision.action, VoiceAction::Wake);
        assert!(controller.get_state() == VoiceSessionState::Listening);

        let decision2 = controller.process_audio("what's the weather");
        assert_eq!(decision2.action, VoiceAction::Talk);
    }

    #[test]
    fn sleep_command_detection() {
        let controller = create_voice_controller();
        controller.wake();

        let decision = controller.process_audio("go to sleep");
        assert_eq!(decision.action, VoiceAction::Sleep);
        assert!(!controller.get_state() == VoiceSessionState::Listening);
    }

    #[test]
    fn alternative_wake_phrases() {
        let config = VoiceWakeConfig {
            enabled: true,
            wake_phrase: "hey krabkrab".to_string(),
            alternative_wake_phrases: vec!["krabkrab".to_string()],
            ..Default::default()
        };

        let decision = detect_wake_or_talk_with_config(
            "krabkrab what's up",
            "hey krabkrab",
            false,
            Some(&config),
        );
        assert_eq!(decision.action, VoiceAction::Wake);
    }

    #[test]
    fn audio_preprocessor_noise_gate() {
        let mut samples = vec![100i16, 50, 10, 1000, 500, 5];
        let preprocessor = AudioPreprocessor::new().with_noise_gate(-30.0);
        preprocessor.apply_noise_gate(&mut samples);

        // Low samples should be gated
        assert_eq!(samples[2], 0);
        assert_eq!(samples[5], 0);
        // Higher samples should remain
        assert!(samples[3] > 0);
    }

    #[test]
    fn quick_detect_function() {
        assert!(quick_detect("hey krabkrab hello", "hey krabkrab"));
        assert!(!quick_detect("hello world", "hey krabkrab"));
    }

    #[test]
    fn similarity_calculation() {
        let sim = calculate_similarity("hey krabkrab", "hey krab");
        assert!(sim > 0.5);

        let sim2 = calculate_similarity("hello world", "goodbye");
        assert!(sim2 < 0.5);
    }

    #[test]
    fn voice_session_info() {
        let controller = create_voice_controller();
        controller.wake();

        let info = controller.get_session_info();
        assert!(info.is_awake);
        assert_eq!(info.state, VoiceSessionState::Listening);
        assert!(info.time_since_wake_secs.is_some());
    }

    #[test]
    fn wake_word_detector_energy_based() {
        let config = VoiceWakeConfig {
            sensitivity: WakeSensitivity::Medium,
            ..Default::default()
        };
        let mut detector = WakeWordDetector::new(config);

        let loud_samples: Vec<i16> = (0..1600)
            .map(|i| {
                let t = i as f32 / 16000.0;
                let wave = (2.0 * std::f32::consts::PI * 440.0 * t).sin();
                (wave * 0.5 * i16::MAX as f32) as i16
            })
            .collect();

        let detected = detector.detect_from_audio(&loud_samples, 16000);
        assert!(detected, "Loud audio should trigger wake detection");
    }

    #[test]
    fn wake_word_detector_silent() {
        let config = VoiceWakeConfig::default();
        let mut detector = WakeWordDetector::new(config);

        let silent_samples: Vec<i16> = vec![0i16; 1600];
        let detected = detector.detect_from_audio(&silent_samples, 16000);
        assert!(!detected, "Silent audio should not trigger wake detection");
    }

    #[test]
    fn wake_word_detector_reset() {
        let config = VoiceWakeConfig::default();
        let mut detector = WakeWordDetector::new(config);

        let loud_samples: Vec<i16> = vec![1000i16; 1600];
        let _ = detector.detect_from_audio(&loud_samples, 16000);

        detector.reset();
        assert!(detector.energy_history.is_empty());
    }

    #[test]
    fn beep_generator_wake_beep() {
        let generator = BeepGenerator::new(16000);
        let wav = generator.generate_wake_beep();

        assert!(wav.len() > 44, "WAV should have header");
        assert!(&wav[0..4] == b"RIFF", "Should start with RIFF");
        assert!(&wav[8..12] == b"WAVE", "Should be WAVE format");
    }

    #[test]
    fn beep_generator_sleep_beep() {
        let generator = BeepGenerator::new(16000);
        let wav = generator.generate_sleep_beep();

        assert!(wav.len() > 44);
    }

    #[test]
    fn beep_generator_error_beep() {
        let generator = BeepGenerator::new(16000);
        let wav = generator.generate_error_beep();

        assert!(wav.len() > 44);
    }

    #[test]
    fn beep_generator_custom_beep() {
        let generator = BeepGenerator::new(16000);
        let wav = generator.generate_beep(1000.0, 100, 0.5);

        assert!(wav.len() > 44);
    }

    #[test]
    fn microphone_list_devices() {
        let devices = microphone::list_devices();
        assert!(!devices.is_empty());
        assert_eq!(devices[0].id, "default");
    }

    #[test]
    fn microphone_config() {
        let mic = microphone::MicrophoneCapture::new(16000, 1024);
        let config = mic.get_config();

        assert_eq!(config.sample_rate, 16000);
        assert_eq!(config.buffer_size, 1024);
        assert_eq!(config.channels, 1);
    }

    #[test]
    fn microphone_with_device() {
        let mic =
            microphone::MicrophoneCapture::new(16000, 1024).with_device("Built-in Microphone");
        let config = mic.get_config();

        assert_eq!(config.device_id, Some("Built-in Microphone".to_string()));
    }

    #[test]
    fn microphone_device_has_is_default() {
        let devices = microphone::list_devices();
        assert!(!devices.is_empty());
        assert!(devices[0].is_default);
    }

    #[test]
    fn microphone_read_frame() {
        let mic = microphone::MicrophoneCapture::new(16000, 1024);
        assert!(!mic.is_recording());
    }

    #[test]
    fn microphone_config_serialization() {
        let config = microphone::MicrophoneConfig {
            sample_rate: 44100,
            buffer_size: 2048,
            channels: 1,
            format: "pcm_s16le".to_string(),
            device_id: Some("test".to_string()),
        };
        let json = serde_json::to_string(&config).unwrap();
        let restored: microphone::MicrophoneConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.sample_rate, 44100);
        assert_eq!(restored.device_id, Some("test".to_string()));
    }

    #[test]
    fn microphone_device_serialization() {
        let device = microphone::MicrophoneDevice {
            id: "test-device".to_string(),
            name: "Test Device".to_string(),
            sample_rates: vec![16000, 44100],
            channels: 2,
            is_default: false,
        };
        let json = serde_json::to_string(&device).unwrap();
        let restored: microphone::MicrophoneDevice = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, "test-device");
        assert!(!restored.is_default);
    }

    #[test]
    fn calculate_rms_loud() {
        let samples: Vec<i16> = vec![10000i16; 100];
        let rms = calculate_rms(&samples);

        assert!(rms > -20.0, "Loud samples should have high RMS");
    }

    #[test]
    fn calculate_rms_silent() {
        let samples: Vec<i16> = vec![0i16; 100];
        let rms = calculate_rms(&samples);

        assert!(rms < -100.0, "Silent samples should have very low RMS");
    }

    #[test]
    fn voice_decision_has_confidence() {
        let decision = detect_wake_or_talk("hey krabkrab hello", "hey krabkrab", false);
        assert!(decision.confidence > 0.0);
    }

    #[test]
    fn talk_mode_config_defaults() {
        let config = TalkModeConfig::default();
        assert!(config.enabled);
        assert_eq!(config.timeout_seconds, 60);
        assert!(config.auto_sleep_after_inactivity);
        assert!(config.tts_enabled);
    }

    #[test]
    fn voice_modes_config_defaults() {
        let config = VoiceModesConfig::default();
        assert!(config.wake.enabled);
        assert!(config.talk.enabled);
        assert_eq!(config.sample_rate, 16000);
    }

    #[test]
    fn vad_speech_detection() {
        let mut vad = VoiceActivityDetector::new();

        let speech_samples: Vec<i16> = (0..1600)
            .map(|i| {
                let t = i as f32 / 16000.0;
                let wave = (2.0 * std::f32::consts::PI * 440.0 * t).sin();
                (wave * 0.5 * i16::MAX as f32) as i16
            })
            .collect();

        let state = vad.process(&speech_samples, 16000);
        assert!(state == VADState::Speaking || state == VADState::SpeechStart);
    }

    #[test]
    fn vad_silence() {
        let mut vad = VoiceActivityDetector::new();

        let silent_samples: Vec<i16> = vec![0i16; 1600];
        let state = vad.process(&silent_samples, 16000);

        assert_eq!(state, VADState::Silence);
        assert!(!vad.is_speaking());
    }

    #[test]
    fn vad_reset() {
        let mut vad = VoiceActivityDetector::new();

        let speech_samples: Vec<i16> = vec![10000i16; 1600];
        let _ = vad.process(&speech_samples, 16000);

        vad.reset();
        assert!(!vad.is_speaking());
    }

    #[test]
    fn spectral_analyzer_basic() {
        let analyzer = SpectralAnalyzer::new(512);

        let samples: Vec<i16> = (0..512).map(|i| (i as i16)).collect();
        let features = analyzer.analyze(&samples);

        assert!(features.spectral_flux >= 0.0);
        assert!(features.spectral_rolloff >= 0.0 && features.spectral_rolloff <= 1.0);
        assert!(features.spectral_flatness >= 0.0 && features.spectral_flatness <= 1.0);
    }

    #[test]
    fn spectral_analyzer_empty() {
        let analyzer = SpectralAnalyzer::new(512);
        let features = analyzer.analyze(&[]);

        assert_eq!(features.spectral_flux, 0.0);
        assert_eq!(features.spectral_rolloff, 0.0);
    }

    #[test]
    fn audio_stats_from_samples() {
        let samples: Vec<i16> = vec![1000i16; 1600];
        let stats = AudioStats::from_samples(&samples, 16000);

        assert_eq!(stats.sample_rate, 16000);
        assert!(stats.duration_ms > 0);
        assert!(stats.rms_dbfs > -60.0);
    }

    #[test]
    fn wake_config_vad_settings() {
        let config = VoiceWakeConfig::default();
        assert!(config.vad_enabled);
        assert_eq!(config.vad_threshold_db, -40.0);
    }

    #[test]
    fn talk_config_vad_settings() {
        let config = TalkModeConfig::default();
        assert!(config.vad_enabled);
        assert_eq!(config.speech_timeout_seconds, 10);
    }

    #[test]
    fn voice_modes_spectral_analysis() {
        let config = VoiceModesConfig::default();
        assert!(config.spectral_analysis);
    }

    #[test]
    fn vad_state_serialization() {
        let states = vec![
            VADState::Silence,
            VADState::Speaking,
            VADState::SpeechStart,
            VADState::SpeechEnd,
        ];

        for state in &states {
            let json = serde_json::to_string(state).unwrap();
            let restored: VADState = serde_json::from_str(&json).unwrap();
            assert_eq!(*state, restored);
        }
    }
}
