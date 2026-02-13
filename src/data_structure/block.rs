use super::entity_id::ObjectId;
use super::layer::Layer;
use super::super::geometry::Point;

#[derive(Debug, Clone)]
pub struct Block {
    id: ObjectId,
    name: String,
    entities: Vec<()>,
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
}
