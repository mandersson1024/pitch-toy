// Audio module for pitch-toy application
// 
// This module provides comprehensive audio processing capabilities using dependency injection
// with AudioSystemContext for centralized audio component management.
// 
// # Architecture
// 
// The audio system is built around AudioSystemContext which uses dependency injection
// to manage all audio components:
// - AudioContextManager: Web Audio API management
// - AudioWorkletManager: Real-time audio processing
// - PitchAnalyzer: Pitch detection and analysis
// - Data setters: Reactive data updates
// 
// # Usage
// 
// ```rust
// // Initialize audio system with dependency injection
// let context = initialize_audio_system_with_context(
//     volume_setter,
//     pitch_setter,
//     status_setter,
// ).await?;
// 
// // Connect microphone using context
// connect_microphone_to_audioworklet_with_context(&context).await?;
// 
// // Setup UI action listeners with context
// setup_ui_action_listeners_with_context(listeners, permission_setter, context);
// ```
// 
// # Migration from Global State
// 
// This module has been migrated from global state access to dependency injection.
// The AudioSystemContext provides centralized access to all audio components,
// eliminating the need for global state management.

pub mod microphone;
pub mod context;
pub mod worklet;
pub mod stream;
pub mod permission;
pub mod buffer;
pub mod buffer_analyzer;
pub mod console_service;
pub mod commands;
pub mod pitch_detector;
pub mod note_mapper;
pub mod pitch_analyzer;
pub mod volume_detector;
pub mod test_signal_generator;
pub mod message_protocol;
pub mod data_types;

use crate::common::dev_log;

use std::cell::RefCell;
use std::rc::Rc;

// Global audio context manager for application-wide access
// TODO: FUTURE REFACTORING - Remove this global variable and replace with dependency injection through AudioSystemContext.
// This is a planned future task. Do NOT refactor this during unrelated work.
// See docs/global_variables_refactoring_guide.md for refactoring strategy.
thread_local! {
    static AUDIO_CONTEXT_MANAGER: RefCell<Option<Rc<RefCell<context::AudioContextManager>>>> = RefCell::new(None);
}

// Note: Buffer pool global state removed - using direct processing with transferable buffers



/// Initialize audio system with dependency injection
/// 
/// This function creates and initializes a complete AudioSystemContext with all required
/// audio components using proper dependency injection patterns.
/// 
/// # Parameters
/// - `volume_level_setter`: Data setter for volume level updates
/// - `pitch_data_setter`: Data setter for pitch detection data
/// - `audioworklet_status_setter`: Data setter for AudioWorklet status updates
/// 
/// # Returns
/// - `Ok(AudioSystemContext)` with fully initialized audio system
/// - `Err(String)` with error details if initialization failed
/// 
/// # Components Initialized
/// - AudioContextManager for Web Audio API management
/// - AudioWorkletManager for real-time audio processing
/// - PitchAnalyzer for pitch detection and analysis
/// - Data setter integration for reactive updates
/// 
/// # Example
/// ```rust
/// let volume_setter = /* volume data setter */;
/// let pitch_setter = /* pitch data setter */;
/// let status_setter = /* status data setter */;
/// 
/// let context = initialize_audio_system_with_context(
///     volume_setter,
///     pitch_setter,
///     status_setter,
/// ).await?;
/// ```
pub async fn initialize_audio_system_with_context(
    volume_level_setter: std::rc::Rc<dyn observable_data::DataSetter<Option<VolumeLevelData>>>,
    pitch_data_setter: std::rc::Rc<dyn observable_data::DataSetter<Option<PitchData>>>,
    audioworklet_status_setter: std::rc::Rc<dyn observable_data::DataSetter<AudioWorkletStatus>>,
    buffer_pool_stats_setter: std::rc::Rc<dyn observable_data::DataSetter<Option<message_protocol::BufferPoolStats>>>,
) -> Result<context::AudioSystemContext, String> {
    dev_log!("Initializing audio system with dependency injection");
    
    // Check AudioContext support
    if !context::AudioContextManager::is_supported() {
        return Err("Web Audio API not supported".to_string());
    }
    
    // Create AudioSystemContext with setters passed at construction
    let mut context = context::AudioSystemContext::new(
        volume_level_setter,
        pitch_data_setter,
        audioworklet_status_setter,
        buffer_pool_stats_setter,
    );
    
    // Initialize the context (this handles all component initialization)
    context.initialize().await
        .map_err(|e| format!("AudioSystemContext initialization failed: {}", e))?;
    
    dev_log!("✓ Audio system initialization completed with dependency injection");
    Ok(context)
}


/// Store AudioContextManager globally for backward compatibility
pub fn set_global_audio_context_manager(manager: Rc<RefCell<context::AudioContextManager>>) {
    AUDIO_CONTEXT_MANAGER.with(|global_manager| {
        *global_manager.borrow_mut() = Some(manager);
    });
}

/// Get the global AudioContext manager
/// Returns None if audio system hasn't been initialized
pub fn get_audio_context_manager() -> Option<Rc<RefCell<context::AudioContextManager>>> {
    AUDIO_CONTEXT_MANAGER.with(|manager| {
        manager.borrow().as_ref().cloned()
    })
}

/// Check if audio system is initialized and running
pub fn is_audio_system_ready() -> bool {
    AUDIO_CONTEXT_MANAGER.with(|manager| {
        if let Some(ref audio_manager_rc) = *manager.borrow() {
            audio_manager_rc.borrow().is_running()
        } else {
            false
        }
    })
}

/// Create a ConsoleAudioService instance
/// Returns a configured console audio service with audio context manager if available
pub fn create_console_audio_service() -> console_service::ConsoleAudioServiceImpl {
    let mut service = console_service::ConsoleAudioServiceImpl::new();
    
    // Set audio context manager if available
    if let Some(manager) = get_audio_context_manager() {
        service.set_audio_context_manager(manager);
    }
    
    service
}




// Note: Buffer pool global functions removed - using direct processing with transferable buffers






// Note: initialize_buffer_pool removed - using direct processing with transferable buffers


// Re-export public API
pub use microphone::{MicrophoneManager, AudioStreamInfo, AudioError, connect_microphone_to_audioworklet_with_context};
pub use permission::{AudioPermission, connect_microphone_with_context};
pub use context::{AudioContextManager, AudioContextState, AudioContextConfig, AudioDevices, AudioSystemContext};
pub use worklet::{AudioWorkletManager, AudioWorkletState, AudioWorkletConfig};
pub use stream::{StreamReconnectionHandler, StreamState, StreamHealth, StreamConfig, StreamError};
pub use permission::PermissionManager;
pub use buffer::{CircularBuffer, BufferState, PRODUCTION_BUFFER_SIZE, DEV_BUFFER_SIZE_MIN, DEV_BUFFER_SIZE_MAX, DEV_BUFFER_SIZE_DEFAULT, AUDIO_CHUNK_SIZE, get_buffer_size, validate_buffer_size, validate_buffer_size_for_creation};
pub use buffer_analyzer::{BufferAnalyzer, WindowFunction};
// Note: BufferPool re-export removed - using direct processing with transferable buffers
pub use console_service::{ConsoleAudioService, ConsoleAudioServiceImpl, AudioStatus};
pub use commands::register_audio_commands;
pub use pitch_detector::{PitchResult, PitchDetectorConfig, MusicalNote, NoteName, TuningSystem, PitchDetector, PitchDetectionError};
pub use note_mapper::NoteMapper;
pub use pitch_analyzer::{PitchAnalyzer, PitchPerformanceMetrics, PitchAnalysisError};
pub use volume_detector::{VolumeDetector, VolumeDetectorConfig, VolumeAnalysis};
pub use test_signal_generator::{TestSignalGenerator, TestSignalGeneratorConfig, TestWaveform, BackgroundNoiseConfig};
pub use data_types::{VolumeLevelData, PitchData, AudioWorkletStatus};
pub use message_protocol::{
    ToWorkletMessage, FromWorkletMessage, ToWorkletEnvelope, FromWorkletEnvelope,
    AudioDataBatch, ProcessorStatus, BatchConfig, WorkletError, WorkletErrorCode,
    ErrorContext, MemoryUsage, MessageEnvelope, 
    SerializationResult, SerializationError, ToJsMessage, FromJsMessage, MessageValidator,
    MessageSerializer, MessageDeserializer,
    MessageConstructionResult, MessageConstructionError, MessageIdGenerator,
    MessageBuilder, AudioWorkletMessageFactory, generate_unique_message_id,
    get_high_resolution_timestamp,
    // Enhanced error handling types
    MessageProtocolError, ValidationError, TransferError, MessageProtocolResult, ValidationResult, TransferResult,
    MessageContext, MessageDirection, SystemState
};

/// Setup UI action listeners for audio module with AudioSystemContext
/// 
/// This function sets up action listeners for audio-related UI controls using dependency injection.
/// The AudioSystemContext parameter provides access to all audio components needed for configuration.
/// 
/// # Parameters
/// - `listeners`: UI control listeners for audio actions
/// - `microphone_permission_setter`: Data setter for microphone permission state updates
/// - `audio_context`: AudioSystemContext instance containing all audio components
/// 
pub fn setup_ui_action_listeners_with_context(
    listeners: crate::UIControlListeners,
    microphone_permission_setter: impl observable_data::DataSetter<AudioPermission> + Clone + 'static,
    audio_context: std::rc::Rc<std::cell::RefCell<AudioSystemContext>>,
) {
    
    // Test signal action listener
    let audio_context_clone = audio_context.clone();
    listeners.test_signal.listen(move |action| {
        dev_log!("Received test signal action: {:?}", action);
        
        let mut context = audio_context_clone.borrow_mut();
        if let Some(worklet_manager) = context.get_audioworklet_manager_mut() {
            // Convert UI action to audio system config
            let audio_config = TestSignalGeneratorConfig {
                enabled: action.enabled,
                frequency: action.frequency,
                amplitude: action.volume / 100.0, // Convert percentage to 0-1 range
                waveform: action.waveform,
                sample_rate: 48000.0, // Use standard sample rate
            };
            
            worklet_manager.update_test_signal_config(audio_config);
            dev_log!("✓ Test signal config updated via action");
        } else {
            dev_log!("Warning: No AudioWorklet manager available for test signal config");
        }
    });
    
    // Background noise action listener
    let audio_context_clone = audio_context.clone();
    listeners.background_noise.listen(move |action| {
        dev_log!("Received background noise action: {:?}", action);
        
        let mut context = audio_context_clone.borrow_mut();
        if let Some(worklet_manager) = context.get_audioworklet_manager_mut() {
            // Convert UI action to audio system config
            let audio_config = BackgroundNoiseConfig {
                enabled: action.enabled,
                level: action.level,
                noise_type: action.noise_type,
            };
            
            worklet_manager.update_background_noise_config(audio_config);
            dev_log!("✓ Background noise config updated via action");
        } else {
            dev_log!("Warning: No AudioWorklet manager available for background noise config");
        }
    });
    
    // Output to speakers action listener
    let audio_context_clone = audio_context.clone();
    listeners.output_to_speakers.listen(move |action| {
        dev_log!("Received output to speakers action: {:?}", action);
        
        let mut context = audio_context_clone.borrow_mut();
        if let Some(worklet_manager) = context.get_audioworklet_manager_mut() {
            worklet_manager.set_output_to_speakers(action.enabled);
            dev_log!("✓ Output to speakers setting updated via action");
        } else {
            dev_log!("Warning: No AudioWorklet manager available for output to speakers setting");
        }
    });
    
    // Microphone permission action listener  
    let audio_context_clone = audio_context.clone();
    let microphone_permission_setter_clone = microphone_permission_setter.clone();
    listeners.microphone_permission.listen(move |action| {
        dev_log!("Received microphone permission action: {:?}", action);
        
        if action.request_permission {
            // Now we can use the context-aware microphone connection!
            wasm_bindgen_futures::spawn_local({
                let audio_context = audio_context_clone.clone();
                let permission_setter = microphone_permission_setter_clone.clone();
                
                async move {
                    match microphone::connect_microphone_to_audioworklet_with_context(&audio_context).await {
                        Ok(_) => {
                            dev_log!("✓ Microphone permission granted and connected via action");
                            permission_setter.set(AudioPermission::Granted);
                        }
                        Err(e) => {
                            dev_log!("✗ Microphone permission denied or error: {}", e);
                            // Determine the appropriate permission state based on the error
                            if e.contains("Permission denied") || e.contains("NotAllowedError") {
                                permission_setter.set(AudioPermission::Denied);
                            } else {
                                permission_setter.set(AudioPermission::Unavailable);
                            }
                        }
                    }
                }
            });
        }
    });
}


#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    // No wasm_bindgen_test_configure! needed for Node.js
   


    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn test_audio_error_types() {
        let permission_error = AudioError::PermissionDenied("Test permission denied".to_string());
        assert!(permission_error.to_string().contains("Permission denied"));
        assert!(permission_error.to_string().contains("Test permission denied"));

        let device_error = AudioError::DeviceUnavailable("Test device unavailable".to_string());
        assert!(device_error.to_string().contains("Device unavailable"));
        assert!(device_error.to_string().contains("Test device unavailable"));

        let not_supported_error = AudioError::NotSupported("Test not supported".to_string());
        assert!(not_supported_error.to_string().contains("Not supported"));
        assert!(not_supported_error.to_string().contains("Test not supported"));

        let stream_error = AudioError::StreamInitFailed("Test stream init failed".to_string());
        assert!(stream_error.to_string().contains("Stream initialization failed"));
        assert!(stream_error.to_string().contains("Test stream init failed"));

        let generic_error = AudioError::Generic("Test generic error".to_string());
        assert!(generic_error.to_string().contains("Audio error"));
        assert!(generic_error.to_string().contains("Test generic error"));
    }

    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn test_microphone_state_enum() {
        // Test all microphone states
        assert_eq!(AudioPermission::Uninitialized.to_string(), "Uninitialized");
        assert_eq!(AudioPermission::Requesting.to_string(), "Requesting");
        assert_eq!(AudioPermission::Granted.to_string(), "Granted");
        assert_eq!(AudioPermission::Denied.to_string(), "Denied");
        assert_eq!(AudioPermission::Unavailable.to_string(), "Unavailable");

        // Test PartialEq implementation
        assert_eq!(AudioPermission::Granted, AudioPermission::Granted);
        assert_ne!(AudioPermission::Granted, AudioPermission::Denied);
    }

    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn test_audio_context_state_enum() {
        // Test all audio context states
        assert_eq!(AudioContextState::Uninitialized.to_string(), "Uninitialized");
        assert_eq!(AudioContextState::Initializing.to_string(), "Initializing");
        assert_eq!(AudioContextState::Running.to_string(), "Running");
        assert_eq!(AudioContextState::Suspended.to_string(), "Suspended");
        assert_eq!(AudioContextState::Closed.to_string(), "Closed");
        assert_eq!(AudioContextState::Recreating.to_string(), "Recreating");

        // Test PartialEq implementation
        assert_eq!(AudioContextState::Running, AudioContextState::Running);
        assert_ne!(AudioContextState::Running, AudioContextState::Suspended);
    }

    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn test_stream_state_enum() {
        // Test all stream states
        assert_eq!(StreamState::Disconnected, StreamState::Disconnected);
        assert_eq!(StreamState::Connecting, StreamState::Connecting);
        assert_eq!(StreamState::Connected, StreamState::Connected);
        assert_eq!(StreamState::Reconnecting, StreamState::Reconnecting);
        assert_eq!(StreamState::Failed, StreamState::Failed);

        // Test different states are not equal
        assert_ne!(StreamState::Connected, StreamState::Disconnected);
        assert_ne!(StreamState::Connecting, StreamState::Reconnecting);
    }

    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn test_stream_error_types() {
        let device_disconnected = StreamError::DeviceDisconnected;
        assert_eq!(device_disconnected.to_string(), "Audio device disconnected");

        let permission_revoked = StreamError::PermissionRevoked;
        assert_eq!(permission_revoked.to_string(), "Microphone permission revoked");

        let unknown_device = StreamError::UnknownDevice;
        assert_eq!(unknown_device.to_string(), "Unknown audio device");

        let reconnection_failed = StreamError::ReconnectionFailed;
        assert_eq!(reconnection_failed.to_string(), "Failed to reconnect audio stream");

        let stream_ended = StreamError::StreamEnded;
        assert_eq!(stream_ended.to_string(), "Audio stream ended unexpectedly");

        let config_error = StreamError::ConfigurationError("Test config error".to_string());
        assert!(config_error.to_string().contains("Stream configuration error"));
        assert!(config_error.to_string().contains("Test config error"));
    }

    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn test_audio_stream_info_default() {
        let info = AudioStreamInfo::default();
        assert_eq!(info.sample_rate, 48000.0);
        assert_eq!(info.buffer_size, 1024);
        assert!(info.device_id.is_none());
        assert!(info.device_label.is_none());
    }

    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn test_audio_context_config_default() {
        let config = AudioContextConfig::default();
        assert_eq!(config.sample_rate, 48000.0);
        assert_eq!(config.buffer_size, 1024);
        assert_eq!(config.max_recreation_attempts, 3);
    }

    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn test_stream_config_default() {
        let config = StreamConfig::default();
        assert_eq!(config.max_reconnect_attempts, 3);
        assert_eq!(config.reconnect_delay_ms, 1000);
        assert_eq!(config.health_check_interval_ms, 5000);
        assert_eq!(config.activity_timeout_ms, 10000);
    }

    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn test_manager_creation() {
        // Test that all managers can be created successfully
        let mic_manager = MicrophoneManager::new();
        assert_eq!(*mic_manager.state(), AudioPermission::Uninitialized);

        let audio_manager = AudioContextManager::new();
        assert_eq!(*audio_manager.state(), AudioContextState::Uninitialized);

        let stream_handler = StreamReconnectionHandler::new(StreamConfig::default());
        assert_eq!(stream_handler.get_health().state, StreamState::Disconnected);
    }

    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn test_error_handling_integration() {
        // Test that error types can be properly used together
        let audio_error = AudioError::Generic("Integration test".to_string());
        let stream_error = StreamError::ConfigurationError("Integration test".to_string());
        
        // Both should format correctly
        assert!(audio_error.to_string().contains("Integration test"));
        assert!(stream_error.to_string().contains("Integration test"));
        
        // Both should be Debug-formatted correctly
        assert!(format!("{:?}", audio_error).contains("Generic"));
        assert!(format!("{:?}", stream_error).contains("ConfigurationError"));
    }
}