use std::collections::HashSet;
use super::ObjectId;

#[derive(Debug, Clone)]
pub struct SelectionSet {
    entity_ids: HashSet<ObjectId>,
    name: String,
    description: String,
}

impl SelectionSet {
    #[inline]
    pub fn new() -> Self {
        Self {
            entity_ids: HashSet::new(),
            name: String::new(),
            description: String::new(),
        }
    }

    #[inline]
    pub fn with_name(name: String) -> Self {
        Self {
            entity_ids: HashSet::new(),
            name,
            description: String::new(),
        }
    }

    #[inline]
    pub fn entity_ids(&self) -> &HashSet<ObjectId> {
        &self.entity_ids
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entity_ids.is_empty()
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.entity_ids.len()
    }

    #[inline]
    pub fn add(&mut self, entity_id: ObjectId) {
        self.entity_ids.insert(entity_id);
    }

    #[inline]
    pub fn add_multiple(&mut self, entity_ids: &[ObjectId]) {
        for id in entity_ids {
            self.entity_ids.insert(id.clone());
        }
    }

    #[inline]
    pub fn remove(&mut self, entity_id: &ObjectId) -> bool {
        self.entity_ids.remove(entity_id)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.entity_ids.clear();
    }

    #[inline]
    pub fn contains(&self, entity_id: &ObjectId) -> bool {
        self.entity_ids.contains(entity_id)
    }

    #[inline]
    pub fn union(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.entity_ids.extend(other.entity_ids.iter().cloned());
        result
    }

    #[inline]
    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.entity_ids.retain(|id| other.entity_ids.contains(id));
        result
    }

    #[inline]
    pub fn difference(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.entity_ids.retain(|id| !other.entity_ids.contains(id));
        result
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[inline]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[inline]
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}

impl Default for SelectionSet {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct SelectionManager {
    selection_sets: Vec<SelectionSet>,
    current_selection: SelectionSet,
}

impl SelectionManager {
    #[inline]
    pub fn new() -> Self {
        Self {
            selection_sets: Vec::new(),
            current_selection: SelectionSet::new(),
        }
    }

    #[inline]
    pub fn current_selection(&self) -> &SelectionSet {
        &self.current_selection
    }

    #[inline]
    pub fn current_selection_mut(&mut self) -> &mut SelectionSet {
        &mut self.current_selection
    }

    #[inline]
    pub fn select(&mut self, entity_id: ObjectId) {
        self.current_selection.add(entity_id);
    }

    #[inline]
    pub fn select_multiple(&mut self, entity_ids: &[ObjectId]) {
        self.current_selection.add_multiple(entity_ids);
    }

    #[inline]
    pub fn deselect(&mut self, entity_id: &ObjectId) {
        self.current_selection.remove(entity_id);
    }

    #[inline]
    pub fn clear_selection(&mut self) {
        self.current_selection.clear();
    }

    #[inline]
    pub fn save_selection(&mut self, name: String) {
        self.selection_sets.push(self.current_selection.clone());
        self.current_selection.set_name(name);
        self.current_selection = SelectionSet::new();
    }

    #[inline]
    pub fn load_selection(&mut self, name: &str) -> Option<&SelectionSet> {
        for selection in &self.selection_sets {
            if selection.name() == name {
                self.current_selection = selection.clone();
                return Some(&self.current_selection);
            }
        }
        None
    }
}

impl Default for SelectionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_set_creation() {
        let selection = SelectionSet::new();
        
        assert!(selection.is_empty());
        assert_eq!(selection.count(), 0);
    }

    #[test]
    fn test_selection_set_add() {
        let mut selection = SelectionSet::new();
        let id = ObjectId::new();
        
        selection.add(id.clone());
        assert_eq!(selection.count(), 1);
        assert!(selection.contains(&id));
    }

    #[test]
    fn test_selection_set_remove() {
        let mut selection = SelectionSet::new();
        let id = ObjectId::new();
        
        selection.add(id.clone());
        selection.remove(&id);
        assert!(selection.is_empty());
    }

    #[test]
    fn test_selection_manager() {
        let mut manager = SelectionManager::new();
        
        manager.select(ObjectId::new());
        assert_eq!(manager.current_selection().count(), 1);
        
        manager.clear_selection();
        assert!(manager.current_selection().is_empty());
    }
}
