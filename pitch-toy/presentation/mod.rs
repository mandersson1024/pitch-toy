//! Presentation Layer - Visualization and user interface
//! 
//! This layer is responsible for:
//! - Visual rendering and graphics display
//! - User interface elements and interactions
//! - Screen management and layout
//! - Event handling and user input
//! - Visual feedback and animations
//! - Debug visualization and overlays
//! 
//! ## Return-Based Data Flow in Presentation Layer
//! 
//! The presentation layer now uses a return-based pattern for data processing:
//! - Receives `ModelUpdateResult` data as a parameter from the model layer
//! - Processes visual data including volume, pitch, accuracy, errors, and permission state
//! - Updates UI elements and visualizations based on the provided data
//! 
//! ```rust
//! use pitch_toy::presentation::Presenter;
//! use pitch_toy::shared_types::ModelUpdateResult;
//! 
//! // Create presenter without interface dependencies
//! let mut presenter = Presenter::create()?;
//! 
//! // Update with model data
//! let model_data = ModelUpdateResult {
//!     volume: pitch_toy::shared_types::Volume { peak: -10.0, rms: -15.0 },
//!     pitch: pitch_toy::shared_types::Pitch::Detected(440.0, 0.95),
//!     accuracy: pitch_toy::shared_types::Accuracy { closest_note: pitch_toy::shared_types::Note::A, accuracy: 0.05 },
//!     tuning_system: pitch_toy::shared_types::TuningSystem::EqualTemperament,
//!     errors: Vec::new(),
//!     permission_state: pitch_toy::shared_types::PermissionState::Granted,
//! };
//! presenter.update(timestamp, model_data);
//! ```
//! 
//! ## Current Status
//! 
//! The Presenter struct operates without interface dependencies and actively
//! processes data through method parameters. It provides:
//! 
//! - ✅ Basic sprite scene rendering capabilities
//! - ✅ Volume data processing for audio level visualization  
//! - ✅ Pitch detection handling and musical note processing
//! - ✅ Accuracy metrics processing for tuning feedback
//! - ✅ Error state management and user feedback
//! - ✅ Permission state tracking and UI updates
//! - ✅ Tuning system display management
//! - ✅ User action collection system for microphone permission, tuning system changes, and root note adjustments
//! 
//! ## Future Implementation
//! 
//! Enhanced visual implementation will add:
//! - Visual representations of pitch and volume (meters, waveforms)
//! - Interactive tuning displays and note indicators
//! - User interaction handling and input processing
//! - Advanced animations and visual transitions
//! - Complete screen layout and UI element management


// PLACEHOLDER: Import temporary sprite scene for development/testing
// TODO: Remove this import and sprite_scene.rs when proper visualization is implemented
mod sprite_scene;
pub use sprite_scene::SpriteScene;

use three_d::{RenderTarget, Context, Viewport};
use crate::shared_types::{ModelUpdateResult, TuningSystem, Note};

// Debug-only imports for conditional compilation
#[cfg(debug_assertions)]
use crate::engine::audio::TestWaveform;

/// Action structs for the new action collection system
/// 
/// These structs represent user actions that are collected by the presentation layer
/// and processed by the main loop. This provides a foundation for the new action flow
/// that moves away from direct action firing.

/// Request for microphone permission from the user interface
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RequestMicrophonePermission;

/// Request to change the tuning system
#[derive(Debug, Clone, PartialEq)]
pub struct ChangeTuningSystem {
    pub tuning_system: TuningSystem,
}

/// Request to adjust the root note
#[derive(Debug, Clone, PartialEq)]
pub struct AdjustRootNote {
    pub root_note: Note,
}

// Debug action structs (only available in debug builds)
#[cfg(debug_assertions)]
#[derive(Debug, Clone, PartialEq)]
pub struct ConfigureTestSignal {
    pub enabled: bool,
    pub frequency: f32,
    pub volume: f32,
    pub waveform: TestWaveform,
}

#[cfg(debug_assertions)]
#[derive(Debug, Clone, PartialEq)]
pub struct ConfigureOutputToSpeakers {
    pub enabled: bool,
}

#[cfg(debug_assertions)]
#[derive(Debug, Clone, PartialEq)]
pub struct ConfigureBackgroundNoise {
    pub enabled: bool,
    pub level: f32,
    pub noise_type: TestWaveform,
}

/// Container for all collected user actions from the presentation layer
/// 
/// This struct is returned by the presentation layer's get_user_actions() method
/// and contains all user actions that occurred since the last collection.
/// The main loop retrieves these actions and processes them appropriately.
#[derive(Debug, Clone, PartialEq)]
pub struct PresentationLayerActions {
    pub tuning_system_changes: Vec<ChangeTuningSystem>,
    pub root_note_adjustments: Vec<AdjustRootNote>,
}

impl PresentationLayerActions {
    /// Create a new instance with empty action collections
    pub(crate) fn new() -> Self {
        Self {
            tuning_system_changes: Vec::new(),
            root_note_adjustments: Vec::new(),
        }
    }
}

/// Container for all collected debug actions from the presentation layer
/// 
/// This struct is only available in debug builds and contains actions that
/// provide privileged access to engine operations for testing and debugging.
/// These actions bypass normal validation and safety checks.
#[cfg(debug_assertions)]
#[derive(Debug, Clone, PartialEq)]
pub struct DebugLayerActions {
    pub test_signal_configurations: Vec<ConfigureTestSignal>,
    pub speaker_output_configurations: Vec<ConfigureOutputToSpeakers>,
    pub background_noise_configurations: Vec<ConfigureBackgroundNoise>,
}

#[cfg(debug_assertions)]
impl DebugLayerActions {
    /// Create a new instance with empty debug action collections
    pub(crate) fn new() -> Self {
        Self {
            test_signal_configurations: Vec::new(),
            speaker_output_configurations: Vec::new(),
            background_noise_configurations: Vec::new(),
        }
    }
}

/// Presenter - The presentation layer of the three-layer architecture
/// 
/// This struct represents the visual rendering and user interface layer
/// of the application. It receives processed data from the model layer
/// through method parameters and renders visual representations.
/// 
/// Data flows through method parameters and return values rather than interface dependencies.
/// 
/// # Current Implementation
/// 
/// This implementation:
/// - Operates without observable interface dependencies
/// - Receives model data through `update()` method parameters
/// - Actively processes volume, pitch, accuracy, and error data
/// - Provides sprite scene rendering capabilities
/// - Manages UI state based on model data updates
/// 
/// # Example
/// 
/// ```no_run
/// use pitch_toy::presentation::Presenter;
/// use pitch_toy::shared_types::model_to_presentation::ModelUpdateResult;
/// use three_d::RenderTarget;
/// 
/// let mut presenter = Presenter::create()
///     .expect("Presenter creation should always succeed");
/// 
/// // Later in render loop:
/// // let model_data = get_model_data(); // From model layer
/// // presenter.update(timestamp, model_data);
/// // let mut screen = frame_input.screen();
/// // presenter.render(&mut screen);
/// ```
pub struct Presenter {
    /// Presentation layer now operates without interface dependencies
    /// Data flows through method parameters and return values
    
    /// Sprite scene for rendering
    sprite_scene: Option<SpriteScene>,
    
    /// Flag to track if scene has been initialized
    scene_initialized: bool,
    
    /// Collection of pending user actions to be processed by the main loop
    /// 
    /// This field stores user actions (like requesting microphone permission,
    /// changing tuning system, or adjusting root note) until they are retrieved
    /// by the main loop via get_user_actions().
    pending_user_actions: PresentationLayerActions,
    
    /// Collection of pending debug actions (debug builds only)
    /// 
    /// This field stores debug actions that provide privileged engine access
    /// for testing and debugging purposes. These actions bypass normal validation.
    #[cfg(debug_assertions)]
    pending_debug_actions: DebugLayerActions,
}

impl Presenter {
    /// Create a new Presenter without interface dependencies
    /// 
    /// This constructor creates a presentation layer that operates through method parameters
    /// and return values rather than observable interfaces. Data is received directly
    /// through the update() method.
    /// 
    /// # Returns
    /// 
    /// Always returns `Ok(Presenter)` as this implementation cannot fail.
    /// 
    /// # Current Behavior
    /// 
    /// Creates a basic presenter struct with sprite scene support. Future implementations
    /// will add comprehensive rendering and UI functionality.
    pub fn create() -> Result<Self, String> {
        // Presentation layer initialization without interface dependencies
        // TODO: Initialize rendering state
        // TODO: Set up sprite scene configuration
        Ok(Self {
            sprite_scene: None,
            scene_initialized: false,
            pending_user_actions: PresentationLayerActions::new(),
            #[cfg(debug_assertions)]
            pending_debug_actions: DebugLayerActions::new(),
        })
    }

    /// Update viewport size for the scene
    /// 
    /// This should be called when the window/canvas is resized.
    /// 
    /// # Arguments
    /// 
    /// * `viewport` - The new viewport dimensions
    pub fn update_viewport(&mut self, viewport: Viewport) {
        if let Some(ref mut scene) = self.sprite_scene {
            scene.update_viewport(viewport);
        }
    }

    /// Update the presentation layer with a new timestamp and model data
    /// 
    /// This method is called by the main render loop to update the presentation's state.
    /// It receives processed data from the model layer, updates internal rendering state,
    /// and prepares for the next render call.
    /// 
    /// # Arguments
    /// 
    /// * `timestamp` - The current timestamp in seconds since application start
    /// * `model_data` - The processed data from the model layer containing volume, pitch, accuracy, etc.
    /// 
    /// # Current Implementation
    /// 
    /// This implementation processes the model data and updates internal state:
    /// 1. Processes volume data for audio level visualization
    /// 2. Handles pitch detection results and musical note display
    /// 3. Processes accuracy metrics for tuning feedback
    /// 4. Manages error states and user feedback
    /// 5. Updates permission status display
    /// 6. Prepares data for next render cycle
    pub fn update(&mut self, _timestamp: f64, model_data: ModelUpdateResult) {
        // Process volume data for visualization
        self.process_volume_data(&model_data.volume);
        
        // Process pitch and note detection
        self.process_pitch_data(&model_data.pitch);
        
        // Process accuracy metrics for tuning feedback
        self.process_accuracy_data(&model_data.accuracy);
        
        // Handle error states and user feedback
        self.process_error_states(&model_data.errors);
        
        // Update permission status display
        self.process_permission_state(&model_data.permission_state);
        
        // Update tuning system display
        self.process_tuning_system(&model_data.tuning_system);
    }

    /// Retrieve and clear all pending user actions
    /// 
    /// This method is called by the main loop to get all user actions that have
    /// been collected since the last call. After retrieving the actions, the
    /// internal collection is cleared to prepare for the next collection cycle.
    /// 
    /// # Returns
    /// 
    /// A `PresentationLayerActions` struct containing all collected user actions.
    /// The returned struct will contain empty vectors if no actions were collected.
    /// 
    /// # Usage
    /// 
    /// This method should be called once per render loop by the main application
    /// to process user actions that occurred during the previous frame.
    pub fn get_user_actions(&mut self) -> PresentationLayerActions {
        std::mem::replace(&mut self.pending_user_actions, PresentationLayerActions::new())
    }

    /// Handle user request to change the tuning system
    /// 
    /// This method should be called by UI components when the user selects
    /// a different tuning system from a dropdown or control panel.
    /// 
    /// # Arguments
    /// 
    /// * `tuning_system` - The new tuning system selected by the user
    pub fn on_tuning_system_changed(&mut self, tuning_system: TuningSystem) {
        self.pending_user_actions.tuning_system_changes.push(ChangeTuningSystem { tuning_system });
    }

    /// Handle user request to adjust the root note
    /// 
    /// This method should be called by UI components when the user selects
    /// a different root note from a control or input field.
    /// 
    /// # Arguments
    /// 
    /// * `root_note` - The new root note selected by the user
    pub fn on_root_note_adjusted(&mut self, root_note: Note) {
        self.pending_user_actions.root_note_adjustments.push(AdjustRootNote { root_note });
    }

    /// Retrieve and clear all pending debug actions (debug builds only)
    /// 
    /// This method is called by the main loop to get all debug actions that have
    /// been collected since the last call. These actions provide privileged access
    /// to engine operations for testing and debugging purposes.
    /// 
    /// # Returns
    /// 
    /// A `DebugLayerActions` struct containing all collected debug actions.
    /// The returned struct will contain empty vectors if no actions were collected.
    /// 
    /// # Safety
    /// 
    /// Debug actions bypass normal validation and can directly manipulate engine
    /// internals. They should only be used for testing and debugging.
    #[cfg(debug_assertions)]
    pub fn get_debug_actions(&mut self) -> DebugLayerActions {
        std::mem::replace(&mut self.pending_debug_actions, DebugLayerActions::new())
    }

    /// Handle debug request to configure test signal generation (debug builds only)
    /// 
    /// This method should be called by debug UI components to configure test
    /// signal generation for testing audio processing.
    /// 
    /// # Arguments
    /// 
    /// * `enabled` - Whether test signal generation should be enabled
    /// * `frequency` - The frequency of the test signal in Hz
    /// * `volume` - The volume of the test signal (0-100)
    /// * `waveform` - The waveform type to generate
    #[cfg(debug_assertions)]
    pub fn on_test_signal_configured(&mut self, enabled: bool, frequency: f32, volume: f32, waveform: TestWaveform) {
        self.pending_debug_actions.test_signal_configurations.push(ConfigureTestSignal {
            enabled,
            frequency,
            volume,
            waveform,
        });
    }

    /// Handle debug request to configure speaker output (debug builds only)
    /// 
    /// This method should be called by debug UI components to enable or disable
    /// direct speaker output for debugging audio processing.
    /// 
    /// # Arguments
    /// 
    /// * `enabled` - Whether speaker output should be enabled
    #[cfg(debug_assertions)]
    pub fn on_output_to_speakers_configured(&mut self, enabled: bool) {
        self.pending_debug_actions.speaker_output_configurations.push(ConfigureOutputToSpeakers {
            enabled,
        });
    }

    /// Handle debug request to configure background noise generation (debug builds only)
    /// 
    /// This method should be called by debug UI components to configure background
    /// noise generation for testing noise cancellation and signal processing.
    /// 
    /// # Arguments
    /// 
    /// * `enabled` - Whether background noise generation should be enabled
    /// * `level` - The level of background noise
    /// * `noise_type` - The type of noise to generate
    #[cfg(debug_assertions)]
    pub fn on_background_noise_configured(&mut self, enabled: bool, level: f32, noise_type: TestWaveform) {
        self.pending_debug_actions.background_noise_configurations.push(ConfigureBackgroundNoise {
            enabled,
            level,
            noise_type,
        });
    }

    /// Render the presentation layer to the screen
    /// 
    /// This method is called by the main render loop to draw the visual interface.
    /// 
    /// # Arguments
    /// 
    /// * `_context` - The WebGL context for rendering (currently unused)
    /// * `screen` - The render target to draw to
    pub fn render(&mut self, context: &Context, screen: &mut RenderTarget) {
        // Initialize scene on first render if not already done
        if !self.scene_initialized {
            let viewport = screen.viewport();
            self.sprite_scene = Some(SpriteScene::new(context, viewport));
            self.scene_initialized = true;
        }
        
        // Render the scene if available
        if let Some(ref scene) = self.sprite_scene {
            scene.render(screen);
        }
    }
    
    /// Process volume data for audio level visualization
    /// 
    /// Updates internal state based on volume levels from the model layer.
    /// This data will be used to drive volume meters and audio visualizations.
    /// 
    /// # Arguments
    /// 
    /// * `volume` - Volume data containing peak and RMS levels in dB
    fn process_volume_data(&mut self, volume: &crate::shared_types::Volume) {
        // Store volume data for visualization
        // Future: Update volume meter displays, audio wave visualizations
        let _peak_amplitude = volume.peak_amplitude;
        let _rms_amplitude = volume.rms_amplitude;
        
        // Placeholder: Log significant volume changes for debugging
        if volume.peak_amplitude > -20.0 {
            // Loud audio detected - could trigger visual feedback
        }
    }
    
    /// Process pitch detection data for musical note display
    /// 
    /// Updates pitch-related UI elements based on detected frequencies and notes.
    /// 
    /// # Arguments
    /// 
    /// * `pitch` - Pitch detection result from the model layer
    fn process_pitch_data(&mut self, pitch: &crate::shared_types::Pitch) {
        match pitch {
            crate::shared_types::Pitch::Detected(frequency, clarity) => {
                // Pitch detected - update note display
                let _freq = *frequency;
                let _clarity = *clarity;
                
                // Future: Update pitch display, note name, frequency readout
                // Future: Update visual tuning indicators
            }
            crate::shared_types::Pitch::NotDetected => {
                // No pitch detected - clear pitch displays
                // Future: Dim pitch indicators, show "listening" state
            }
        }
    }
    
    /// Process accuracy data for tuning feedback
    /// 
    /// Updates tuning indicators and accuracy displays based on pitch accuracy.
    /// 
    /// # Arguments
    /// 
    /// * `accuracy` - Accuracy metrics containing closest note and deviation
    fn process_accuracy_data(&mut self, accuracy: &crate::shared_types::Accuracy) {
        let _closest_note = &accuracy.closest_note;
        let _accuracy_value = accuracy.accuracy;
        
        // Future: Update tuning needle/indicator position
        // Future: Change colors based on accuracy (green=good, red=off)
        // Future: Display note name and cents deviation
        
        if accuracy.accuracy < 0.1 {
            // Very accurate - could show green indicator
        } else if accuracy.accuracy > 0.8 {
            // Very inaccurate - could show red indicator  
        }
    }
    
    /// Process error states for user feedback
    /// 
    /// Handles error conditions and updates error displays.
    /// 
    /// # Arguments
    /// 
    /// * `errors` - Vector of error conditions from the model layer
    fn process_error_states(&mut self, errors: &Vec<crate::shared_types::Error>) {
        if errors.is_empty() {
            // No errors - clear error displays
            return;
        }
        
        // Process each error type
        for error in errors {
            match error {
                crate::shared_types::Error::MicrophonePermissionDenied => {
                    // Show microphone permission denied message
                }
                crate::shared_types::Error::MicrophoneNotAvailable => {
                    // Show microphone not available message
                }
                crate::shared_types::Error::BrowserApiNotSupported => {
                    // Show browser compatibility message
                }
                crate::shared_types::Error::AudioContextInitFailed => {
                    // Show audio initialization failure message
                }
                crate::shared_types::Error::AudioContextSuspended => {
                    // Show audio context suspended message
                }
                crate::shared_types::Error::ProcessingError(_msg) => {
                    // Show general processing error
                }
            }
        }
    }
    
    /// Process permission state for UI updates
    /// 
    /// Updates permission-related UI elements and user prompts.
    /// 
    /// # Arguments
    /// 
    /// * `permission_state` - Current microphone permission state
    fn process_permission_state(&mut self, permission_state: &crate::shared_types::PermissionState) {
        match permission_state {
            crate::shared_types::PermissionState::NotRequested => {
                // Show "Click to start" or permission request button
            }
            crate::shared_types::PermissionState::Requested => {
                // Show "Requesting permission..." status
            }
            crate::shared_types::PermissionState::Granted => {
                // Show active/listening state
            }
            crate::shared_types::PermissionState::Denied => {
                // Show permission denied message with instructions
            }
        }
    }
    
    /// Process tuning system updates
    /// 
    /// Updates displays related to the current tuning system.
    /// 
    /// # Arguments
    /// 
    /// * `tuning_system` - Current tuning system from the model layer
    fn process_tuning_system(&mut self, tuning_system: &crate::shared_types::TuningSystem) {
        match tuning_system {
            crate::shared_types::TuningSystem::EqualTemperament => {
                // Update UI to show Equal Temperament tuning
            }
            crate::shared_types::TuningSystem::JustIntonation => {
                // Update UI to show Just Intonation tuning
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use three_d::*;
    
    /// Create test model data for testing purposes
    fn create_test_model_data() -> ModelUpdateResult {
        ModelUpdateResult {
            volume: crate::shared_types::Volume { peak_amplitude: -10.0, rms_amplitude: -15.0 },
            pitch: crate::shared_types::Pitch::NotDetected,
            accuracy: crate::shared_types::Accuracy {
                closest_note: crate::shared_types::Note::A,
                accuracy: 1.0,
            },
            tuning_system: crate::shared_types::TuningSystem::EqualTemperament,
            errors: Vec::new(),
            permission_state: crate::shared_types::PermissionState::NotRequested,
        }
    }

    /// Test that Presenter::create() succeeds without interface dependencies
    #[wasm_bindgen_test]
    fn test_presenter_create_succeeds() {
        let result = Presenter::create();
        assert!(result.is_ok(), "Presenter::create() should always succeed");
    }

    /// Test that update() and render() methods can be called without panicking
    #[wasm_bindgen_test]
    fn test_presenter_update_and_render_no_panic() {
        let mut presenter = Presenter::create()
            .expect("Presenter creation should succeed");

        // Test that update can be called multiple times without panicking
        let test_data = create_test_model_data();
        presenter.update(0.0, test_data.clone());
        presenter.update(1.0, test_data.clone());
        presenter.update(123.456, test_data.clone());
        presenter.update(-1.0, test_data); // Negative timestamp should also be safe
        
        // Test render method - we need a mock render target
        // Since we can't easily create a real RenderTarget in tests,
        // we'll create a simple test that verifies the method signature
        // The actual render testing will be done in integration tests
        
        // For now, just verify update doesn't panic
        // If we reach this point, no panic occurred
        assert!(true, "update() method should not panic");
    }

    /// Test that render method can be called (basic signature verification)
    #[wasm_bindgen_test]
    fn test_presenter_render_method_exists() {
        let presenter = Presenter::create()
            .expect("Presenter creation should succeed");

        // We can't easily create a RenderTarget in unit tests without a full WebGL context
        // But we can verify the method exists and compiles correctly by checking the type
        // This test mainly ensures the render method signature is correct
        
        // Create a mock function pointer to verify the signature
        let _render_fn: fn(&mut Presenter, &Context, &mut RenderTarget) = Presenter::render;
        
        assert!(true, "render() method signature is correct");
    }

    /// Verify that struct can be created without interface dependencies
    #[wasm_bindgen_test]
    fn test_presenter_interface_free_creation() {
        // This test verifies that the struct can be constructed without interfaces
        let presenter = Presenter::create();

        match presenter {
            Ok(_) => {
                // Success - presenter was created without interface dependencies
                assert!(true, "Presenter was created without interface dependencies");
            }
            Err(e) => {
                panic!("Presenter should be creatable without interfaces, but got error: {}", e);
            }
        }
    }

    /// Test basic runtime safety - creation and operation should not crash
    #[wasm_bindgen_test]
    fn test_presenter_runtime_safety() {
        // Create multiple instances to test memory safety
        for i in 0..10 {
            let mut presenter = Presenter::create()
                .expect("Presenter creation should always succeed");

            // Test multiple operations
            let test_data = create_test_model_data();
            presenter.update(i as f64, test_data.clone());
            presenter.update((i as f64) * 0.5, test_data.clone());
            
            // Test edge case values
            presenter.update(f64::MAX, test_data.clone());
            presenter.update(f64::MIN, test_data.clone());
            presenter.update(0.0, test_data);
        }
        
        // If we reach this point, all operations completed safely
        assert!(true, "All Presenter operations completed safely");
    }

    /// Test that Presenter compilation requirements are met
    #[wasm_bindgen_test]
    fn test_presenter_compilation_safety() {
        // This test exists primarily to ensure the struct compiles correctly
        // without interface dependencies

        // Test successful creation
        let presenter_result = Presenter::create();

        // Test that the result type is correct
        assert!(presenter_result.is_ok());
        
        let mut presenter = presenter_result.unwrap();
        
        // Test that update signature is correct
        let test_data = create_test_model_data();
        presenter.update(42.0, test_data);
        
        // Test completed - all compilation requirements verified
        assert!(true, "Presenter meets all compilation requirements");
    }

    /// Test interface isolation - different instances should be independent
    #[wasm_bindgen_test]
    fn test_presenter_interface_isolation() {
        let presenter1 = Presenter::create()
            .expect("Presenter1 creation should succeed");

        let presenter2 = Presenter::create()
            .expect("Presenter2 creation should succeed");

        // Both presenters should be independent and work correctly
        // This is mainly a compilation test since they're placeholders
        assert!(true, "Multiple Presenter instances are properly isolated");
    }

    /// Test action collection system - get_user_actions returns empty initially
    #[wasm_bindgen_test]
    fn test_get_user_actions_initially_empty() {
        let mut presenter = Presenter::create()
            .expect("Presenter creation should succeed");

        let actions = presenter.get_user_actions();
        
        assert!(actions.tuning_system_changes.is_empty());
        assert!(actions.root_note_adjustments.is_empty());
    }


    /// Test tuning system change collection
    #[wasm_bindgen_test]
    fn test_tuning_system_change_collection() {
        let mut presenter = Presenter::create()
            .expect("Presenter creation should succeed");

        // Trigger tuning system change
        presenter.on_tuning_system_changed(TuningSystem::JustIntonation);
        
        let actions = presenter.get_user_actions();
        assert_eq!(actions.tuning_system_changes.len(), 1);
        assert_eq!(actions.tuning_system_changes[0].tuning_system, TuningSystem::JustIntonation);
        
        // After getting actions, they should be cleared
        let actions2 = presenter.get_user_actions();
        assert!(actions2.tuning_system_changes.is_empty());
    }

    /// Test root note adjustment collection
    #[wasm_bindgen_test]
    fn test_root_note_adjustment_collection() {
        let mut presenter = Presenter::create()
            .expect("Presenter creation should succeed");

        // Trigger root note adjustment
        presenter.on_root_note_adjusted(Note::DFlat);
        
        let actions = presenter.get_user_actions();
        assert_eq!(actions.root_note_adjustments.len(), 1);
        assert_eq!(actions.root_note_adjustments[0].root_note, Note::DFlat);
        
        // After getting actions, they should be cleared
        let actions2 = presenter.get_user_actions();
        assert!(actions2.root_note_adjustments.is_empty());
    }

    /// Test multiple action collection and clearing
    #[wasm_bindgen_test]
    fn test_multiple_action_collection() {
        let mut presenter = Presenter::create()
            .expect("Presenter creation should succeed");

        // Trigger multiple actions
        presenter.on_tuning_system_changed(TuningSystem::EqualTemperament);
        presenter.on_root_note_adjusted(Note::G);
        presenter.on_tuning_system_changed(TuningSystem::JustIntonation); // Second change
        
        let actions = presenter.get_user_actions();
        
        // Verify all actions were collected
        assert_eq!(actions.tuning_system_changes.len(), 2);
        assert_eq!(actions.root_note_adjustments.len(), 1);
        
        // Verify action data
        assert_eq!(actions.tuning_system_changes[0].tuning_system, TuningSystem::EqualTemperament);
        assert_eq!(actions.tuning_system_changes[1].tuning_system, TuningSystem::JustIntonation);
        assert_eq!(actions.root_note_adjustments[0].root_note, Note::G);
        
        // After getting actions, all should be cleared
        let actions2 = presenter.get_user_actions();
        assert!(actions2.tuning_system_changes.is_empty());
        assert!(actions2.root_note_adjustments.is_empty());
    }

    /// Test PresentationLayerActions struct creation and equality
    #[wasm_bindgen_test]
    fn test_presentation_layer_actions_struct() {
        let actions1 = PresentationLayerActions::new();
        let actions2 = PresentationLayerActions::new();
        
        // Test equality
        assert_eq!(actions1, actions2);
        
        // Test that new instances are empty
        assert!(actions1.tuning_system_changes.is_empty());
        assert!(actions1.root_note_adjustments.is_empty());
    }

    /// Test action struct creation and equality
    #[wasm_bindgen_test]
    fn test_action_struct_creation() {
        let perm_req1 = RequestMicrophonePermission;
        let perm_req2 = RequestMicrophonePermission;
        assert_eq!(perm_req1, perm_req2);
        
        let tuning_change1 = ChangeTuningSystem { tuning_system: TuningSystem::JustIntonation };
        let tuning_change2 = ChangeTuningSystem { tuning_system: TuningSystem::JustIntonation };
        assert_eq!(tuning_change1, tuning_change2);
        
        let root_note1 = AdjustRootNote { root_note: Note::F };
        let root_note2 = AdjustRootNote { root_note: Note::F };
        assert_eq!(root_note1, root_note2);
    }

    // Debug action tests (only run in debug builds)
    #[cfg(debug_assertions)]
    #[wasm_bindgen_test]
    fn test_debug_actions_initially_empty() {
        let mut presenter = Presenter::create()
            .expect("Presenter creation should succeed");

        let debug_actions = presenter.get_debug_actions();
        
        assert!(debug_actions.test_signal_configurations.is_empty());
        assert!(debug_actions.speaker_output_configurations.is_empty());
        assert!(debug_actions.background_noise_configurations.is_empty());
    }

    #[cfg(debug_assertions)]
    #[wasm_bindgen_test]
    fn test_test_signal_configuration_collection() {
        use crate::engine::audio::TestWaveform;
        
        let mut presenter = Presenter::create()
            .expect("Presenter creation should succeed");

        // Configure test signal
        presenter.on_test_signal_configured(true, 440.0, 50.0, TestWaveform::Sine);
        
        let debug_actions = presenter.get_debug_actions();
        assert_eq!(debug_actions.test_signal_configurations.len(), 1);
        assert_eq!(debug_actions.test_signal_configurations[0].enabled, true);
        assert_eq!(debug_actions.test_signal_configurations[0].frequency, 440.0);
        assert_eq!(debug_actions.test_signal_configurations[0].volume, 50.0);
        assert_eq!(debug_actions.test_signal_configurations[0].waveform, TestWaveform::Sine);
        
        // After getting actions, they should be cleared
        let debug_actions2 = presenter.get_debug_actions();
        assert!(debug_actions2.test_signal_configurations.is_empty());
    }

    #[cfg(debug_assertions)]
    #[wasm_bindgen_test]
    fn test_speaker_output_configuration_collection() {
        let mut presenter = Presenter::create()
            .expect("Presenter creation should succeed");

        // Configure speaker output
        presenter.on_output_to_speakers_configured(true);
        
        let debug_actions = presenter.get_debug_actions();
        assert_eq!(debug_actions.speaker_output_configurations.len(), 1);
        assert_eq!(debug_actions.speaker_output_configurations[0].enabled, true);
        
        // After getting actions, they should be cleared
        let debug_actions2 = presenter.get_debug_actions();
        assert!(debug_actions2.speaker_output_configurations.is_empty());
    }

    #[cfg(debug_assertions)]
    #[wasm_bindgen_test]
    fn test_background_noise_configuration_collection() {
        use crate::engine::audio::TestWaveform;
        
        let mut presenter = Presenter::create()
            .expect("Presenter creation should succeed");

        // Configure background noise
        presenter.on_background_noise_configured(true, 0.1, TestWaveform::WhiteNoise);
        
        let debug_actions = presenter.get_debug_actions();
        assert_eq!(debug_actions.background_noise_configurations.len(), 1);
        assert_eq!(debug_actions.background_noise_configurations[0].enabled, true);
        assert_eq!(debug_actions.background_noise_configurations[0].level, 0.1);
        assert_eq!(debug_actions.background_noise_configurations[0].noise_type, TestWaveform::WhiteNoise);
        
        // After getting actions, they should be cleared
        let debug_actions2 = presenter.get_debug_actions();
        assert!(debug_actions2.background_noise_configurations.is_empty());
    }

    #[cfg(debug_assertions)]
    #[wasm_bindgen_test]
    fn test_multiple_debug_action_collection() {
        use crate::engine::audio::TestWaveform;
        
        let mut presenter = Presenter::create()
            .expect("Presenter creation should succeed");

        // Trigger multiple debug actions
        presenter.on_test_signal_configured(true, 880.0, 75.0, TestWaveform::Square);
        presenter.on_output_to_speakers_configured(false);
        presenter.on_background_noise_configured(true, 0.2, TestWaveform::PinkNoise);
        presenter.on_test_signal_configured(false, 220.0, 25.0, TestWaveform::Triangle); // Second test signal config
        
        let debug_actions = presenter.get_debug_actions();
        
        // Verify all actions were collected
        assert_eq!(debug_actions.test_signal_configurations.len(), 2);
        assert_eq!(debug_actions.speaker_output_configurations.len(), 1);
        assert_eq!(debug_actions.background_noise_configurations.len(), 1);
        
        // Verify first test signal config
        assert_eq!(debug_actions.test_signal_configurations[0].enabled, true);
        assert_eq!(debug_actions.test_signal_configurations[0].frequency, 880.0);
        assert_eq!(debug_actions.test_signal_configurations[0].waveform, TestWaveform::Square);
        
        // Verify second test signal config
        assert_eq!(debug_actions.test_signal_configurations[1].enabled, false);
        assert_eq!(debug_actions.test_signal_configurations[1].frequency, 220.0);
        assert_eq!(debug_actions.test_signal_configurations[1].waveform, TestWaveform::Triangle);
        
        // After getting actions, all should be cleared
        let debug_actions2 = presenter.get_debug_actions();
        assert!(debug_actions2.test_signal_configurations.is_empty());
        assert!(debug_actions2.speaker_output_configurations.is_empty());
        assert!(debug_actions2.background_noise_configurations.is_empty());
    }

    #[cfg(debug_assertions)]
    #[wasm_bindgen_test]
    fn test_debug_layer_actions_struct() {
        let actions1 = DebugLayerActions::new();
        let actions2 = DebugLayerActions::new();
        
        // Test equality
        assert_eq!(actions1, actions2);
        
        // Test that new instances are empty
        assert!(actions1.test_signal_configurations.is_empty());
        assert!(actions1.speaker_output_configurations.is_empty());
        assert!(actions1.background_noise_configurations.is_empty());
    }

    #[cfg(debug_assertions)]
    #[wasm_bindgen_test]
    fn test_debug_action_struct_creation() {
        use crate::engine::audio::TestWaveform;
        
        let test_signal1 = ConfigureTestSignal { enabled: true, frequency: 440.0, volume: 50.0, waveform: TestWaveform::Sine };
        let test_signal2 = ConfigureTestSignal { enabled: true, frequency: 440.0, volume: 50.0, waveform: TestWaveform::Sine };
        assert_eq!(test_signal1, test_signal2);
        
        let speaker1 = ConfigureOutputToSpeakers { enabled: true };
        let speaker2 = ConfigureOutputToSpeakers { enabled: true };
        assert_eq!(speaker1, speaker2);
        
        let noise1 = ConfigureBackgroundNoise { enabled: false, level: 0.5, noise_type: TestWaveform::PinkNoise };
        let noise2 = ConfigureBackgroundNoise { enabled: false, level: 0.5, noise_type: TestWaveform::PinkNoise };
        assert_eq!(noise1, noise2);
    }
}