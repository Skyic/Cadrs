use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::{Entity, ObjectId, Layer, Block, BlockReference};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandType {
    AddEntity,
    RemoveEntity,
    ModifyEntity,
    AddLayer,
    RemoveLayer,
    ModifyLayer,
    AddBlock,
    RemoveBlock,
    AddBlockRef,
    RemoveBlockRef,
    Group,
    Ungroup,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    command_type: CommandType,
    description: String,
    timestamp: std::time::SystemTime,
    before_state: Option<CommandState>,
    after_state: Option<CommandState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandState {
    entities: HashMap<ObjectId, Entity>,
    layers: HashMap<ObjectId, Layer>,
    blocks: HashMap<ObjectId, Block>,
    block_references: HashMap<ObjectId, BlockReference>,
}

impl Command {
    fn new(command_type: CommandType, description: String) -> Self {
        Self {
            command_type,
            description,
            timestamp: std::time::SystemTime::now(),
            before_state: None,
            after_state: None,
        }
    }

    pub fn command_type(&self) -> &CommandType {
        &self.command_type
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn timestamp(&self) -> &std::time::SystemTime {
        &self.timestamp
    }
}

pub struct CommandManager {
    undo_stack: Vec<Command>,
    redo_stack: Vec<Command>,
    max_history_size: usize,
    current_state: CommandState,
}

impl CommandManager {
    #[inline]
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history_size: 100,
            current_state: CommandState::new(),
        }
    }

    #[inline]
    pub fn set_max_history_size(&mut self, size: usize) {
        self.max_history_size = size;
    }

    #[inline]
    pub fn max_history_size(&self) -> usize {
        self.max_history_size
    }

    #[inline]
    pub fn execute(&mut self, command: Command) {
        if let Some(last_command) = self.undo_stack.last() {
            if command.timestamp() <= last_command.timestamp() {
                return;
            }
        }

        self.undo_stack.push(command.clone());
        self.redo_stack.clear();

        while self.undo_stack.len() > self.max_history_size {
            self.undo_stack.remove(0);
        }
    }

    #[inline]
    pub fn undo(&mut self) -> Option<Command> {
        if let Some(command) = self.undo_stack.pop() {
            self.redo_stack.push(command.clone());
            Some(command)
        } else {
            None
        }
    }

    #[inline]
    pub fn redo(&mut self) -> Option<Command> {
        if let Some(command) = self.redo_stack.pop() {
            self.undo_stack.push(command.clone());
            Some(command)
        } else {
            None
        }
    }

    #[inline]
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    #[inline]
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    #[inline]
    pub fn undo_description(&self) -> Option<String> {
        self.undo_stack.last().map(|c| c.description().to_string())
    }

    #[inline]
    pub fn redo_description(&self) -> Option<String> {
        self.redo_stack.last().map(|c| c.description().to_string())
    }

    #[inline]
    pub fn undo_stack_size(&self) -> usize {
        self.undo_stack.len()
    }

    #[inline]
    pub fn redo_stack_size(&self) -> usize {
        self.redo_stack.len()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    #[inline]
    pub fn begin_batch(&mut self) -> BatchCommand {
        BatchCommand::new(self)
    }

    #[inline]
    pub fn add_entity(&mut self, entity: &Entity) -> Command {
        let mut command = Command::new(
            CommandType::AddEntity,
            format!("Add {}", entity.entity_type()),
        );
        command.after_state = Some(self.current_state.clone());
        self.execute(command.clone());
        self.current_state.add_entity(entity);
        command
    }

    #[inline]
    pub fn remove_entity(&mut self, entity_id: &ObjectId, entity: &Entity) -> Command {
        let mut command = Command::new(
            CommandType::RemoveEntity,
            format!("Remove {}", entity.entity_type()),
        );
        command.before_state = Some(self.current_state.clone());
        self.execute(command.clone());
        self.current_state.remove_entity(entity_id);
        command
    }

    #[inline]
    pub fn modify_entity(&mut self, entity_id: &ObjectId, old_entity: &Entity, new_entity: &Entity) -> Command {
        let mut command = Command::new(
            CommandType::ModifyEntity,
            format!("Modify {}", old_entity.entity_type()),
        );
        command.before_state = Some(self.current_state.clone());
        command.after_state = Some(self.current_state.clone());
        self.execute(command.clone());
        self.current_state.remove_entity(entity_id);
        self.current_state.add_entity(new_entity);
        command
    }
}

impl Default for CommandManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BatchCommand<'a> {
    manager: &'a mut CommandManager,
    commands: Vec<Command>,
    description: String,
}

impl<'a> BatchCommand<'a> {
    fn new(manager: &'a mut CommandManager) -> Self {
        Self {
            manager,
            commands: Vec::new(),
            description: String::new(),
        }
    }

    #[inline]
    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    #[inline]
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    #[inline]
    pub fn finish(mut self) {
        if !self.commands.is_empty() {
            let batch_command = Command::new(
                CommandType::Group,
                self.description.clone(),
            );
            self.manager.execute(batch_command);
        }
    }
}

impl CommandState {
    #[inline]
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            layers: HashMap::new(),
            blocks: HashMap::new(),
            block_references: HashMap::new(),
        }
    }

    #[inline]
    pub fn add_entity(&mut self, entity: &Entity) {
        self.entities.insert(entity.id().clone(), entity.clone());
    }

    #[inline]
    pub fn remove_entity(&mut self, entity_id: &ObjectId) {
        self.entities.remove(entity_id);
    }

    #[inline]
    pub fn entities(&self) -> &HashMap<ObjectId, Entity> {
        &self.entities
    }

    #[inline]
    pub fn layers(&self) -> &HashMap<ObjectId, Layer> {
        &self.layers
    }

    #[inline]
    pub fn blocks(&self) -> &HashMap<ObjectId, Block> {
        &self.blocks
    }

    #[inline]
    pub fn block_references(&self) -> &HashMap<ObjectId, BlockReference> {
        &self.block_references
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_manager_creation() {
        let manager = CommandManager::new();
        
        assert!(!manager.can_undo());
        assert!(!manager.can_redo());
        assert_eq!(manager.undo_stack_size(), 0);
        assert_eq!(manager.redo_stack_size(), 0);
    }

    #[test]
    fn test_command_execution() {
        let mut manager = CommandManager::new();
        let command = Command::new(
            CommandType::AddEntity,
            "Add Line".to_string(),
        );
        
        manager.execute(command);
        
        assert!(manager.can_undo());
        assert!(!manager.can_redo());
        assert_eq!(manager.undo_stack_size(), 1);
    }

    #[test]
    fn test_undo_redo() {
        let mut manager = CommandManager::new();
        let command = Command::new(
            CommandType::AddEntity,
            "Add Circle".to_string(),
        );
        
        manager.execute(command);
        assert!(manager.can_undo());
        
        let undone = manager.undo();
        assert!(undone.is_some());
        assert!(!manager.can_undo());
        assert!(manager.can_redo());
        
        let redone = manager.redo();
        assert!(redone.is_some());
        assert!(manager.can_undo());
        assert!(!manager.can_redo());
    }

    #[test]
    fn test_clear_history() {
        let mut manager = CommandManager::new();
        
        for i in 0..10 {
            let command = Command::new(
                CommandType::AddEntity,
                format!("Command {}", i),
            );
            manager.execute(command);
        }
        
        assert_eq!(manager.undo_stack_size(), 10);
        
        manager.clear();
        assert_eq!(manager.undo_stack_size(), 0);
        assert_eq!(manager.redo_stack_size(), 0);
    }

    #[test]
    fn test_max_history_size() {
        let mut manager = CommandManager::new();
        manager.set_max_history_size(5);
        
        for i in 0..10 {
            let command = Command::new(
                CommandType::AddEntity,
                format!("Command {}", i),
            );
            manager.execute(command);
        }
        
        assert_eq!(manager.undo_stack_size(), 5);
    }

    #[test]
    fn test_command_descriptions() {
        let mut manager = CommandManager::new();
        
        let command = Command::new(
            CommandType::AddEntity,
            "Add Rectangle".to_string(),
        );
        manager.execute(command);
        
        assert_eq!(manager.undo_description(), Some("Add Rectangle".to_string()));
        assert_eq!(manager.redo_description(), None);
    }
}
