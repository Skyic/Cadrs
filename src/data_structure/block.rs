use super::entity_id::ObjectId;
use super::layer::Layer;
use super::super::geometry::Point;
use super::entity::Entity;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Block {
    id: ObjectId,
    name: String,
    entities: Vec<Entity>,
    origin: Point,
    description: String,
}

impl Block {
    #[inline]
    pub fn new(name: String) -> Self {
        Self {
            id: ObjectId::new(),
            name,
            entities: Vec::new(),
            origin: Point::origin(),
            description: String::new(),
        }
    }

    #[inline]
    pub fn id(&self) -> &ObjectId {
        &self.id
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
    pub fn origin(&self) -> Point {
        self.origin
    }

    #[inline]
    pub fn set_origin(&mut self, origin: Point) {
        self.origin = origin;
    }

    #[inline]
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    #[inline]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[inline]
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    #[inline]
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    #[inline]
    pub fn get_entities(&self) -> &[Entity] {
        &self.entities
    }

    #[inline]
    pub fn get_entity(&self, id: &ObjectId) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id() == id)
    }

    #[inline]
    pub fn remove_entity(&mut self, id: &ObjectId) -> bool {
        if let Some(index) = self.entities.iter().position(|e| e.id() == id) {
            self.entities.remove(index);
            true
        } else {
            false
        }
    }
}
