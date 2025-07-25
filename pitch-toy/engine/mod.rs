//! Engine Layer - Audio processing and hardware interface
//!
//! This layer handles low-level audio operations and browser API interactions.
//! It communicates with the Model layer by returning structured data from update calls.
//!
//! ## Data Flow in Engine Layer
//!
//! The engine layer:
//! - Processes audio data from microphone and browser APIs
//! - Returns structured data via EngineUpdateResult from update() calls
//! - Provides audio analysis, error information, and permission state
//!
//! ```rust
//! use pitch_toy::engine::AudioEngine;
//!
//! // Create engine without dependencies
//! let mut engine = AudioEngine::create().await?;
//!
//! // Engine returns data directly from update calls
//! let result = engine.update(timestamp);
//! // result contains: audio_analysis, audio_errors, permission_state
//! 
//! // Access the aggregated data
//! if let Some(analysis) = result.audio_analysis {
//!     println!("Volume: {} dB", analysis.volume_level.peak);
//!     match analysis.pitch {
//!         Pitch::Detected(freq, clarity) => {
//!             println!("Pitch: {} Hz (clarity: {})", freq, clarity);
//!         }
//!         Pitch::NotDetected => println!("No pitch detected"),
//!     }
//! }
//! 
//! // Check for errors
//! for error in &result.audio_errors {
//!     eprintln!("Audio error: {:?}", error);
//! }
//! 
//! // Check permission state
//! match result.permission_state {
//!     PermissionState::Granted => println!("Microphone access granted"),
//!     PermissionState::Denied => println!("Microphone access denied"),
//!     _ => {}
//! }
//! ```

pub mod audio;
pub(crate) mod platform;

use crate::shared_types::EngineUpdateResult;
use crate::model::ModelLayerActions;

// Debug-only imports for conditional compilation
#[cfg(debug_assertions)]
use crate::presentation::{DebugLayerActions, ConfigureTestSignal, ConfigureOutputToSpeakers, ConfigureBackgroundNoise};
#[cfg(debug_assertions)]
use self::audio::{TestWaveform, AudioDevices, AudioWorkletStatus, message_protocol::BufferPoolStats};

/// Execution action for microphone permission requests
/// 
/// This unit struct represents the execution of a microphone permission request 
/// at the engine layer. It contains no additional data as the execution process
/// is handled entirely by the existing microphone connection functionality.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ExecuteMicrophonePermissionRequest;

/// Execution action for audio system configuration
/// 
/// This struct represents the execution of audio system configuration at the engine layer.
/// Temporarily disabled as tuning system handling is being moved to model layer.
/// #[derive(Debug, Clone, PartialEq)]
/// pub(crate) struct ConfigureAudioSystem {
///     /// The tuning system to configure in the audio processing pipeline
///     pub tuning_system: TuningSystem,
/// }

/// Execution action for tuning configuration updates
/// 
/// This struct represents the execution of tuning configuration updates at the engine layer.
/// Temporarily disabled as tuning system handling is being moved to model layer.
/// #[derive(Debug, Clone, PartialEq)]
/// pub(crate) struct UpdateTuningConfiguration {
///     /// The tuning system being used
///     pub tuning_system: TuningSystem,
///     /// The root note that will serve as the tonic
///     pub root_note: Note,
/// }

/// Container for all executed engine layer actions
/// 
/// This struct contains vectors of low-level execution actions that have been
/// processed by the engine layer. These actions represent the actual operations
/// performed on the audio system hardware and processing pipeline.
/// 
/// The engine layer receives `ModelLayerActions` from the model layer, transforms
/// them into executable operations, performs the execution, and returns the results
/// as `EngineLayerActions` for logging and feedback purposes.
/// 
/// Temporarily simplified as tuning system handling is being moved to model layer.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EngineLayerActions {
    // Placeholder for future action types
    // Executed audio system configurations
    // pub audio_system_configurations: Vec<ConfigureAudioSystem>,
    
    // Executed tuning configuration updates  
    // pub tuning_configurations: Vec<UpdateTuningConfiguration>,
}

impl EngineLayerActions {
    /// Create a new instance with empty action collections
    /// 
    /// Returns a new `EngineLayerActions` struct with all action vectors initialized
    /// as empty. This is used as the starting point for collecting executed actions.
    /// Temporarily simplified as tuning system handling is being moved to model layer.
    pub(crate) fn new() -> Self {
        Self {
            // audio_system_configurations: Vec::new(),
            // tuning_configurations: Vec::new(),
        }
    }
}

// Debug execution action structs (only available in debug builds)
#[cfg(debug_assertions)]
#[derive(Debug, Clone, PartialEq)]
pub struct ExecuteTestSignalConfiguration {
    pub enabled: bool,
    pub frequency: f32,
    pub volume: f32,
    pub waveform: TestWaveform,
}

#[cfg(debug_assertions)]
#[derive(Debug, Clone, PartialEq)]
pub struct ExecuteOutputToSpeakersConfiguration {
    pub enabled: bool,
}

#[cfg(debug_assertions)]
#[derive(Debug, Clone, PartialEq)]
pub struct ExecuteBackgroundNoiseConfiguration {
    pub enabled: bool,
    pub level: f32,
    pub noise_type: TestWaveform,
}

/// Container for all executed debug layer actions (debug builds only)
/// 
/// This struct contains vectors of privileged debug execution actions that have been
/// processed by the engine layer. These actions represent direct operations on the
/// audio system that bypass normal validation and safety checks.
/// 
/// Debug actions provide privileged access to engine internals for testing purposes:
/// - Direct test signal generation control
/// - Direct speaker output manipulation
/// - Direct background noise injection
/// 
/// These actions should only be used for debugging and testing purposes.
#[cfg(debug_assertions)]
#[derive(Debug, Clone, PartialEq)]
pub struct DebugEngineActions {
    /// Executed test signal configurations
    pub test_signal_executions: Vec<ExecuteTestSignalConfiguration>,
    
    /// Executed speaker output configurations
    pub speaker_output_executions: Vec<ExecuteOutputToSpeakersConfiguration>,
    
    /// Executed background noise configurations
    pub background_noise_executions: Vec<ExecuteBackgroundNoiseConfiguration>,
}

#[cfg(debug_assertions)]
impl DebugEngineActions {
    /// Create a new instance with empty debug action collections
    /// 
    /// Returns a new `DebugEngineActions` struct with all action vectors initialized
    /// as empty. This is used as the starting point for collecting executed debug actions.
    pub fn new() -> Self {
        Self {
            test_signal_executions: Vec::new(),
            speaker_output_executions: Vec::new(),
            background_noise_executions: Vec::new(),
        }
    }
}

/// AudioEngine - The engine layer of the three-layer architecture
/// 
/// This struct represents the audio processing and hardware interface layer
/// of the application. It handles low-level audio operations, browser API
/// interactions, and microphone/speaker communication.
/// 
/// # Example
/// 
/// ```no_run
/// use pitch_toy::engine::AudioEngine;
/// 
/// let engine = AudioEngine::create()
///     .await.expect("AudioEngine creation should succeed");
/// ```
pub struct AudioEngine {
    /// Audio system context for managing audio processing
    audio_context: Option<std::rc::Rc<std::cell::RefCell<audio::AudioSystemContext>>>,
}

impl AudioEngine {
    /// Create a new AudioEngine without observable data dependencies
    /// 
    /// This constructor initializes the audio processing system using direct
    /// data return patterns instead of the observable data pattern.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(AudioEngine)` on successful initialization, or `Err(String)`
    /// if audio system initialization fails.
    pub async fn create() -> Result<Self, String> {
        crate::common::dev_log!("Creating AudioEngine with return-based pattern");
        
        // Create audio context using the new return-based constructor
        let mut audio_context = audio::AudioSystemContext::new_return_based();
        
        // Initialize the audio system
        match audio_context.initialize().await {
            Ok(()) => {
                crate::common::dev_log!("✓ AudioEngine created and initialized successfully");
                Ok(Self {
                    audio_context: Some(std::rc::Rc::new(std::cell::RefCell::new(audio_context))),
                })
            }
            Err(e) => {
                crate::common::dev_log!("⚠ AudioEngine created but audio system initialization failed: {}", e);
                // Still create the engine but without audio context for now
                // This allows the application to continue running
                Ok(Self {
                    audio_context: None,
                })
            }
        }
    }

    /// Update the engine layer with a new timestamp
    /// 
    /// This method is called by the main render loop to update the engine's state.
    /// It processes audio data, handles device changes, and returns updates
    /// for the model layer.
    /// 
    /// # Arguments
    /// 
    /// * `timestamp` - The current timestamp in seconds since application start
    /// 
    /// # Returns
    /// 
    /// Returns `EngineUpdateResult` containing audio analysis data, errors, and permission state
    pub fn update(&mut self, timestamp: f64) -> EngineUpdateResult {
        if let Some(ref context) = self.audio_context {
            // Borrow once and collect all data to avoid multiple borrows
            let borrowed_context = context.borrow();
            let audio_analysis = borrowed_context.collect_audio_analysis(timestamp);
            let audio_errors = borrowed_context.collect_audio_errors();
            let permission_state = borrowed_context.collect_permission_state();
            
            EngineUpdateResult {
                audio_analysis,
                audio_errors,
                permission_state,
            }
        } else {
            // No audio context available
            EngineUpdateResult {
                audio_analysis: None,
                audio_errors: vec![crate::shared_types::Error::ProcessingError("Audio system not initialized".to_string())],
                permission_state: crate::shared_types::PermissionState::NotRequested,
            }
        }
    }
    
    #[cfg(debug_assertions)]
    pub fn get_debug_audio_devices(&self) -> Option<AudioDevices> {
        self.audio_context.as_ref().map(|ctx| {
            match ctx.try_borrow() {
                Ok(borrowed) => borrowed.get_audio_devices(),
                Err(_) => AudioDevices { input_devices: Vec::new(), output_devices: Vec::new() }
            }
        })
    }

    #[cfg(debug_assertions)]
    pub fn get_debug_audioworklet_status(&self) -> Option<AudioWorkletStatus> {
        self.audio_context.as_ref().and_then(|ctx| {
            match ctx.try_borrow() {
                Ok(borrowed) => borrowed.get_audioworklet_status(),
                Err(_) => None
            }
        })
    }

    #[cfg(debug_assertions)]
    pub fn get_debug_buffer_pool_stats(&self) -> Option<BufferPoolStats> {
        self.audio_context.as_ref().and_then(|ctx| {
            match ctx.try_borrow() {
                Ok(borrowed) => borrowed.get_buffer_pool_stats(),
                Err(_) => None
            }
        })
    }
    
    
    /// Get the audio context for async operations
    /// 
    /// Returns a clone of the Rc<RefCell<AudioSystemContext>> if available.
    /// This is used for async operations that need access to the audio context
    /// outside of the main engine instance.
    pub fn get_audio_context(&self) -> Option<std::rc::Rc<std::cell::RefCell<audio::AudioSystemContext>>> {
        self.audio_context.clone()
    }
    
    /// Connect an existing MediaStream to the audio processing pipeline
    /// 
    /// This method accepts a MediaStream that was obtained through user gesture
    /// and connects it directly to the AudioWorklet for processing.
    /// 
    /// # Arguments
    /// 
    /// * `media_stream` - The MediaStream to connect (should contain audio tracks)
    /// 
    /// # Returns
    /// 
    /// Returns `Result<(), String>` indicating success or failure of the connection.
    pub async fn connect_mediastream(&self, media_stream: web_sys::MediaStream) -> Result<(), String> {
        if let Some(ref audio_context) = self.audio_context {
            audio::microphone::connect_existing_mediastream_to_audioworklet(media_stream, audio_context).await
        } else {
            Err("Audio system not initialized".to_string())
        }
    }
    
    /// Execute model layer actions and return executed actions for logging/feedback
    /// 
    /// This method receives validated actions from the model layer, transforms them
    /// into engine-specific execution actions, performs the actual execution using
    /// existing audio system functionality, and returns the executed actions for
    /// logging and feedback purposes.
    /// 
    /// # Arguments
    /// 
    /// * `model_actions` - Validated actions from the model layer to execute
    /// 
    /// # Returns
    /// 
    /// Returns `Result<EngineLayerActions, String>` containing either the successfully
    /// executed actions or an error message if execution failed.
    /// 
    /// # Execution Process
    /// 
    /// 1. Transforms model actions into engine execution actions
    /// 2. Executes each action type using existing engine functionality:
    ///    - Microphone permission requests use `connect_microphone_to_audioworklet_with_context()`
    ///    - Audio system configurations apply tuning system settings
    ///    - Tuning configurations apply root note settings
    /// 3. Collects executed actions for logging and feedback
    /// 4. Provides comprehensive error handling with detailed logging
    pub fn execute_actions(&mut self, model_actions: ModelLayerActions) -> Result<(), String> {
        self.log_execution_start(&model_actions);
        
        let engine_actions = EngineLayerActions::new();
        
        // Temporarily disabled as tuning system handling is being moved to model layer
        // self.execute_audio_system_configuration_actions_sync(&model_actions, &mut engine_actions)?;
        // self.execute_tuning_configuration_actions_sync(&model_actions, &mut engine_actions)?;
        
        crate::common::dev_log!("PLACEHOLDER: Model actions execution disabled during engine layer refactoring");
        
        self.log_execution_completion(&engine_actions);
        
        Ok(())
    }
    
    /// Log the start of action execution with count information
    /// 
    /// This helper method logs the beginning of action execution, providing
    /// visibility into the number of actions being processed.
    /// 
    /// # Arguments
    /// 
    /// * `model_actions` - The model layer actions to be executed
    fn log_execution_start(&self, model_actions: &ModelLayerActions) {
        // Temporarily disabled as tuning system handling is being moved to model layer
        // let total_actions = model_actions.audio_system_configurations.len() + 
        //                   model_actions.tuning_configurations.len();
        let total_actions = 0; // Placeholder
        
        crate::common::dev_log!("Engine layer executing {} model actions", total_actions);
    }
    
    /// Log the completion of action execution with result information
    /// 
    /// This helper method logs the successful completion of action execution,
    /// providing visibility into the number of actions that were executed.
    /// 
    /// # Arguments
    /// 
    /// * `engine_actions` - The successfully executed engine layer actions
    fn log_execution_completion(&self, engine_actions: &EngineLayerActions) {
        // Temporarily disabled as tuning system handling is being moved to model layer
        // let total_executed = engine_actions.audio_system_configurations.len() + 
        //                    engine_actions.tuning_configurations.len();
        let total_executed = 0; // Placeholder
        
        crate::common::dev_log!("✓ Engine layer successfully executed {} total actions", total_executed);
    }
    
    // Temporarily disabled as tuning system handling is being moved to model layer
    // 
    // /// Execute audio system configurations synchronously
    // /// 
    // /// Synchronous version of execute_audio_system_configurations for use in the render loop.
    // fn execute_audio_system_configuration_actions_sync(
    //     &self,
    //     model_actions: &ModelLayerActions,
    //     engine_actions: &mut EngineLayerActions
    // ) -> Result<(), String> {
    //     for config in &model_actions.audio_system_configurations {
    //         let engine_config = ConfigureAudioSystem {
    //             tuning_system: config.tuning_system.clone(),
    //         };
    //         
    //         crate::common::dev_log!("Configuring audio system with tuning: {:?}", 
    //             engine_config.tuning_system);
    //         
    //         // Placeholder implementation - always succeeds with inline configuration
    //         if let Some(ref audio_context) = self.audio_context {
    //             // Placeholder implementation - always succeeds
    //             crate::common::dev_log!("PLACEHOLDER: Configuring audio worklet with tuning system {:?}",
    //                 engine_config.tuning_system);
    //             
    //             engine_actions.audio_system_configurations.push(engine_config);
    //             crate::common::dev_log!("✓ Audio system configuration executed successfully");
    //         } else {
    //             crate::common::dev_log!("✗ No audio context available for audio system configuration");
    //             return Err("Audio system not initialized".to_string());
    //         }
    //     }
    //     
    //     Ok(())
    // }
    // 
    // /// Execute tuning configurations synchronously
    // /// 
    // /// Synchronous version of execute_tuning_configurations for use in the render loop.
    // fn execute_tuning_configuration_actions_sync(
    //     &self,
    //     model_actions: &ModelLayerActions,
    //     engine_actions: &mut EngineLayerActions
    // ) -> Result<(), String> {
    //     for config in &model_actions.tuning_configurations {
    //         let engine_config = UpdateTuningConfiguration {
    //             tuning_system: config.tuning_system.clone(),
    //             root_note: config.root_note.clone(),
    //         };
    //         
    //         crate::common::dev_log!("Updating tuning configuration - tuning: {:?}, root note: {:?}", 
    //             engine_config.tuning_system, engine_config.root_note);
    //         
    //         // Placeholder implementation - always succeeds with inline tuning update
    //         if let Some(ref _audio_context) = self.audio_context {
    //             // Placeholder implementation - always succeeds
    //             crate::common::dev_log!("PLACEHOLDER: Updating audio worklet tuning - system: {:?}, root note: {:?}",
    //                 engine_config.tuning_system, engine_config.root_note);
    //             
    //             engine_actions.tuning_configurations.push(engine_config);
    //             crate::common::dev_log!("✓ Tuning configuration executed successfully");
    //         } else {
    //             crate::common::dev_log!("✗ No audio context available for tuning configuration");
    //             return Err("Audio system not initialized".to_string());
    //         }
    //     }
    //     
    //     Ok(())
    // }
    
    
    
    
    /// Execute debug actions with privileged engine access (debug builds only)
    /// 
    /// This method processes debug actions from the presentation layer that provide
    /// direct, privileged access to engine operations. These actions bypass normal
    /// validation and safety checks and should only be used for testing and debugging.
    /// 
    /// # Arguments
    /// 
    /// * `debug_actions` - Debug actions from the presentation layer to execute
    /// 
    /// # Returns
    /// 
    /// Returns `Result<DebugEngineActions, String>` containing either the successfully
    /// executed debug actions or an error message if execution failed.
    /// 
    /// # Safety
    /// 
    /// Debug actions provide direct access to engine internals and bypass normal
    /// safety checks. They should only be used in debug builds for testing purposes.
    /// 
    /// # Privileged Operations
    /// 
    /// - Test signal generation: Direct control over audio worklet test signals
    /// - Speaker output: Direct manipulation of speaker output routing
    /// - Background noise: Direct injection of noise into the audio pipeline
    #[cfg(debug_assertions)]
    pub fn execute_debug_actions_sync(&mut self, debug_actions: DebugLayerActions) -> Result<DebugEngineActions, String> {
        crate::common::dev_log!("[DEBUG] Engine layer executing debug actions");
        
        let mut debug_engine_actions = DebugEngineActions::new();
        
        // Execute test signal configurations with privileged access
        self.execute_test_signal_configurations(
            &debug_actions.test_signal_configurations,
            &mut debug_engine_actions
        )?;
        
        // Execute speaker output configurations with privileged access
        self.execute_speaker_output_configurations(
            &debug_actions.speaker_output_configurations,
            &mut debug_engine_actions
        )?;
        
        // Execute background noise configurations with privileged access
        self.execute_background_noise_configurations(
            &debug_actions.background_noise_configurations,
            &mut debug_engine_actions
        )?;
        
        let total_executed = debug_engine_actions.test_signal_executions.len() + 
                           debug_engine_actions.speaker_output_executions.len() + 
                           debug_engine_actions.background_noise_executions.len();
        
        crate::common::dev_log!("[DEBUG] ✓ Engine layer successfully executed {} debug actions", total_executed);
        
        Ok(debug_engine_actions)
    }
    
    /// Execute debug actions asynchronously (async wrapper for compatibility)
    ///
    /// This method provides backward compatibility for async callers while internally
    /// using the synchronous implementation since debug actions don't require async operations.
    ///
    /// # Arguments
    ///
    /// * `debug_actions` - Debug actions to execute
    ///
    /// # Returns
    ///
    /// Returns `Result<DebugEngineActions, String>` containing the executed debug actions
    /// or an error message if execution failed.
    #[cfg(debug_assertions)]
    pub async fn execute_debug_actions(&mut self, debug_actions: DebugLayerActions) -> Result<DebugEngineActions, String> {
        self.execute_debug_actions_sync(debug_actions)
    }
    
    /// Execute test signal configurations with privileged engine access (debug builds only)
    /// 
    /// This method provides direct control over test signal generation in the audio
    /// worklet, bypassing normal validation checks.
    /// 
    /// # Arguments
    /// 
    /// * `test_signal_configs` - Test signal configurations to execute
    /// * `debug_engine_actions` - Container to store executed actions
    /// 
    /// # Returns
    /// 
    /// Returns `Result<(), String>` indicating success or failure
    #[cfg(debug_assertions)]
    fn execute_test_signal_configurations(
        &self,
        test_signal_configs: &[ConfigureTestSignal],
        debug_engine_actions: &mut DebugEngineActions
    ) -> Result<(), String> {
        for config in test_signal_configs {
            crate::common::dev_log!(
                "[DEBUG] Executing privileged test signal configuration - enabled: {}, freq: {} Hz, vol: {}%, waveform: {:?}",
                config.enabled, config.frequency, config.volume, config.waveform
            );
            
            // Direct privileged access to test signal generation
            if let Some(ref audio_context) = self.audio_context {
                let mut borrowed_context = audio_context.borrow_mut();
                if let Some(worklet_manager) = borrowed_context.get_audioworklet_manager_mut() {
                    // Convert debug action to audio system config
                    let audio_config = crate::engine::audio::SignalGeneratorConfig {
                        enabled: config.enabled,
                        frequency: config.frequency,
                        amplitude: config.volume / 100.0, // Convert percentage to 0-1 range
                        waveform: config.waveform.clone(),
                        sample_rate: 48000.0, // Use standard sample rate
                    };
                    
                    worklet_manager.update_test_signal_config(audio_config);
                    crate::common::dev_log!(
                        "[DEBUG] ✓ Test signal control updated - enabled: {}, freq: {}, vol: {}%", 
                        config.enabled, config.frequency, config.volume
                    );
                } else {
                    crate::common::dev_log!(
                        "[DEBUG] ⚠ AudioWorkletManager not available for test signal control"
                    );
                }
                
                // Record the executed action
                debug_engine_actions.test_signal_executions.push(ExecuteTestSignalConfiguration {
                    enabled: config.enabled,
                    frequency: config.frequency,
                    volume: config.volume,
                    waveform: config.waveform.clone(),
                });
            } else {
                return Err("[DEBUG] Audio context not available for test signal execution".to_string());
            }
        }
        
        crate::common::dev_log!(
            "[DEBUG] ✓ Executed {} test signal configurations with privileged access",
            test_signal_configs.len()
        );
        Ok(())
    }
    
    /// Execute speaker output configurations with privileged engine access (debug builds only)
    /// 
    /// This method provides direct control over speaker output routing, bypassing
    /// normal permission checks and safety validations.
    /// 
    /// # Arguments
    /// 
    /// * `speaker_configs` - Speaker output configurations to execute
    /// * `debug_engine_actions` - Container to store executed actions
    /// 
    /// # Returns
    /// 
    /// Returns `Result<(), String>` indicating success or failure
    #[cfg(debug_assertions)]
    fn execute_speaker_output_configurations(
        &self,
        speaker_configs: &[ConfigureOutputToSpeakers],
        debug_engine_actions: &mut DebugEngineActions
    ) -> Result<(), String> {
        for config in speaker_configs {
            crate::common::dev_log!(
                "[DEBUG] Executing privileged speaker output configuration - enabled: {}",
                config.enabled
            );
            
            // Direct privileged access to speaker output control
            if let Some(ref audio_context) = self.audio_context {
                let mut borrowed_context = audio_context.borrow_mut();
                if let Some(worklet_manager) = borrowed_context.get_audioworklet_manager_mut() {
                    worklet_manager.set_output_to_speakers(config.enabled);
                    crate::common::dev_log!(
                        "[DEBUG] ✓ Speaker output control updated - enabled: {}", 
                        config.enabled
                    );
                } else {
                    crate::common::dev_log!(
                        "[DEBUG] ⚠ AudioWorkletManager not available for speaker output control"
                    );
                }
                
                // Record the executed action
                debug_engine_actions.speaker_output_executions.push(ExecuteOutputToSpeakersConfiguration {
                    enabled: config.enabled,
                });
            } else {
                return Err("[DEBUG] Audio context not available for speaker output execution".to_string());
            }
        }
        
        crate::common::dev_log!(
            "[DEBUG] ✓ Executed {} speaker output configurations with privileged access",
            speaker_configs.len()
        );
        Ok(())
    }
    
    /// Execute background noise configurations with privileged engine access (debug builds only)
    /// 
    /// This method provides direct control over background noise generation in the
    /// audio pipeline, useful for testing noise cancellation and signal processing.
    /// 
    /// # Arguments
    /// 
    /// * `noise_configs` - Background noise configurations to execute
    /// * `debug_engine_actions` - Container to store executed actions
    /// 
    /// # Returns
    /// 
    /// Returns `Result<(), String>` indicating success or failure
    #[cfg(debug_assertions)]
    fn execute_background_noise_configurations(
        &self,
        noise_configs: &[ConfigureBackgroundNoise],
        debug_engine_actions: &mut DebugEngineActions
    ) -> Result<(), String> {
        for config in noise_configs {
            crate::common::dev_log!(
                "[DEBUG] Executing privileged background noise configuration - enabled: {}, level: {}, type: {:?}",
                config.enabled, config.level, config.noise_type
            );
            
            // Direct privileged access to background noise generation
            if let Some(ref audio_context) = self.audio_context {
                let mut borrowed_context = audio_context.borrow_mut();
                if let Some(worklet_manager) = borrowed_context.get_audioworklet_manager_mut() {
                    // Convert debug action to audio system config
                    let audio_config = crate::engine::audio::BackgroundNoiseConfig {
                        enabled: config.enabled,
                        level: config.level,
                        noise_type: config.noise_type.clone(),
                    };
                    
                    worklet_manager.update_background_noise_config(audio_config);
                    crate::common::dev_log!(
                        "[DEBUG] ✓ Background noise control updated - enabled: {}, level: {}", 
                        config.enabled, config.level
                    );
                } else {
                    crate::common::dev_log!(
                        "[DEBUG] ⚠ AudioWorkletManager not available for background noise control"
                    );
                }
                
                // Record the executed action
                debug_engine_actions.background_noise_executions.push(ExecuteBackgroundNoiseConfiguration {
                    enabled: config.enabled,
                    level: config.level,
                    noise_type: config.noise_type.clone(),
                });
            } else {
                return Err("[DEBUG] Audio context not available for background noise execution".to_string());
            }
        }
        
        crate::common::dev_log!(
            "[DEBUG] ✓ Executed {} background noise configurations with privileged access",
            noise_configs.len()
        );
        Ok(())
    }
}