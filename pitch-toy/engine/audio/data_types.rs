// Generic data types for audio module
// These structures allow the audio module to remain independent
// while still providing data to other modules that may need it

use super::AudioWorkletState;

/// Volume level data for external consumption
#[derive(Debug, Clone, PartialEq)]
pub struct VolumeLevelData {
    pub rms_amplitude: f32,
    pub peak_amplitude: f32,
}

/// Pitch detection data for external consumption
#[derive(Debug, Clone, PartialEq)]
pub struct PitchData {
    pub frequency: f32,
    pub confidence: f32,
    pub clarity: f32,
    pub timestamp: f64,
}

/// AudioWorklet status for external consumption
#[derive(Debug, Clone, PartialEq)]
pub struct AudioWorkletStatus {
    pub state: AudioWorkletState,
    pub processor_loaded: bool,
    pub chunk_size: u32,
    pub chunks_processed: u32,
    pub last_update: f64,
}

impl Default for AudioWorkletStatus {
    fn default() -> Self {
        Self {
            state: AudioWorkletState::Uninitialized,
            processor_loaded: false,
            chunk_size: 128,
            chunks_processed: 0,
            last_update: 0.0,
        }
    }
}