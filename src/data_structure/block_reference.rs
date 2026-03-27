use super::entity_id::ObjectId;
use super::super::geometry::Point;
use crate::Vector2;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BlockReference {
    id: ObjectId,
    block_id: ObjectId,
    insertion_point: Point,
    scale: Vector2,
    rotation: f64,
}

impl BlockReference {
    #[inline]
    pub fn new(block_id: ObjectId, insertion_point: Point) -> Self {
        Self {
            id: ObjectId::new(),
            block_id,
            insertion_point,
            scale: Vector2::new(1.0, 1.0),
            rotation: 0.0,
        }
    }

    #[inline]
    pub fn id(&self) -> &ObjectId {
        &self.id
    }

    #[inline]
    pub fn block_id(&self) -> &ObjectId {
        &self.block_id
    }

    #[inline]
    pub fn insertion_point(&self) -> Point {
        self.insertion_point
    }

    #[inline]
    pub fn set_insertion_point(&mut self, point: Point) {
        self.insertion_point = point;
    }

    #[inline]
    pub fn scale(&self) -> Vector2 {
        self.scale
    }

    #[inline]
    pub fn set_scale(&mut self, scale: Vector2) {
        self.scale = scale;
    }

    #[inline]
    pub fn rotation(&self) -> f64 {
        self.rotation
    }

    #[inline]
    pub fn set_rotation(&mut self, rotation: f64) {
        self.rotation = rotation;
    }
}
