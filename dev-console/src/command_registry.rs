// Console Command System
// Provides extensible command framework for development console

use std::collections::HashMap;
use crate::output::ConsoleOutput;
use crate::command::{ConsoleCommand, ConsoleCommandResult};

// Command registry for managing available commands
pub struct ConsoleCommandRegistry {
    commands: HashMap<String, Box<dyn ConsoleCommand>>,
}

impl Default for ConsoleCommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsoleCommandRegistry {
    /// Create a new registry with only built-in commands (no module dependencies)
    /// Built-in commands: help, clear, test
    pub fn new() -> Self {
        let mut registry = Self {
            commands: HashMap::new(),
        };
        
        // Register built-in commands that require no external module dependencies
        registry.register(Box::new(HelpCommand));
        registry.register(Box::new(ClearCommand));
        registry.register(Box::new(TestCommand));
        
        registry
    }
    
    pub fn register(&mut self, command: Box<dyn ConsoleCommand>) {
        self.commands.insert(command.name().to_string(), command);
    }
    
    pub fn execute(&self, input: &str) -> ConsoleCommandResult {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return ConsoleCommandResult::Output(ConsoleOutput::error("Empty command"));
        }
        
        let command_name = parts[0];
        let args = parts[1..].to_vec();
        
        if let Some(command) = self.commands.get(command_name) {
            return command.execute(args, self);
        }
        
        ConsoleCommandResult::Output(ConsoleOutput::error(format!("Unknown command: {}", command_name)))
    }
    
    pub fn get_commands(&self) -> Vec<&dyn ConsoleCommand> {
        self.commands.values().map(|cmd| cmd.as_ref()).collect()
    }
}

// Built-in Help Command
struct HelpCommand;

impl ConsoleCommand for HelpCommand {
    fn name(&self) -> &str {
        "help"
    }
    
    fn description(&self) -> &str {
        "Display available commands and usage"
    }
    
    fn execute(&self, _args: Vec<&str>, registry: &ConsoleCommandRegistry) -> ConsoleCommandResult {
        let mut help_lines = vec!["Available commands:".to_string()];
        
        let mut commands = registry.get_commands();
        commands.sort_by(|a, b| a.name().cmp(b.name()));
        
        // Show all registered commands
        for command in commands {
            help_lines.push(format!("  {} - {}", command.name(), command.description()));
        }
        
        let help_text = help_lines.join("\n");
        ConsoleCommandResult::Output(ConsoleOutput::info(help_text))
    }
}

// Built-in Clear Command
struct ClearCommand;

impl ConsoleCommand for ClearCommand {
    fn name(&self) -> &str {
        "clear"
    }
    
    fn description(&self) -> &str {
        "Clear console output"
    }
    
    fn execute(&self, _args: Vec<&str>, _registry: &ConsoleCommandRegistry) -> ConsoleCommandResult {
        ConsoleCommandResult::ClearAndOutput(ConsoleOutput::info("Console cleared"))
    }
}


// Built-in Test Command - Shows examples of all ConsoleOutput variants
struct TestCommand;

impl ConsoleCommand for TestCommand {
    fn name(&self) -> &str {
        "test"
    }
    
    fn description(&self) -> &str {
        "Show examples of all console output types"
    }
    
    fn execute(&self, _args: Vec<&str>, _registry: &ConsoleCommandRegistry) -> ConsoleCommandResult {
        // This command demonstrates all available ConsoleOutput variants
        // by returning multiple outputs with proper styling
        
        let outputs = vec![
            ConsoleOutput::info("Console Output Examples:"),
            ConsoleOutput::empty(),
            ConsoleOutput::info("This is an informational message"),
            ConsoleOutput::success("This is a success message"),
            ConsoleOutput::warning("This is a warning message"),
            ConsoleOutput::error("This is an error message"),
            ConsoleOutput::empty(),
        ];
        
        ConsoleCommandResult::MultipleOutputs(outputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    // No wasm_bindgen_test_configure! needed for Node.js
    
    #[wasm_bindgen_test]
    fn test_command_registry_basic_functionality() {
        let registry = ConsoleCommandRegistry::new();
        
        // Test help command
        let result = registry.execute("help");
        match result {
            ConsoleCommandResult::Output(ConsoleOutput::Info(text)) => {
                assert!(text.contains("Available commands"));
                assert!(text.contains("help - Display available commands and usage"));
                assert!(text.contains("clear - Clear console output"));
                assert!(text.contains("test - Show examples of all console output types"));
                // Module commands should NOT be present in built-ins only registry
                assert!(!text.contains("api-status - Show application and API status"));
                assert!(!text.contains("mic-status"));
            },
            _ => panic!("Expected Info output from help command"),
        }
        
        // Test clear command
        let result = registry.execute("clear");
        match result {
            ConsoleCommandResult::ClearAndOutput(ConsoleOutput::Info(text)) => assert_eq!(text, "Console cleared"),
            _ => panic!("Expected ClearAndOutput result from clear command"),
        }
        
        
        // Test unknown command
        let result = registry.execute("unknown");
        match result {
            ConsoleCommandResult::Output(ConsoleOutput::Error(text)) => assert!(text.contains("Unknown command")),
            _ => panic!("Expected Error output for unknown command"),
        }
        
        // Test test command
        let result = registry.execute("test");
        match result {
            ConsoleCommandResult::MultipleOutputs(outputs) => {
                assert!(!outputs.is_empty());
                // Should contain examples of different output types
                assert!(outputs.iter().any(|o| matches!(o, ConsoleOutput::Info(_))));
                assert!(outputs.iter().any(|o| matches!(o, ConsoleOutput::Success(_))));
                assert!(outputs.iter().any(|o| matches!(o, ConsoleOutput::Warning(_))));
                assert!(outputs.iter().any(|o| matches!(o, ConsoleOutput::Error(_))));
            },
            _ => panic!("Expected MultipleOutputs result from test command"),
        }
    }
    
    #[wasm_bindgen_test]
    fn test_command_parsing() {
        let registry = ConsoleCommandRegistry::new();
        
        // Test empty command
        let result = registry.execute("");
        match result {
            ConsoleCommandResult::Output(ConsoleOutput::Error(text)) => assert_eq!(text, "Empty command"),
            _ => panic!("Expected Error output for empty command"),
        }
        
        // Test command with whitespace
        let result = registry.execute("  help  ");
        match result {
            ConsoleCommandResult::Output(ConsoleOutput::Info(_)) => (), // Success
            _ => panic!("Expected Info output from help command with whitespace"),
        }
    }
    
    #[wasm_bindgen_test]
    fn test_console_output_types() {
        let info = ConsoleOutput::info("test");
        let error = ConsoleOutput::error("test");
        let command = ConsoleOutput::echo("test");
        
        assert_ne!(info, error);
        assert_ne!(error, command);
        assert_ne!(command, info);
    }


    #[wasm_bindgen_test]
    fn test_help_shows_all_commands() {
        // Create a test registry with multiple commands
        struct BaseTestCommand;
        impl ConsoleCommand for BaseTestCommand {
            fn name(&self) -> &str { "base" }
            fn description(&self) -> &str { "Base command" }
            fn execute(&self, _args: Vec<&str>, _registry: &ConsoleCommandRegistry) -> ConsoleCommandResult {
                ConsoleCommandResult::Output(ConsoleOutput::info("base"))
            }
        }

        struct SubTestCommand;
        impl ConsoleCommand for SubTestCommand {
            fn name(&self) -> &str { "base-sub" }
            fn description(&self) -> &str { "Sub command" }
            fn execute(&self, _args: Vec<&str>, _registry: &ConsoleCommandRegistry) -> ConsoleCommandResult {
                ConsoleCommandResult::Output(ConsoleOutput::info("compound"))
            }
        }

        let mut registry = ConsoleCommandRegistry::new();
        registry.register(Box::new(BaseTestCommand));
        registry.register(Box::new(SubTestCommand));

        // Test that help shows all registered commands
        let result = registry.execute("help");
        match result {
            ConsoleCommandResult::Output(ConsoleOutput::Info(text)) => {
                assert!(text.contains("base - Base command"));
                assert!(text.contains("base-sub - Sub command")); // Should show all commands
            },
            _ => panic!("Expected Info output from help command"),
        }
    }   
}