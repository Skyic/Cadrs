use crate::geometry::Point;
use crate::math::Vector2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewRect {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl ViewRect {
    pub fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        Self { min_x, min_y, max_x, max_y }
    }

    pub fn width(&self) -> f64 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> f64 {
        self.max_y - self.min_y
    }

    pub fn center(&self) -> (f64, f64) {
        ((self.min_x + self.max_x) / 2.0, (self.min_y + self.max_y) / 2.0)
    }

    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    pub fn expand(&self, factor: f64) -> Self {
        let cx = (self.min_x + self.max_x) / 2.0;
        let cy = (self.min_y + self.max_y) / 2.0;
        let hw = self.width() / 2.0 * factor;
        let hh = self.height() / 2.0 * factor;
        
        Self::new(cx - hw, cy - hh, cx + hw, cy + hh)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    pub origin: (f64, f64),
    pub size: (f64, f64),
    pub zoom: f64,
    pub rotation: f64,
    pub view_rect: ViewRect,
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            origin: (0.0, 0.0),
            size: (800.0, 600.0),
            zoom: 1.0,
            rotation: 0.0,
            view_rect: ViewRect::new(0.0, 0.0, 800.0, 600.0),
        }
    }

    pub fn with_size(width: f64, height: f64) -> Self {
        let view_rect = ViewRect::new(0.0, 0.0, width, height);
        Self {
            origin: (0.0, 0.0),
            size: (width, height),
            zoom: 1.0,
            rotation: 0.0,
            view_rect,
        }
    }

    pub fn set_view(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.view_rect = ViewRect::new(x, y, x + width, y + height);
    }

    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.origin.0 += dx;
        self.origin.1 += dy;
        self.view_rect.min_x += dx;
        self.view_rect.max_x += dx;
        self.view_rect.min_y += dy;
        self.view_rect.max_y += dy;
    }

    pub fn zoom_at(&mut self, factor: f64, center_x: f64, center_y: f64) {
        let before_width = self.view_rect.width();
        let before_height = self.view_rect.height();
        
        self.zoom *= factor;
        
        let after_width = before_width / factor;
        let after_height = before_height / factor;
        
        let cx = (self.view_rect.min_x + self.view_rect.max_x) / 2.0;
        let cy = (self.view_rect.min_y + self.view_rect.max_y) / 2.0;
        
        self.view_rect.min_x = cx - after_width / 2.0;
        self.view_rect.max_x = cx + after_width / 2.0;
        self.view_rect.min_y = cy - after_height / 2.0;
        self.view_rect.max_y = cy + after_height / 2.0;
    }

    pub fn zoom_extents(&mut self, rect: ViewRect) {
        let padding = 1.1;
        let width = rect.width() * padding;
        let height = rect.height() * padding;
        
        let cx = (rect.min_x + rect.max_x) / 2.0;
        let cy = (rect.min_y + rect.max_y) / 2.0;
        
        self.view_rect.min_x = cx - width / 2.0;
        self.view_rect.max_x = cx + width / 2.0;
        self.view_rect.min_y = cy - height / 2.0;
        self.view_rect.max_y = cy + height / 2.0;
    }

    pub fn world_to_screen(&self, x: f64, y: f64) -> (f64, f64) {
        let vw = self.view_rect.width();
        let vh = self.view_rect.height();
        let sw = self.size.0;
        let sh = self.size.1;
        
        let screen_x = (x - self.view_rect.min_x) / vw * sw;
        let screen_y = sh - (y - self.view_rect.min_y) / vh * sh;
        
        (screen_x, screen_y)
    }

    pub fn screen_to_world(&self, x: f64, y: f64) -> (f64, f64) {
        let vw = self.view_rect.width();
        let vh = self.view_rect.height();
        let sw = self.size.0;
        let sh = self.size.1;
        
        let world_x = self.view_rect.min_x + x / sw * vw;
        let world_y = self.view_rect.min_y + (sh - y) / sh * vh;
        
        (world_x, world_y)
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_rect() {
        let rect = ViewRect::new(0.0, 0.0, 100.0, 100.0);
        assert_eq!(rect.width(), 100.0);
        assert_eq!(rect.height(), 100.0);
        assert!(rect.contains(50.0, 50.0));
        assert!(!rect.contains(150.0, 150.0));
    }

    #[test]
    fn test_viewport_world_to_screen() {
        let mut viewport = Viewport::with_size(800.0, 600.0);
        viewport.set_view(0.0, 0.0, 100.0, 100.0);
        
        let (sx, sy) = viewport.world_to_screen(50.0, 50.0);
        assert!((sx - 400.0).abs() < 1e-10);
        assert!((sy - 300.0).abs() < 1e-10);
    }

    #[test]
    fn test_viewport_pan() {
        let mut viewport = Viewport::new();
        viewport.pan(100.0, 50.0);
        
        assert_eq!(viewport.origin.0, 100.0);
        assert_eq!(viewport.origin.1, 50.0);
    }
}
