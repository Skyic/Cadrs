use crate::geometry::Point;
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurfaceTexture {
    pub symbol: SurfaceSymbol,
    pub value: Option<f64>,
    pub unit: TextureUnit,
    pub sampling_length: Option<f64>,
    pub production_method: ProductionMethod,
    pub additional_instructions: Vec<String>,
    pub direction: TextureDirection,
    pub allowance: Option<f64>,
    pub roughness_other: Option<String>,
    pub symbol_placement: SymbolPlacement,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SurfaceSymbol {
    Basic,
    Machined,
    Unmachined,
    MaterialRemovalRequired,
    MaterialRemovalProhibited,
    RoughnessValue,
    waviness_value,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TextureUnit {
    Micrometers,
    Microinches,
    Millimeters,
}

impl Default for TextureUnit {
    fn default() -> Self {
        TextureUnit::Micrometers
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ProductionMethod {
    NotSpecified,
    Machined,
    Ground,
    Honed,
    Polished,
    Superfinished,
    Drilled,
    Bored,
    Milled,
    Planed,
    Turned,
    Tapped,
    Rolled,
    Forged,
    Cast,
    Molded,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TextureDirection {
    Perpendicular,
    Parallel,
    Angular,
    MultiDirectional,
    Circular,
    Radial,
    Pitted,
    FreeForm,
    NotSpecified,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SymbolPlacement {
    Above,
    Below,
    Both,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurfaceTextureSymbol {
    pub symbol_type: SurfaceSymbol,
    pub value: Option<f64>,
    pub unit: TextureUnit,
    pub sampling_length: Option<f64>,
    pub production_method: Option<ProductionMethod>,
    pub direction: Option<TextureDirection>,
    pub allowance: Option<f64>,
}

impl SurfaceTextureSymbol {
    pub fn new() -> Self {
        Self {
            symbol_type: SurfaceSymbol::Basic,
            value: None,
            unit: TextureUnit::Micrometers,
            sampling_length: None,
            production_method: None,
            direction: None,
            allowance: None,
        }
    }
    
    pub fn with_roughness(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }
    
    pub fn with_production_method(mut self, method: ProductionMethod) -> Self {
        self.production_method = Some(method);
        self
    }
    
    pub fn with_direction(mut self, direction: TextureDirection) -> Self {
        self.direction = Some(direction);
        self
    }
    
    pub fn with_sampling_length(mut self, length: f64) -> Self {
        self.sampling_length = Some(length);
        self
    }
    
    pub fn with_unit(mut self, unit: TextureUnit) -> Self {
        self.unit = unit;
        self
    }
    
    pub fn with_allowance(mut self, allowance: f64) -> Self {
        self.allowance = Some(allowance);
        self
    }
    
    pub fn format_roughness(&self) -> String {
        if let Some(value) = self.value {
            match self.unit {
                TextureUnit::Micrometers => format!("{:.1}", value),
                TextureUnit::Microinches => format!("{:.1}", value),
                TextureUnit::Millimeters => format!("{:.3}", value),
            }
        } else {
            String::new()
        }
    }
    
    pub fn format_symbol(&self) -> String {
        match self.symbol_type {
            SurfaceSymbol::Basic => String::from(""),
            SurfaceSymbol::Machined => String::from(""),
            SurfaceSymbol::Unmachined => String::from(""),
            SurfaceSymbol::MaterialRemovalRequired => String::from(""),
            SurfaceSymbol::MaterialRemovalProhibited => String::from(""),
            SurfaceSymbol::RoughnessValue => self.format_roughness(),
            SurfaceSymbol::waviness_value => self.format_roughness(),
        }
    }
}

impl Default for SurfaceTextureSymbol {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SurfaceTextureSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol_char = match self.symbol_type {
            SurfaceSymbol::Basic => "",
            SurfaceSymbol::Machined => "",
            SurfaceSymbol::Unmachined => "",
            SurfaceSymbol::MaterialRemovalRequired => "Ⓡ",
            SurfaceSymbol::MaterialRemovalProhibited => "Ⓝ",
            SurfaceSymbol::RoughnessValue => "",
            SurfaceSymbol::waviness_value => "",
        };
        
        write!(f, "{}{}", symbol_char, self.format_roughness())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurfaceTexturePlacement {
    pub symbol: SurfaceTextureSymbol,
    pub location: Point,
    pub rotation: f64,
    pub scale: f64,
    pub attachment_point: SymbolPlacement,
}

impl SurfaceTexturePlacement {
    pub fn new(symbol: SurfaceTextureSymbol, location: Point) -> Self {
        Self {
            symbol,
            location,
            rotation: 0.0,
            scale: 1.0,
            attachment_point: SymbolPlacement::Below,
        }
    }
    
    pub fn rotated(self, angle: f64) -> Self {
        Self {
            rotation: self.rotation + angle,
            ..self
        }
    }
    
    pub fn scaled(self, factor: f64) -> Self {
        Self {
            scale: self.scale * factor,
            ..self
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurfaceTextureAnnotation {
    pub symbol_placements: Vec<SurfaceTexturePlacement>,
    pub text_height: f64,
    pub font: String,
    pub common_placement: Option<CommonPlacement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CommonPlacement {
    Above,
    Below,
    Left,
    Right,
    AllAround,
}

impl SurfaceTextureAnnotation {
    pub fn new() -> Self {
        Self {
            symbol_placements: Vec::new(),
            text_height: 2.5,
            font: "Standard".to_string(),
            common_placement: None,
        }
    }
    
    pub fn add_symbol(&mut self, placement: SurfaceTexturePlacement) {
        self.symbol_placements.push(placement);
    }
    
    pub fn add_roughness_symbol(
        &mut self,
        value: f64,
        location: Point,
        direction: TextureDirection,
    ) {
        let symbol = SurfaceTextureSymbol::new()
            .with_roughness(value)
            .with_direction(direction);
        
        let placement = SurfaceTexturePlacement::new(symbol, location);
        self.symbol_placements.push(placement);
    }
    
    pub fn set_common_placement(&mut self, placement: CommonPlacement) {
        self.common_placement = Some(placement);
    }
}

impl Default for SurfaceTextureAnnotation {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurfaceTextureRequirements {
    pub primary_requirement: SurfaceTextureSymbol,
    pub secondary_requirement: Option<SurfaceTextureSymbol>,
    pub sampling_length: Option<f64>,
    pub production_method: Option<ProductionMethod>,
    pub direction: Option<TextureDirection>,
    pub surface_treatments: Vec<SurfaceTreatment>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SurfaceTreatment {
    Plated,
    Hardened,
    Nitride,
    Carburize,
    Anodized,
    Painted,
    Coated,
    Oxidized,
    Other(String),
}

impl SurfaceTextureRequirements {
    pub fn new(roughness: f64) -> Self {
        Self {
            primary_requirement: SurfaceTextureSymbol::new().with_roughness(roughness),
            secondary_requirement: None,
            sampling_length: None,
            production_method: None,
            direction: None,
            surface_treatments: Vec::new(),
            notes: Vec::new(),
        }
    }
    
    pub fn with_secondary_roughness(mut self, value: f64) -> Self {
        self.secondary_requirement = Some(SurfaceTextureSymbol::new().with_roughness(value));
        self
    }
    
    pub fn with_sampling_length(mut self, length: f64) -> Self {
        self.sampling_length = Some(length);
        self
    }
    
    pub fn with_production_method(mut self, method: ProductionMethod) -> Self {
        self.production_method = Some(method);
        self
    }
    
    pub fn add_treatment(&mut self, treatment: SurfaceTreatment) {
        self.surface_treatments.push(treatment);
    }
    
    pub fn add_note(&mut self, note: String) {
        self.notes.push(note);
    }
}
