use crate::data_structure::{Entity, EntityGeometry};
use crate::geometry::{Point, Line, Circle, Arc, Ellipse, Polyline};
use crate::render::viewport::Viewport;
use crate::math::Vector2;

#[derive(Debug, Clone, PartialEq)]
pub struct RenderStyle {
    pub color: (u8, u8, u8),
    pub line_width: f64,
    pub line_pattern: LinePattern,
    pub fill: Option<(u8, u8, u8)>,
    pub anti_aliasing: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LinePattern {
    Solid,
    Dashed,
    Dotted,
    DashDot,
    Custom(Vec<f64>),
}

impl Default for RenderStyle {
    fn default() -> Self {
        Self {
            color: (0, 0, 0),
            line_width: 1.0,
            line_pattern: LinePattern::Solid,
            fill: None,
            anti_aliasing: true,
        }
    }
}

pub trait Renderer {
    fn initialize(&mut self) -> Result<(), String>;
    fn clear(&mut self, color: (u8, u8, u8));
    fn draw_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, style: &RenderStyle);
    fn draw_circle(&mut self, cx: f64, cy: f64, r: f64, style: &RenderStyle);
    fn draw_arc(&mut self, cx: f64, cy: f64, r: f64, start_angle: f64, end_angle: f64, style: &RenderStyle);
    fn draw_polyline(&mut self, points: &[(f64, f64)], style: &RenderStyle);
    fn draw_point(&mut self, x: f64, y: f64, size: f64, style: &RenderStyle);
    fn draw_text(&mut self, x: f64, y: f64, text: &str, size: f64, rotation: f64, style: &RenderStyle);
    fn draw_text_enhanced(
        &mut self,
        x: f64,
        y: f64,
        text: &str,
        font_name: &str,
        size: f64,
        rotation: f64,
        width_factor: f64,
        style: &RenderStyle,
        bold: bool,
        italic: bool,
        underline: bool,
        alignment: i32,
    );
    fn present(&mut self);
    fn flush(&mut self);
}

pub struct RenderBuffer {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
}

impl RenderBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; width * height],
        }
    }

    pub fn clear(&mut self, color: (u8, u8, u8)) {
        let color_value = ((color.0 as u32) << 16) | ((color.1 as u32) << 8) | (color.2 as u32);
        for pixel in &mut self.pixels {
            *pixel = color_value;
        }
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: (u8, u8, u8)) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = if dx > dy { dx } else { dy } as i32 / 2;
        
        let mut x = x0;
        let mut y = y0;
        
        while x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.set_pixel(x as usize, y as usize, color);
            
            if x == x1 && y == y1 {
                break;
            }
            
            let e2 = 2 * err;
            if e2 > -dx {
                err -= dx;
                x += sx;
            }
            if e2 < dy {
                err += dy;
                y += sy;
            }
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: (u8, u8, u8)) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.pixels[idx] = ((color.0 as u32) << 16) | ((color.1 as u32) << 8) | (color.2 as u32);
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }
}

pub struct RenderQueue {
    viewport: Viewport,
    entities: Vec<(Entity, RenderStyle)>,
}

impl RenderQueue {
    pub fn new(viewport: Viewport) -> Self {
        Self {
            viewport,
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity, style: RenderStyle) {
        self.entities.push((entity, style));
    }

    pub fn clear(&mut self) {
        self.entities.clear();
    }

    pub fn sort_by_layer(&mut self) {
        self.entities.sort_by_key(|(e, _): &(Entity, RenderStyle)| {
            e.layer_id().to_string()
        });
    }

    pub fn entities(&self) -> &[(Entity, RenderStyle)] {
        &self.entities
    }
}

pub struct EntityRenderer;

impl EntityRenderer {
    pub fn render_line(line: &Line, viewport: &Viewport, style: &RenderStyle) {
        let start = line.start;
        let end = line.end;
        
        let (sx1, sy1) = viewport.world_to_screen(start.x, start.y);
        let (sx2, sy2) = viewport.world_to_screen(end.x, end.y);
        
        println!("Line: ({:.1}, {:.1}) -> ({:.1}, {:.1})", sx1, sy1, sx2, sy2);
    }

    pub fn render_circle(circle: &Circle, viewport: &Viewport, style: &RenderStyle) {
        let (cx, cy) = viewport.world_to_screen(circle.center.x, circle.center.y);
        
        let (min_x, min_y) = viewport.world_to_screen(
            circle.center.x - circle.radius,
            circle.center.y - circle.radius,
        );
        let (max_x, _) = viewport.world_to_screen(
            circle.center.x + circle.radius,
            circle.center.y + circle.radius,
        );
        
        let radius = ((max_x - min_x).abs() / 2.0).abs();
        
        println!("Circle: center=({:.1}, {:.1}), radius={:.1}", cx, cy, radius);
    }

    pub fn render_arc(arc: &Arc, viewport: &Viewport, style: &RenderStyle) {
        let (cx, cy) = viewport.world_to_screen(arc.center.x, arc.center.y);
        
        let start = arc.point_at_angle(arc.start_angle);
        let end = arc.point_at_angle(arc.end_angle);
        let (sx1, sy1) = viewport.world_to_screen(start.x, start.y);
        let (sx2, sy2) = viewport.world_to_screen(end.x, end.y);
        
        println!("Arc: center=({:.1}, {:.1}), start=({:.1}, {:.1}), end=({:.1}, {:.1})", 
                 cx, cy, sx1, sy1, sx2, sy2);
    }

    pub fn render_polyline(polyline: &Polyline, viewport: &Viewport, style: &RenderStyle) {
        let mut path = String::new();
        for (i, vertex) in polyline.vertices.iter().enumerate() {
            let (sx, sy) = viewport.world_to_screen(vertex.x, vertex.y);
            if i > 0 {
                path.push_str(" -> ");
            }
            path.push_str(&format!("({:.1}, {:.1})", sx, sy));
        }
        println!("Polyline: {}", path);
    }

    pub fn render(entity: &Entity, viewport: &Viewport, style: &RenderStyle) {
        match entity.geometry() {
            crate::data_structure::EntityGeometry::Line(line) => {
                Self::render_line(line, viewport, style);
            }
            crate::data_structure::EntityGeometry::Circle(circle) => {
                Self::render_circle(circle, viewport, style);
            }
            crate::data_structure::EntityGeometry::Arc(arc) => {
                Self::render_arc(arc, viewport, style);
            }
            crate::data_structure::EntityGeometry::Polyline(polyline) => {
                Self::render_polyline(polyline, viewport, style);
            }
            _ => {
                println!("Entity type not fully supported for rendering yet");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_buffer() {
        let buffer = RenderBuffer::new(800, 600);
        assert_eq!(buffer.width(), 800);
        assert_eq!(buffer.height(), 600);
    }

    #[test]
    fn test_render_style() {
        let style = RenderStyle::default();
        assert_eq!(style.color, (0, 0, 0));
        assert_eq!(style.line_width, 1.0);
    }

    #[test]
    fn test_render_queue() {
        let viewport = Viewport::new();
        let queue = RenderQueue::new(viewport);
        assert!(queue.entities().is_empty());
    }
}
