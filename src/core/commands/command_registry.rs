use std::collections::HashMap;

use super::command_trait::CommandTrait;

/// A command registry.
/// 
/// This struct is used to store all the commands as a rocket managed state.
pub struct CommandRegistry<'a> {
    /// The commands.
    /// 
    /// Commands are stored in a boxed trait object to allow different types of commands to be stored in the same hashmap.
    pub commands: HashMap<&'a str, Box<dyn CommandTrait<'a>>>,
}

impl<'a> CommandRegistry<'a> {
    /// Create a new command registry.
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    /// Register a command into the registry.
    pub fn register(&mut self, command: Box<dyn CommandTrait<'a>>) {
        self.commands.insert(command.name(), command);
    }

    /// Get a command from the registry.
    pub fn get(&self, name: &str) -> Option<&Box<dyn CommandTrait<'a>>> {
        self.commands.get(name)
    }
}