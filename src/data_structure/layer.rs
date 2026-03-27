use uuid::Uuid;
use std::collections::HashMap;
use super::entity_id::ObjectId;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Layer {
    id: ObjectId,
    name: String,
    description: String,
    color: Color,
    line_type: (),
    line_weight: f64,
    visibility: LayerVisibility,
    plot_style: String,
    properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LayerVisibility {
    Visible,
    Hidden,
    Frozen,
    Locked,
}

impl Layer {
    pub fn new(name: String) -> Self {
        Self {
            id: ObjectId::new(),
            name,
            description: String::new(),
            color: Color { red: 255, green: 255, blue: 255 },
            line_type: (),
            line_weight: 0.25,
            visibility: LayerVisibility::Visible,
            plot_style: String::new(),
            properties: HashMap::new(),
        }
    }
    
    pub fn id(&self) -> &ObjectId {
        &self.id
    }
    
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}
