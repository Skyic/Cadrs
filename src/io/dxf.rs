use thiserror::Error;
use crate::io::Error as ImportError;
use crate::data_structure::{Document, Layer, Entity, ObjectId, Block, EntityType, EntityGeometry, TextStyle, TextAlignment, BlockReference};
use crate::geometry::{Point, Line, Circle, Arc, Ellipse, Polyline, BSpline};
use std::collections::HashMap;

#[derive(Debug, Error)]
pub enum DXFError {
    #[error("Failed to parse DXF file: {0}")]
    ParseError(String),
    
    #[error("Invalid DXF version: {0}")]
    InvalidVersion(String),
    
    #[error("Missing required section: {0}")]
    MissingSection(String),
    
    #[error("Invalid entity: {0}")]
    InvalidEntity(String),
    
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DXFVersion {
    R12,
    R14,
    R2000,
    R2004,
    R2007,
    R2010,
    R2013,
    R2018,
}

impl DXFVersion {
    pub fn from_header(header: &str) -> Option<Self> {
        if header.contains("$ACADVER") {
            if header.contains("AC1009") {
                Some(DXFVersion::R12)
            } else if header.contains("AC1012") {
                Some(DXFVersion::R14)
            } else if header.contains("AC1015") {
                Some(DXFVersion::R2000)
            } else if header.contains("AC1018") {
                Some(DXFVersion::R2004)
            } else if header.contains("AC1021") {
                Some(DXFVersion::R2007)
            } else if header.contains("AC1024") {
                Some(DXFVersion::R2010)
            } else if header.contains("AC1027") {
                Some(DXFVersion::R2013)
            } else if header.contains("AC1032") {
                Some(DXFVersion::R2018)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct DXFWriter;

impl DXFWriter {
    pub fn new() -> Self {
        Self
    }

    pub fn write_header(&self, version: DXFVersion) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push("SECTION".to_string());
        lines.push("  2".to_string());
        lines.push("HEADER".to_string());
        
        lines.push("  9".to_string());
        lines.push("$ACADVER".to_string());
        lines.push("  1".to_string());
        
        let acad_version = match version {
            DXFVersion::R12 => "AC1009",
            DXFVersion::R14 => "AC1012",
            DXFVersion::R2000 => "AC1015",
            DXFVersion::R2004 => "AC1018",
            DXFVersion::R2007 => "AC1021",
            DXFVersion::R2010 => "AC1024",
            DXFVersion::R2013 => "AC1027",
            DXFVersion::R2018 => "AC1032",
        };
        lines.push(acad_version.to_string());
        
        lines.push("ENDSEC".to_string());
        
        lines
    }

    pub fn write_line(&self, start: (f64, f64, f64), end: (f64, f64, f64), layer: &str) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push("  0".to_string());
        lines.push("LINE".to_string());
        lines.push("  8".to_string());
        lines.push(layer.to_string());
        lines.push(" 10".to_string());
        lines.push(format!("{:.6}", start.0));
        lines.push(" 20".to_string());
        lines.push(format!("{:.6}", start.1));
        lines.push(" 30".to_string());
        lines.push(format!("{:.6}", start.2));
        lines.push(" 11".to_string());
        lines.push(format!("{:.6}", end.0));
        lines.push(" 21".to_string());
        lines.push(format!("{:.6}", end.1));
        lines.push(" 31".to_string());
        lines.push(format!("{:.6}", end.2));
        lines
    }

    pub fn write_circle(&self, center: (f64, f64, f64), radius: f64, layer: &str) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push("  0".to_string());
        lines.push("CIRCLE".to_string());
        lines.push("  8".to_string());
        lines.push(layer.to_string());
        lines.push(" 10".to_string());
        lines.push(format!("{:.6}", center.0));
        lines.push(" 20".to_string());
        lines.push(format!("{:.6}", center.1));
        lines.push(" 30".to_string());
        lines.push(format!("{:.6}", center.2));
        lines.push(" 40".to_string());
        lines.push(format!("{:.6}", radius));
        lines
    }

    pub fn write_arc(&self, center: (f64, f64, f64), radius: f64, start_angle: f64, end_angle: f64, layer: &str) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push("  0".to_string());
        lines.push("ARC".to_string());
        lines.push("  8".to_string());
        lines.push(layer.to_string());
        lines.push(" 10".to_string());
        lines.push(format!("{:.6}", center.0));
        lines.push(" 20".to_string());
        lines.push(format!("{:.6}", center.1));
        lines.push(" 30".to_string());
        lines.push(format!("{:.6}", center.2));
        lines.push(" 40".to_string());
        lines.push(format!("{:.6}", radius));
        lines.push(" 50".to_string());
        lines.push(format!("{:.6}", start_angle.to_degrees()));
        lines.push(" 51".to_string());
        lines.push(format!("{:.6}", end_angle.to_degrees()));
        lines
    }

    pub fn write_polyline(&self, vertices: &[(f64, f64, f64)], layer: &str) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push("  0".to_string());
        lines.push("POLYLINE".to_string());
        lines.push("  8".to_string());
        lines.push(layer.to_string());
        lines.push(" 62".to_string());
        lines.push("1".to_string());
        
        for (i, vertex) in vertices.iter().enumerate() {
            lines.push("  0".to_string());
            lines.push("VERTEX".to_string());
            lines.push("  8".to_string());
            lines.push(layer.to_string());
            lines.push(" 10".to_string());
            lines.push(format!("{:.6}", vertex.0));
            lines.push(" 20".to_string());
            lines.push(format!("{:.6}", vertex.1));
            lines.push(" 30".to_string());
            lines.push(format!("{:.6}", vertex.2));
            if i == vertices.len() - 1 {
                lines.push(" 70".to_string());
                lines.push("1".to_string());
            }
        }
        
        lines.push("  0".to_string());
        lines.push("SEQEND".to_string());
        lines.push("  8".to_string());
        lines.push(layer.to_string());
        
        lines
    }
}

struct DXFParser<'a> {
    lines: Vec<&'a str>,
    current_index: usize,
    entities: Vec<Entity>,
    layers: HashMap<String, ObjectId>,
    blocks: HashMap<String, Block>,
    current_block: Option<String>,
}

impl<'a> DXFParser<'a> {
    fn new(lines: Vec<&'a str>) -> Self {
        Self {
            lines,
            current_index: 0,
            entities: Vec::new(),
            layers: HashMap::new(),
            blocks: HashMap::new(),
            current_block: None,
        }
    }

    fn next_pair(&mut self) -> Option<(&str, &str)> {
        if self.current_index + 1 >= self.lines.len() {
            return None;
        }
        let group_code = self.lines[self.current_index].trim();
        let value = self.lines[self.current_index + 1].trim();
        self.current_index += 2;
        Some((group_code, value))
    }

    fn parse_coordinate(&mut self, x_code: &str, y_code: &str, z_code: &str) -> Point {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        
        while let Some((code, value)) = self.next_pair() {
            match code {
                x_code => x = value.parse().unwrap_or(0.0),
                y_code => y = value.parse().unwrap_or(0.0),
                z_code => z = value.parse().unwrap_or(0.0),
                _ => {
                    self.current_index -= 2;
                    break;
                }
            }
        }
        
        Point::new(x, y, z)
    }

    fn parse_layer(&mut self) {
        let mut layer_name = String::new();
        let mut color = 7;
        
        while let Some((code, value)) = self.next_pair() {
            match code {
                "  2" => layer_name = value.to_string(),
                " 62" => color = value.parse().unwrap_or(7),
                _ => {}
            }
            
            if layer_name.is_empty() {
                continue;
            }
            
            if code == "  0" && value == "ENDSEC" {
                self.current_index -= 2;
                break;
            }
            
            if code == "  0" {
                self.current_index -= 2;
                break;
            }
        }
        
        if !layer_name.is_empty() {
            let layer = Layer::new(layer_name.clone());
            let layer_id = ObjectId::new();
            self.layers.insert(layer_name, layer_id);
        }
    }

    fn parse_line(&mut self, layer: &str) {
        let start = self.parse_coordinate(" 10", " 20", " 30");
        let end = self.parse_coordinate(" 11", " 21", " 31");
        
        let line = Line::new(start, end);
        let entity = Entity::new(
            EntityType::Line,
            EntityGeometry::Line(line),
        );
        
        if let Some(ref mut block) = self.current_block.as_mut().and_then(|name| self.blocks.get_mut(name)) {
            block.add_entity(entity);
        } else {
            self.entities.push(entity);
        }
    }

    fn parse_circle(&mut self, layer: &str) {
        let center = self.parse_coordinate(" 10", " 20", " 30");
        let mut radius = 1.0;
        
        while let Some((code, value)) = self.next_pair() {
            if code == " 40" {
                radius = value.parse().unwrap_or(1.0);
                break;
            }
        }
        
        let circle = Circle::new(center, radius);
        let entity = Entity::new(
            EntityType::Circle,
            EntityGeometry::Circle(circle),
        );
        
        if let Some(ref mut block) = self.current_block.as_mut().and_then(|name| self.blocks.get_mut(name)) {
            block.add_entity(entity);
        } else {
            self.entities.push(entity);
        }
    }

    fn parse_arc(&mut self, layer: &str) {
        let center = self.parse_coordinate(" 10", " 20", " 30");
        let mut radius = 1.0;
        let mut start_angle = 0.0;
        let mut end_angle = 0.0;
        
        while let Some((code, value)) = self.next_pair() {
            match code {
                " 40" => radius = value.parse().unwrap_or(1.0),
                " 50" => start_angle = value.parse::<f64>().unwrap_or(0.0).to_radians(),
                " 51" => end_angle = value.parse::<f64>().unwrap_or(0.0).to_radians(),
                _ => {
                    if code.starts_with("  0") || code.starts_with("  8") {
                        self.current_index -= 2;
                        break;
                    }
                }
            }
        }
        
        let arc = Arc::new(center, radius, start_angle, end_angle);
        let entity = Entity::new(
            EntityType::Arc,
            EntityGeometry::Arc(arc),
        );
        
        if let Some(ref mut block) = self.current_block.as_mut().and_then(|name| self.blocks.get_mut(name)) {
            block.add_entity(entity);
        } else {
            self.entities.push(entity);
        }
    }

    fn parse_ellipse(&mut self, layer: &str) {
        let center = self.parse_coordinate(" 10", " 20", " 30");
        let mut major_axis = Point::new(1.0, 0.0, 0.0);
        let mut ratio = 0.5;
        let mut start_param = 0.0;
        let mut end_param = std::f64::consts::PI * 2.0;
        
        while let Some((code, value)) = self.next_pair() {
            match code {
                " 11" => {
                    if let (Some(x), Some(y), Some(z)) = (
                        value.parse().ok(),
                        self.lines.get(self.current_index).and_then(|s| s.parse().ok()),
                        self.lines.get(self.current_index + 1).and_then(|s| s.parse().ok())
                    ) {
                        major_axis = Point::new(x, y, z);
                        self.current_index += 2;
                    }
                }
                " 40" => ratio = value.parse().unwrap_or(0.5),
                " 51" => start_param = value.parse::<f64>().unwrap_or(0.0).to_radians(),
                " 52" => end_param = value.parse::<f64>().unwrap_or(std::f64::consts::PI * 2.0).to_radians(),
                _ => {
                    if code.starts_with("  0") || code.starts_with("  8") {
                        self.current_index -= 2;
                        break;
                    }
                }
            }
        }
        
        let major_axis_length = major_axis.distance_to(&Point::origin());
        let minor_axis_length = major_axis_length * ratio;
        let rotation = if major_axis.x.abs() > major_axis.y.abs() {
            (major_axis.y / major_axis.x).atan()
        } else {
            std::f64::consts::PI / 2.0 - (major_axis.x / major_axis.y).atan()
        };
        let ellipse = Ellipse::new(center, major_axis_length / 2.0, minor_axis_length / 2.0, rotation);
        let entity = Entity::new(
            EntityType::Ellipse,
            EntityGeometry::Ellipse(ellipse),
        );
        
        if let Some(ref mut block) = self.current_block.as_mut().and_then(|name| self.blocks.get_mut(name)) {
            block.add_entity(entity);
        } else {
            self.entities.push(entity);
        }
    }

    fn parse_polyline(&mut self, layer: &str) {
        let mut vertices = Vec::new();
        
        while let Some((code, value)) = self.next_pair() {
            if code == "  0" {
                if value == "SEQEND" {
                    break;
                } else if value == "VERTEX" {
                    let vertex = self.parse_coordinate(" 10", " 20", " 30");
                    vertices.push(vertex);
                }
            }
        }
        
        let polyline = Polyline::from_points(&vertices);
        let entity = Entity::new(
            EntityType::Polyline,
            EntityGeometry::Polyline(polyline),
        );
        
        if let Some(ref mut block) = self.current_block.as_mut().and_then(|name| self.blocks.get_mut(name)) {
            block.add_entity(entity);
        } else {
            self.entities.push(entity);
        }
    }

    fn parse_lwpolyline(&mut self, layer: &str) {
        let mut vertices = Vec::new();
        let mut x = 0.0;
        let mut y = 0.0;
        let mut has_vertex = false;
        
        while let Some((code, value)) = self.next_pair() {
            match code {
                " 10" => {
                    x = value.parse().unwrap_or(0.0);
                    has_vertex = false;
                }
                " 20" => {
                    y = value.parse().unwrap_or(0.0);
                    vertices.push(Point::new(x, y, 0.0));
                    has_vertex = true;
                }
                "  0" => {
                    if value == "SEQEND" {
                        break;
                    }
                }
                _ => {
                    if !has_vertex && code == "  0" {
                        self.current_index -= 2;
                        break;
                    }
                }
            }
        }
        
        if !vertices.is_empty() {
            let polyline = Polyline::from_points(&vertices);
            let entity = Entity::new(
                EntityType::Polyline,
                EntityGeometry::Polyline(polyline),
            );
            
            if let Some(ref mut block) = self.current_block.as_mut().and_then(|name| self.blocks.get_mut(name)) {
                block.add_entity(entity);
            } else {
                self.entities.push(entity);
            }
        }
    }

    fn parse_spline(&mut self) {
        let mut degree = 3;
        let mut knots = Vec::new();
        let mut control_points = Vec::new();
        
        while let Some((code, value)) = self.next_pair() {
            match code {
                " 70" => degree = value.parse().unwrap_or(3),
                " 40" => knots.push(value.parse().unwrap_or(0.0)),
                " 10" => {
                    if let (Some(x), Some(y), Some(z)) = (
                        value.parse().ok(),
                        self.lines.get(self.current_index).and_then(|s| s.parse().ok()),
                        self.lines.get(self.current_index + 1).and_then(|s| s.parse().ok())
                    ) {
                        control_points.push(Point::new(x, y, z));
                        self.current_index += 2;
                    }
                }
                _ => {
                    if code.starts_with("  0") || code.starts_with("  8") {
                        self.current_index -= 2;
                        break;
                    }
                }
            }
        }
        
        if control_points.len() >= 2 && knots.len() >= control_points.len() + degree + 1 {
            let spline = BSpline::new(control_points, knots, degree);
            let entity = Entity::new(
                EntityType::BSpline,
                EntityGeometry::BSpline(spline),
            );
            
            if let Some(ref mut block) = self.current_block.as_mut().and_then(|name| self.blocks.get_mut(name)) {
                block.add_entity(entity);
            } else {
                self.entities.push(entity);
            }
        }
    }

    fn parse_text(&mut self, layer: &str) {
        let mut content = String::new();
        let position = self.parse_coordinate(" 10", " 20", " 30");
        let mut height = 2.5;
        let mut rotation = 0.0;
        
        while let Some((code, value)) = self.next_pair() {
            match code {
                "  1" => content = value.to_string(),
                " 40" => height = value.parse().unwrap_or(2.5),
                " 50" => rotation = value.parse::<f64>().unwrap_or(0.0).to_radians(),
                _ => {
                    if code.starts_with("  0") || code.starts_with("  8") {
                        self.current_index -= 2;
                        break;
                    }
                }
            }
        }
        
        let entity = Entity::new(
            EntityType::Text,
            EntityGeometry::Text {
                content,
                position,
                height,
                rotation,
                width_factor: 1.0,
                font_name: "Standard".to_string(),
                style: TextStyle {
                    bold: false,
                    italic: false,
                    underline: false,
                    alignment: TextAlignment::Left,
                },
            },
        );
        
        if let Some(ref mut block) = self.current_block.as_mut().and_then(|name| self.blocks.get_mut(name)) {
            block.add_entity(entity);
        } else {
            self.entities.push(entity);
        }
    }

    fn parse_dimension(&mut self, layer: &str) {
        let mut dim_type = 0;
        let mut text = String::new();
        let mut insertion_point = Point::origin();
        let mut text_height = 2.5;
        let mut rotation = 0.0;
        
        while let Some((code, value)) = self.next_pair() {
            match code {
                " 70" => dim_type = value.parse().unwrap_or(0),
                "  1" => text = value.to_string(),
                " 15" => {
                    insertion_point.x = value.parse().unwrap_or(0.0);
                }
                " 25" => {
                    insertion_point.y = value.parse().unwrap_or(0.0);
                }
                " 40" => text_height = value.parse().unwrap_or(2.5),
                " 50" => rotation = value.parse::<f64>().unwrap_or(0.0).to_radians(),
                _ => {
                    if code.starts_with("  0") || code.starts_with("  8") {
                        self.current_index -= 2;
                        break;
                    }
                }
            }
        }
        
        let entity = Entity::new(
            EntityType::Dimension,
            EntityGeometry::Text {
                content: text,
                position: insertion_point,
                height: text_height,
                rotation,
                width_factor: 1.0,
                font_name: "Standard".to_string(),
                style: TextStyle {
                    bold: false,
                    italic: false,
                    underline: false,
                    alignment: TextAlignment::Left,
                },
            },
        );
        
        if let Some(ref mut block) = self.current_block.as_mut().and_then(|name| self.blocks.get_mut(name)) {
            block.add_entity(entity);
        } else {
            self.entities.push(entity);
        }
    }

    fn parse_block_definition(&mut self) {
        let mut block_name = String::new();
        let mut base_point = Point::origin();
        
        while let Some((code, value)) = self.next_pair() {
            match code {
                "  2" => block_name = value.to_string(),
                " 10" => {
                    if let (Some(x), Some(y), Some(z)) = (
                        value.parse().ok(),
                        self.lines.get(self.current_index).and_then(|s| s.parse().ok()),
                        self.lines.get(self.current_index + 1).and_then(|s| s.parse().ok())
                    ) {
                        base_point = Point::new(x, y, z);
                        self.current_index += 2;
                    }
                }
                "  0" => {
                    if value == "BLOCK" {
                        continue;
                    } else {
                        self.current_index -= 2;
                        break;
                    }
                }
                _ => {
                    if code.starts_with("ENDSEC") || (code.starts_with("  0") && value != "ENTITY") {
                        self.current_index -= 2;
                        break;
                    }
                }
            }
        }
        
        if !block_name.is_empty() {
            let mut block = Block::new(block_name.clone());
            block.set_origin(base_point);
            self.blocks.insert(block_name.clone(), block);
            self.current_block = Some(block_name);
        }
    }

    fn parse_block_reference(&mut self, layer: &str) {
        let mut block_name = String::new();
        let position = self.parse_coordinate(" 10", " 20", " 30");
        let mut x_scale = 1.0;
        let mut y_scale = 1.0;
        let mut z_scale = 1.0;
        let mut rotation = 0.0;
        let mut column_count = 1u32;
        let mut row_count = 1u32;
        let mut column_spacing = 0.0;
        let mut row_spacing = 0.0;
        
        while let Some((code, value)) = self.next_pair() {
            match code {
                "  2" => block_name = value.to_string(),
                " 50" => rotation = value.parse::<f64>().unwrap_or(0.0).to_radians(),
                " 44" => x_scale = value.parse().unwrap_or(1.0),
                " 45" => y_scale = value.parse().unwrap_or(1.0),
                " 46" => z_scale = value.parse().unwrap_or(1.0),
                " 70" => column_count = value.parse().unwrap_or(1),
                " 71" => row_count = value.parse().unwrap_or(1),
                " 91" => column_spacing = value.parse().unwrap_or(0.0),
                " 92" => row_spacing = value.parse().unwrap_or(0.0),
                _ => {
                    if code.starts_with("  0") || code.starts_with("  8") {
                        self.current_index -= 2;
                        break;
                    }
                }
            }
        }
        
        let entity = Entity::new(
            EntityType::BlockRef,
            EntityGeometry::BlockRef {
                block_name: block_name.clone(),
                position,
                scale_x: x_scale,
                scale_y: y_scale,
                scale_z: z_scale,
                rotation,
                column_count,
                row_count,
                column_spacing,
                row_spacing,
            },
        );
        
        if let Some(ref mut block) = self.current_block.as_mut().and_then(|name| self.blocks.get_mut(name)) {
            block.add_entity(entity);
        } else {
            self.entities.push(entity);
        }
    }

    fn parse_insert_entity(&mut self, layer: &str) {
        self.parse_block_reference(layer);
    }

    fn parse_hatch(&mut self, layer: &str) {
        while let Some((code, value)) = self.next_pair() {
            if code.starts_with("  0") {
                self.current_index -= 2;
                break;
            }
        }
    }

    fn parse_section(&mut self) {
        let mut section_name = String::new();
        let mut current_layer = "0".to_string();
        
        while let Some((code, value)) = self.next_pair() {
            if code == "  2" {
                section_name = value.to_string();
                break;
            }
        }
        
        match section_name.as_str() {
            "HEADER" => {
                while let Some((code, value)) = self.next_pair() {
                    if code == "ENDSEC" {
                        break;
                    }
                }
            }
            "LAYERS" | "LAYER" => {
                while let Some((code, value)) = self.next_pair() {
                    if code == "ENDSEC" {
                        break;
                    }
                    if code == "  0" && value == "LAYER" {
                        self.parse_layer();
                    }
                }
            }
            "BLOCKS" | "BLOCK" => {
                while let Some((code, value)) = self.next_pair() {
                    if code == "ENDSEC" {
                        break;
                    }
                    if code == "  0" && value == "BLOCK" {
                        self.parse_block_definition();
                    }
                }
                self.current_block = None;
            }
            "ENTITIES" | "OBJECTS" => {
                while let Some((code, value)) = self.next_pair() {
                    if code == "ENDSEC" {
                        break;
                    }
                    
                    match value {
                        "LINE" => self.parse_line(&current_layer),
                        "CIRCLE" => self.parse_circle(&current_layer),
                        "ARC" => self.parse_arc(&current_layer),
                        "ELLIPSE" => self.parse_ellipse(&current_layer),
                        "LWPOLYLINE" | "POLYLINE" => {
                            if value == "LWPOLYLINE" {
                                self.parse_lwpolyline(&current_layer);
                            } else {
                                self.parse_polyline(&current_layer);
                            }
                        }
                        "SPLINE" => self.parse_spline(),
                        "TEXT" | "MTEXT" => self.parse_text(&current_layer),
                        "DIMENSION" => self.parse_dimension(&current_layer),
                        "INSERT" => self.parse_insert_entity(&current_layer),
                        "HATCH" => self.parse_hatch(&current_layer),
                        _ => {
                            if code.starts_with("ENDSEC") {
                                break;
                            }
                        }
                    }
                }
            }
            _ => {
                while let Some((code, _)) = self.next_pair() {
                    if code == "ENDSEC" {
                        break;
                    }
                }
            }
        }
    }

    fn parse(&mut self) {
        let mut current_layer = "0".to_string();
        
        while let Some((code, value)) = self.next_pair() {
            if code == "SECTION" {
                self.parse_section();
            }
        }
    }

    fn get_document(self) -> Document {
        let mut doc = Document::new("Imported from DXF".to_string());
        
        for (name, layer_id) in self.layers {
            let mut layer = Layer::new(name.clone());
            doc.add_layer(layer);
        }
        
        for (_, block) in self.blocks {
            doc.add_block(block);
        }
        
        for entity in self.entities {
            doc.add_entity(entity);
        }
        
        doc
    }
}

pub struct DXFImporter;

impl DXFImporter {
    pub fn new() -> Self {
        Self
    }
}

impl crate::io::Importer for DXFImporter {
    fn can_import(&self, extension: &str) -> bool {
        extension.to_lowercase() == "dxf"
    }

    fn import_from_file(&self, filename: &str) -> Result<Document, ImportError> {
        let content = std::fs::read_to_string(filename)
            .map_err(|e| ImportError::Io(e.to_string()))?;
        self.import_from_bytes(content.as_bytes(), "dxf")
    }

    fn import_from_bytes(&self, data: &[u8], extension: &str) -> Result<Document, ImportError> {
        if !self.can_import(extension) {
            return Err(ImportError::UnsupportedFormat(extension.to_string()));
        }

        let content = String::from_utf8_lossy(data);
        let lines: Vec<&str> = content.lines().collect();
        
        let mut parser = DXFParser::new(lines);
        parser.parse();
        
        Ok(parser.get_document())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dxf_version_parsing() {
        let header = "$ACADVER\n  1\nAC1015";
        let version = DXFVersion::from_header(header);
        assert_eq!(version, Some(DXFVersion::R2000));
    }

    #[test]
    fn test_dxf_line_write() {
        let writer = DXFWriter::new();
        let lines = writer.write_line(
            (0.0, 0.0, 0.0),
            (1.0, 1.0, 0.0),
            "Layer1",
        );
        
        assert!(lines.contains(&"LINE".to_string()));
        assert!(lines.contains(&"Layer1".to_string()));
    }

    #[test]
    fn test_dxf_importer_can_import() {
        let importer = DXFImporter::new();
        assert!(importer.can_import("dxf"));
        assert!(importer.can_import("DXF"));
        assert!(!importer.can_import("svg"));
    }
}
