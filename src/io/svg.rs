use thiserror::Error;
use quick_xml::events::{Event, BytesStart};
use quick_xml::reader::Reader;
use crate::io::io::Error as ImportError;
use crate::data_structure::{Document, Layer, Entity, EntityType, EntityGeometry, TextStyle, TextAlignment};
use crate::geometry::{Point, Line, Circle, Ellipse, Polyline};
use std::collections::HashMap;

#[derive(Debug, Error)]
pub enum SVGError {
    #[error("Failed to create SVG: {0}")]
    CreationError(String),
    
    #[error("Invalid viewport: {0}")]
    InvalidViewport(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Unsupported SVG element: {0}")]
    UnsupportedElement(String),
    
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct SVGStyle {
    pub stroke: Option<String>,
    pub stroke_width: f64,
    pub fill: Option<String>,
    pub opacity: f64,
    pub line_cap: String,
    pub line_join: String,
}

impl Default for SVGStyle {
    fn default() -> Self {
        Self {
            stroke: Some("#000000".to_string()),
            stroke_width: 1.0,
            fill: None,
            opacity: 1.0,
            line_cap: "round".to_string(),
            line_join: "round".to_string(),
        }
    }
}

impl SVGStyle {
    pub fn to_string(&self) -> String {
        let mut style = String::new();
        if let Some(stroke) = &self.stroke {
            style.push_str(&format!("stroke:{};", stroke));
        }
        style.push_str(&format!("stroke-width:{};", self.stroke_width));
        if let Some(fill) = &self.fill {
            style.push_str(&format!("fill:{};", fill));
        } else {
            style.push_str("fill:none;");
        }
        style.push_str(&format!("opacity:{};", self.opacity));
        style.push_str(&format!("stroke-linecap:{};", self.line_cap));
        style.push_str(&format!("stroke-linejoin:{};", self.line_join));
        style
    }
}

pub struct SVGImporter;

impl SVGImporter {
    pub fn new() -> Self {
        Self
    }
}

impl crate::io::Importer for SVGImporter {
    fn can_import(&self, extension: &str) -> bool {
        extension.to_lowercase() == "svg"
    }

    fn import_from_file(&self, filename: &str) -> Result<Document, ImportError> {
        let content = std::fs::read_to_string(filename)
            .map_err(|e| ImportError::Io(e.to_string()))?;
        self.import_from_bytes(content.as_bytes(), "svg")
    }

    fn import_from_bytes(&self, data: &[u8], extension: &str) -> Result<Document, ImportError> {
        if !self.can_import(extension) {
            return Err(ImportError::UnsupportedFormat(extension.to_string()));
        }

        let content = String::from_utf8_lossy(data);
        let mut reader = Reader::from_str(&content);

        let mut doc = Document::new("Imported from SVG".to_string());
        let mut parser = SVGParser::new(&mut doc);
        
        let mut buf = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Eof) => break,
                Ok(Event::Start(ref tag)) => {
                    let tag_name_bytes = tag.name().into_inner();
                    let tag_name = tag_name_bytes.as_ref();
                    if tag_name == b"svg" {
                        parser.parse_svg_element(tag)?;
                    } else {
                        parser.parse_element(tag);
                    }
                }
                Ok(Event::End(_)) => {}
                Ok(Event::Text(_)) => {}
                Ok(Event::Comment(_)) => {}
                Ok(Event::Decl(_)) => {}
                Ok(Event::CData(_)) => {}
                Err(e) => return Err(ImportError::Io(format!("XML parse error: {}", e))),
                _ => {}
            }
            buf.clear();
        }

        Ok(parser.into_document())
    }
}

struct SVGParser<'a> {
    doc: &'a mut Document,
    viewport: (f64, f64, f64, f64),
    svg_width: f64,
    svg_height: f64,
    default_style: SVGStyle,
}

impl<'a> SVGParser<'a> {
    fn new(doc: &'a mut Document) -> Self {
        let mut parser = Self {
            doc,
            viewport: (0.0, 0.0, 100.0, 100.0),
            svg_width: 100.0,
            svg_height: 100.0,
            default_style: SVGStyle::default(),
        };
        
        let default_layer = Layer::new("0".to_string());
        parser.doc.add_layer(default_layer);
        
        parser
    }

    fn into_document(self) -> Document {
        self.doc.clone()
    }

    fn parse_svg_element(&mut self, tag: &BytesStart) -> Result<(), ImportError> {
        for attr in tag.attributes().flatten() {
            let key = attr.key.as_ref();
            let value = attr.value.as_ref();
            match key {
                b"width" => {
                    self.svg_width = parse_length(value);
                }
                b"height" => {
                    self.svg_height = parse_length(value);
                }
                b"viewBox" => {
                    self.parse_viewbox(value);
                }
                _ => {}
            }
        }

        self.viewport = (0.0, 0.0, self.svg_width, self.svg_height);
        Ok(())
    }

    fn parse_viewbox(&mut self, value: &[u8]) {
        let value_str = String::from_utf8_lossy(value);
        let parts: Vec<&str> = value_str.split_whitespace().collect();
        if parts.len() == 4 {
            if let (Ok(min_x), Ok(min_y), Ok(width), Ok(height)) = (
                parts[0].parse::<f64>(),
                parts[1].parse::<f64>(),
                parts[2].parse::<f64>(),
                parts[3].parse::<f64>()
            ) {
                self.viewport = (min_x, min_y, min_x + width, min_y + height);
            }
        }
    }

    fn parse_element(&mut self, tag: &BytesStart) {
        let name_bytes = tag.name().into_inner();
        let name = name_bytes.as_ref();
        let attrs = parse_attributes(tag);

        match name {
            b"rect" => self.parse_rect(&attrs),
            b"circle" => self.parse_circle(&attrs),
            b"ellipse" => self.parse_ellipse(&attrs),
            b"line" => self.parse_line(&attrs),
            b"polyline" => self.parse_polyline(&attrs),
            b"polygon" => self.parse_polygon(&attrs),
            b"path" => self.parse_path(&attrs),
            b"text" => self.parse_text(&attrs),
            b"g" => {},
            _ => {},
        }
    }

    fn parse_rect(&mut self, attrs: &HashMap<String, String>) {
        let x = attrs.get("x").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let y = attrs.get("y").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let width = attrs.get("width").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let height = attrs.get("height").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let rx = attrs.get("rx").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let ry = attrs.get("ry").and_then(|s| s.parse().ok()).unwrap_or(0.0);

        if width > 0.0 && height > 0.0 {
            if rx > 0.0 || ry > 0.0 {
                let rx = if rx == 0.0 { ry } else { rx };
                let ry = if ry == 0.0 { rx } else { ry };
                self.create_rounded_rect(x, y, width, height, rx, ry);
            } else {
                let mut polyline = Polyline::new();
                polyline.push(Point::new(x, y, 0.0));
                polyline.push(Point::new(x + width, y, 0.0));
                polyline.push(Point::new(x + width, y + height, 0.0));
                polyline.push(Point::new(x, y + height, 0.0));
                polyline.close();
                
                let entity = Entity::new(
                    EntityType::Polyline,
                    EntityGeometry::Polyline(polyline),
                );
                self.add_entity(entity);
            }
        }
    }

    fn create_rounded_rect(&mut self, x: f64, y: f64, width: f64, height: f64, rx: f64, ry: f64) {
        let mut polyline = Polyline::new();
        polyline.push(Point::new(x + rx, y, 0.0));
        polyline.push(Point::new(x + width - rx, y, 0.0));
        polyline.push(Point::new(x + width, y, 0.0));
        polyline.push(Point::new(x + width, y + ry, 0.0));
        polyline.push(Point::new(x + width, y + height - ry, 0.0));
        polyline.push(Point::new(x + width, y + height, 0.0));
        polyline.push(Point::new(x + width - rx, y + height, 0.0));
        polyline.push(Point::new(x + rx, y + height, 0.0));
        polyline.push(Point::new(x, y + height, 0.0));
        polyline.push(Point::new(x, y + height - ry, 0.0));
        polyline.push(Point::new(x, y + ry, 0.0));
        polyline.push(Point::new(x, y, 0.0));
        polyline.close();

        let entity = Entity::new(
            EntityType::Polyline,
            EntityGeometry::Polyline(polyline),
        );
        self.add_entity(entity);
    }

    fn parse_circle(&mut self, attrs: &HashMap<String, String>) {
        let cx = attrs.get("cx").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let cy = attrs.get("cy").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let r = attrs.get("r").and_then(|s| s.parse().ok()).unwrap_or(0.0);

        if r > 0.0 {
            let entity = Entity::new(
                EntityType::Circle,
                EntityGeometry::Circle(Circle::new(Point::new(cx, cy, 0.0), r)),
            );
            self.add_entity(entity);
        }
    }

    fn parse_ellipse(&mut self, attrs: &HashMap<String, String>) {
        let cx = attrs.get("cx").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let cy = attrs.get("cy").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let rx = attrs.get("rx").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let ry = attrs.get("ry").and_then(|s| s.parse().ok()).unwrap_or(0.0);

        if rx > 0.0 && ry > 0.0 {
            let entity = Entity::new(
                EntityType::Ellipse,
                EntityGeometry::Ellipse(Ellipse::new(Point::new(cx, cy, 0.0), rx, ry, 0.0)),
            );
            self.add_entity(entity);
        }
    }

    fn parse_line(&mut self, attrs: &HashMap<String, String>) {
        let x1 = attrs.get("x1").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let y1 = attrs.get("y1").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let x2 = attrs.get("x2").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let y2 = attrs.get("y2").and_then(|s| s.parse().ok()).unwrap_or(0.0);

        let entity = Entity::new(
            EntityType::Line,
            EntityGeometry::Line(Line::new(Point::new(x1, y1, 0.0), Point::new(x2, y2, 0.0))),
        );
        self.add_entity(entity);
    }

    fn parse_polyline(&mut self, attrs: &HashMap<String, String>) {
        if let Some(points_str) = attrs.get("points") {
            let points = parse_points(points_str);
            if points.len() >= 2 {
                let mut polyline = Polyline::new();
                for point in points {
                    polyline.push(point);
                }
                
                let entity = Entity::new(
                    EntityType::Polyline,
                    EntityGeometry::Polyline(polyline),
                );
                self.add_entity(entity);
            }
        }
    }

    fn parse_polygon(&mut self, attrs: &HashMap<String, String>) {
        if let Some(points_str) = attrs.get("points") {
            let mut points = parse_points(points_str);
            if points.len() >= 3 {
                points.push(points[0]);
                let mut polyline = Polyline::new();
                for point in points {
                    polyline.push(point);
                }
                polyline.close();
                
                let entity = Entity::new(
                    EntityType::Polyline,
                    EntityGeometry::Polyline(polyline),
                );
                self.add_entity(entity);
            }
        }
    }

    fn parse_path(&mut self, attrs: &HashMap<String, String>) {
        if let Some(d) = attrs.get("d") {
            let path_commands = parse_path_data(d);
            let points = convert_path_to_points(&path_commands);
            if points.len() >= 2 {
                let mut polyline = Polyline::new();
                for point in points {
                    polyline.push(point);
                }
                if polyline.vertices.len() >= 3 {
                    polyline.close();
                }
                
                let entity = Entity::new(
                    EntityType::Polyline,
                    EntityGeometry::Polyline(polyline),
                );
                self.add_entity(entity);
            }
        }
    }

    fn parse_text(&mut self, attrs: &HashMap<String, String>) {
        let x = attrs.get("x").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let y = attrs.get("y").and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let font_size = attrs.get("font-size").and_then(|s| s.parse().ok()).unwrap_or(12.0);
        let content = attrs.get("content").cloned().unwrap_or_else(|| "".to_string());

        if !content.is_empty() {
            let entity = Entity::new(
                EntityType::Text,
                EntityGeometry::Text {
                    content,
                    position: Point::new(x, y, 0.0),
                    height: font_size,
                    rotation: 0.0,
                    width_factor: 1.0,
                    font_name: "Arial".to_string(),
                    style: TextStyle {
                        bold: false,
                        italic: false,
                        underline: false,
                        alignment: TextAlignment::Left,
                    },
                },
            );
            self.add_entity(entity);
        }
    }

    fn add_entity(&mut self, entity: Entity) {
        self.doc.add_entity(entity);
    }
}

fn parse_attributes(tag: &BytesStart) -> HashMap<String, String> {
    let mut attrs = HashMap::new();
    for attr in tag.attributes().flatten() {
        let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
        let value = String::from_utf8_lossy(attr.value.as_ref()).to_string();
        attrs.insert(key, value);
    }
    attrs
}

fn parse_length(value: &[u8]) -> f64 {
    String::from_utf8_lossy(value)
        .trim_end_matches(|c| matches!(c, 'p' | 'x' | 'm' | '%'))
        .parse()
        .unwrap_or(0.0)
}

fn parse_points(s: &str) -> Vec<Point> {
    let mut points = Vec::new();
    let mut current = String::new();
    let mut is_x = true;
    let mut x = 0.0;
    let mut y = 0.0;

    for c in s.chars() {
        if c.is_whitespace() || c == ',' {
            if !current.is_empty() {
                if is_x {
                    x = current.parse().unwrap_or(0.0);
                } else {
                    y = current.parse().unwrap_or(0.0);
                    points.push(Point::new(x, y, 0.0));
                }
                is_x = !is_x;
                current.clear();
            }
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        if is_x {
            x = current.parse().unwrap_or(0.0);
        } else {
            y = current.parse().unwrap_or(0.0);
            points.push(Point::new(x, y, 0.0));
        }
    }

    points
}

#[derive(Debug, Clone)]
enum PathCommand {
    MoveTo(f64, f64),
    LineTo(f64, f64),
    HorizontalTo(f64),
    VerticalTo(f64),
    Close,
}

fn parse_path_data(d: &str) -> Vec<PathCommand> {
    let mut commands = Vec::new();
    let mut current = String::new();
    let mut chars = d.chars().peekable();
    let mut command = b'M';

    while let Some(c) = chars.next() {
        if c.is_alphabetic() {
            if !current.trim().is_empty() {
                let values: Vec<f64> = current.split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                
                process_path_command(command, &values, &mut commands);
            }
            command = c as u8;
            current.clear();
        } else if c.is_whitespace() || c == ',' {
            if !current.is_empty() {
                current.push(c);
            }
        } else {
            current.push(c);
        }
    }

    if !current.trim().is_empty() {
        let values: Vec<f64> = current.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        process_path_command(command, &values, &mut commands);
    }

    commands
}

fn process_path_command(cmd: u8, values: &[f64], commands: &mut Vec<PathCommand>) {
    match cmd {
        b'M' | b'm' => {
            let mut i = 0;
            while i + 1 < values.len() {
                commands.push(PathCommand::MoveTo(values[i], values[i + 1]));
                i += 2;
            }
        }
        b'L' | b'l' => {
            let mut i = 0;
            while i + 1 < values.len() {
                commands.push(PathCommand::LineTo(values[i], values[i + 1]));
                i += 2;
            }
        }
        b'H' | b'h' => {
            for &x in values {
                commands.push(PathCommand::HorizontalTo(x));
            }
        }
        b'V' | b'v' => {
            for &y in values {
                commands.push(PathCommand::VerticalTo(y));
            }
        }
        b'Z' | b'z' => {
            commands.push(PathCommand::Close);
        }
        _ => {}
    }
}

fn convert_path_to_points(commands: &[PathCommand]) -> Vec<Point> {
    let mut points = Vec::new();
    let mut last_x = 0.0;
    let mut last_y = 0.0;

    for cmd in commands {
        match cmd {
            PathCommand::MoveTo(x, y) => {
                last_x = *x;
                last_y = *y;
                points.push(Point::new(last_x, last_y, 0.0));
            }
            PathCommand::LineTo(x, y) => {
                last_x = *x;
                last_y = *y;
                points.push(Point::new(last_x, last_y, 0.0));
            }
            PathCommand::HorizontalTo(x) => {
                last_x = *x;
                points.push(Point::new(last_x, last_y, 0.0));
            }
            PathCommand::VerticalTo(y) => {
                last_y = *y;
                points.push(Point::new(last_x, last_y, 0.0));
            }
            PathCommand::Close => {
                if let Some(first) = points.first() {
                    points.push(Point::new(first.x, first.y, 0.0));
                }
            }
        }
    }

    points
}

#[derive(Debug, Clone)]
pub struct SVGWriter {
    width: f64,
    height: f64,
    viewport: (f64, f64, f64, f64),
    content: Vec<String>,
}

impl SVGWriter {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            viewport: (0.0, 0.0, width, height),
            content: Vec::new(),
        }
    }

    pub fn set_viewport(&mut self, min_x: f64, min_y: f64, max_x: f64, max_y: f64) {
        self.viewport = (min_x, min_y, max_x, max_y);
    }

    pub fn to_svg_coordinates(&self, x: f64, y: f64) -> (f64, f64) {
        let (min_x, min_y, max_x, max_y) = self.viewport;
        let svg_x = (x - min_x) / (max_x - min_x) * self.width;
        let svg_y = self.height - (y - min_y) / (max_y - min_y) * self.height;
        (svg_x, svg_y)
    }

    pub fn write_header(&mut self) {
        self.content.push(format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{:.2}" height="{:.2}">"#,
            self.width, self.height
        ));
        self.content.push(format!(
            r#"<rect width="100%" height="100%" fill="white"/>"#
        ));
    }

    pub fn write_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, style: &SVGStyle) {
        let (x1, y1) = self.to_svg_coordinates(x1, y1);
        let (x2, y2) = self.to_svg_coordinates(x2, y2);
        
        self.content.push(format!(
            r#"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" style="{}"/>"#,
            x1, y1, x2, y2, style.to_string()
        ));
    }

    pub fn write_circle(&mut self, cx: f64, cy: f64, r: f64, style: &SVGStyle) {
        let (cx, cy) = self.to_svg_coordinates(cx, cy);
        
        self.content.push(format!(
            r#"<circle cx="{:.2}" cy="{:.2}" r="{:.2}" style="{}"/>"#,
            cx, cy, r, style.to_string()
        ));
    }

    pub fn write_arc(&mut self, cx: f64, cy: f64, r: f64, start_angle: f64, end_angle: f64, style: &SVGStyle) {
        let (cx, cy) = self.to_svg_coordinates(cx, cy);
        
        let start_rad = -start_angle;
        let end_rad = -end_angle;
        
        let x1 = cx + r * start_rad.cos();
        let y1 = cy + r * start_rad.sin();
        let x2 = cx + r * end_rad.cos();
        let y2 = cy + r * end_rad.sin();
        
        let large_arc = if (end_angle - start_angle).abs() > std::f64::consts::PI { 1 } else { 0 };
        let sweep = if start_angle < end_angle { 1 } else { 0 };
        
        self.content.push(format!(
            r#"<path d="M {:.2} {:.2} A {:.2} {:.2} 0 {} {} {:.2} {:.2}" style="{}"/>"#,
            x1, y1, r, r, large_arc, sweep, x2, y2, style.to_string()
        ));
    }

    pub fn write_polyline(&mut self, points: &[(f64, f64)], style: &SVGStyle) {
        if points.is_empty() {
            return;
        }

        let coords: Vec<String> = points
            .iter()
            .map(|(x, y)| {
                let (svg_x, svg_y) = self.to_svg_coordinates(*x, *y);
                format!("{:.2},{:.2}", svg_x, svg_y)
            })
            .collect();
        
        self.content.push(format!(
            r#"<polyline points="{}" style="{}"/>"#,
            coords.join(" "),
            style.to_string()
        ));
    }

    pub fn write_text(&mut self, x: f64, y: f64, text: &str, font_size: f64, color: &str) {
        let (svg_x, svg_y) = self.to_svg_coordinates(x, y);
        
        self.content.push(format!(
            r#"<text x="{:.2}" y="{:.2}" font-size="{:.2}" fill="{}">{}</text>"#,
            svg_x, svg_y, font_size, color, text
        ));
    }

    pub fn write_grid(&mut self, spacing: f64, color: &str) {
        let (min_x, min_y, max_x, max_y) = self.viewport;
        
        let mut x = min_x;
        while x <= max_x {
            self.write_line(x, min_y, x, max_y, &SVGStyle {
                stroke: Some(color.to_string()),
                stroke_width: 0.5,
                ..Default::default()
            });
            x += spacing;
        }
        
        let mut y = min_y;
        while y <= max_y {
            self.write_line(min_x, y, max_x, y, &SVGStyle {
                stroke: Some(color.to_string()),
                stroke_width: 0.5,
                ..Default::default()
            });
            y += spacing;
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for line in &self.content {
            result.push_str(line);
            result.push('\n');
        }
        result
    }

    pub fn save(&self, filename: &str) -> Result<(), SVGError> {
        let mut file = std::fs::File::create(filename).map_err(|e| SVGError::IOError(e))?;
        
        let mut svg_content = String::new();
        svg_content.push_str(&format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{:.2}" height="{:.2}">
"#,
            self.width, self.height
        ));
        svg_content.push_str(&self.to_string());
        svg_content.push_str("</svg>");
        
        use std::io::Write;
        file.write_all(svg_content.as_bytes()).map_err(|e| SVGError::IOError(e))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_writer_creation() {
        let writer = SVGWriter::new(800.0, 600.0);
        assert_eq!(writer.width, 800.0);
        assert_eq!(writer.height, 600.0);
    }

    #[test]
    fn test_svg_line_write() {
        let mut writer = SVGWriter::new(800.0, 600.0);
        writer.set_viewport(0.0, 0.0, 100.0, 100.0);
        
        let style = SVGStyle::default();
        writer.write_line(0.0, 0.0, 100.0, 100.0, &style);
        
        let svg = writer.to_string();
        assert!(svg.contains("<line"));
        assert!(svg.contains("x1=\"0.00\""));
    }

    #[test]
    fn test_svg_circle_write() {
        let mut writer = SVGWriter::new(800.0, 600.0);
        writer.set_viewport(0.0, 0.0, 100.0, 100.0);
        
        let style = SVGStyle::default();
        writer.write_circle(50.0, 50.0, 25.0, &style);
        
        let svg = writer.to_string();
        assert!(svg.contains("<circle"));
    }

    #[test]
    fn test_svg_importer_basic_circle() {
        let importer = SVGImporter;
        let svg_content = r#"<?xml version="1.0"?>
<svg width="100" height="100" viewBox="0 0 100 100">
    <circle cx="50" cy="50" r="25" fill="none" stroke="black" stroke-width="2"/>
</svg>"#;
        
        let result = importer.import_from_bytes(svg_content.as_bytes(), "svg");
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert!(doc.entity_count() > 0);
    }

    #[test]
    fn test_svg_importer_multiple_elements() {
        let importer = SVGImporter;
        let svg_content = r#"<?xml version="1.0"?>
<svg width="200" height="200" viewBox="0 0 200 200">
    <rect x="10" y="10" width="50" height="30" fill="blue"/>
    <circle cx="100" cy="100" r="40" stroke="red" stroke-width="3"/>
    <line x1="150" y1="150" x2="180" y2="180" stroke="green" stroke-width="2"/>
</svg>"#;
        
        let result = importer.import_from_bytes(svg_content.as_bytes(), "svg");
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.entity_count(), 3);
    }

    #[test]
    fn test_svg_importer_polyline() {
        let importer = SVGImporter;
        let svg_content = r#"<?xml version="1.0"?>
<svg width="100" height="100">
    <polyline points="10,10 20,20 30,10 40,20" fill="none" stroke="black"/>
</svg>"#;
        
        let result = importer.import_from_bytes(svg_content.as_bytes(), "svg");
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert!(doc.entity_count() > 0);
    }

    #[test]
    fn test_svg_importer_path() {
        let importer = SVGImporter;
        let svg_content = r#"<?xml version="1.0"?>
<svg width="100" height="100">
    <path d="M10,10 L20,20 L30,10 Z" fill="none" stroke="black"/>
</svg>"#;
        
        let result = importer.import_from_bytes(svg_content.as_bytes(), "svg");
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert!(doc.entity_count() > 0);
    }

    #[test]
    fn test_svg_importer_polygon() {
        let importer = SVGImporter;
        let svg_content = r#"<?xml version="1.0"?>
<svg width="100" height="100">
    <polygon points="50,10 90,90 10,90" fill="yellow" stroke="black"/>
</svg>"#;
        
        let result = importer.import_from_bytes(svg_content.as_bytes(), "svg");
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert!(doc.entity_count() > 0);
    }

    #[test]
    fn test_svg_importer_ellipse() {
        let importer = SVGImporter;
        let svg_content = r#"<?xml version="1.0"?>
<svg width="100" height="100">
    <ellipse cx="50" cy="50" rx="40" ry="20" fill="lightblue"/>
</svg>"#;
        
        let result = importer.import_from_bytes(svg_content.as_bytes(), "svg");
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert!(doc.entity_count() > 0);
    }

    #[test]
    fn test_svg_importer_empty_svg() {
        let importer = SVGImporter;
        let svg_content = r#"<?xml version="1.0"?>
<svg width="100" height="100">
</svg>"#;
        
        let result = importer.import_from_bytes(svg_content.as_bytes(), "svg");
        assert!(result.is_ok());
        let doc = result.unwrap();
        assert_eq!(doc.entity_count(), 0);
    }

    #[test]
    fn test_parse_points() {
        let points = parse_points("10,20 30,40 50,60");
        assert_eq!(points.len(), 3);
        assert_eq!(points[0].x, 10.0);
        assert_eq!(points[0].y, 20.0);
        assert_eq!(points[1].x, 30.0);
        assert_eq!(points[1].y, 40.0);
    }

    #[test]
    fn test_parse_path_data() {
        let commands = parse_path_data("M10,10 L20,20 L30,10 Z");
        assert!(!commands.is_empty());
        if let Some(first) = commands.first() {
            match first {
                PathCommand::MoveTo(x, y) => {
                    assert_eq!(*x, 10.0);
                    assert_eq!(*y, 10.0);
                }
                _ => panic!("Expected MoveTo"),
            }
        }
    }
}
