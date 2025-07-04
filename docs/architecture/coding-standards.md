# Coding Standards

## Core Development Principles

### YAGNI (You Aren't Gonna Need It)

**YAGNI is a fundamental principle for this project.** Do not implement features, modules, or infrastructure until they are actually needed for the current task.

#### Key Rules:
- **Never create placeholder files** for future implementation
- **Never implement unused modules** that will create compiler warnings
- **Never build infrastructure** for anticipated future needs
- **Use TODO comments** instead of empty implementations
- **Write stubs only** when code is referenced but incomplete

#### Examples:

**L YAGNI Violation:**
```rust
// Creating unused module files
// src/future_feature.rs - empty file for "future use"
pub struct FutureFeature; // unused, causes warnings
```

** YAGNI Compliant:**
```rust
// TODO: Implement FutureFeature when Task 15 is ready
// This avoids compiler warnings and complexity
```

#### Why YAGNI Matters:
- Prevents compiler warnings that complicate code review
- Keeps codebase focused on current requirements
- Reduces complexity and maintenance burden
- Roadmaps change - unused code becomes technical debt

#### Implementation Guidelines:
- Create files and modules **only when they are actively used**
- Use TODO comments to document planned future work
- Implement stubs only for **referenced but incomplete** functionality
- Remove unused code immediately during refactoring

### Code Quality Standards

#### Rust-Specific Guidelines
- Follow `rustfmt` formatting standards
- Use `clippy` lints for code quality
- Prefer explicit error handling over `.unwrap()`
- Use type annotations for complex generic functions

#### Performance Standards
- Zero-allocation paths for audio processing
- Pre-allocate buffers for real-time operations
- Use `Arc<T>` for large data sharing
- Profile memory usage in development builds

#### Documentation Standards
- Document public APIs with rustdoc comments
- Include code examples for complex functions
- Document performance expectations for critical paths
- Maintain architecture decision records

#### Testing Standards

##### Phased Testing Strategy
- **Phase 1 (Current) - Native Tests (cargo test)**: Fast feedback for Rust logic
  - 1 meaningful test covering build configuration detection
  - Immediate feedback during development
  - No browser dependencies for core logic testing
- **Phase 2 (Future) - WASM Tests (wasm-pack test)**: WebAssembly-specific functionality
  - Planned for when we have audio processing and module interactions
  - Focus on WASM compilation, memory management, and module boundaries
  - **NOT** for browser API integration (use E2E tools instead)
- **Phase 3 (Later) - E2E Tests**: Browser integration and user workflows
  - Cypress/Playwright for Canvas/WebGPU, Web Audio API, user interactions

##### Test Organization
- Unit tests for all audio processing algorithms (when implemented)
- WASM integration tests for module boundaries (Phase 2)
- Performance tests for real-time requirements
- Module structure validation following YAGNI principle
- Browser integration testing via E2E tools (Phase 3)

##### Testing Commands
- **Current**: `cargo test` (1 test, instant feedback)
- **Future**: `wasm-pack test --headless --firefox` (when we have WASM-specific functionality)
- **Later**: Cypress/Playwright for browser integration testing

## Module Organization

### File Structure
- Keep modules focused on single responsibilities
- Use clear, descriptive naming conventions
- Group related functionality in module directories
- Separate development tools from production code

### Dependency Management
- Minimize external dependencies
- Verify WebAssembly compatibility
- Pin dependency versions for reproducible builds
- Regular security audits of third-party crates

## Error Handling

### Error Strategy
- Use `Result<T, E>` for recoverable errors
- Use `panic!` only for unrecoverable programming errors
- Provide meaningful error messages
- Log errors with context for debugging

### Audio Processing Errors
- Handle microphone permission failures gracefully
- Recover from audio stream interruptions
- Provide user feedback for audio-related issues
- Maintain application stability during audio errors

## Security Guidelines

### WebAssembly Security
- No file system access beyond browser APIs
- Validate all external input data
- Use Rust's memory safety guarantees
- Follow secure coding practices

### Audio Privacy
- Process audio data locally only
- No persistent storage of audio data
- Clear audio buffers on session end
- Respect user privacy preferences