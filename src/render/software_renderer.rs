use crate::geometry::{Point, Line, Circle, Arc, Ellipse, Polyline};
use crate::render::{RenderBackend, RenderBuffer, RenderStyle};
use super::render_backend::BackendCapabilities;

pub struct SoftwareRenderer {
    width: usize,
    height: usize,
    buffer: RenderBuffer,
}

impl SoftwareRenderer {
    pub fn new() -> Self {
        Self {
            width: 800,
            height: 600,
            buffer: RenderBuffer::new(800, 600),
        }
    }

    pub fn with_size(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: RenderBuffer::new(width, height),
        }
    }
}

impl RenderBackend for SoftwareRenderer {
    fn name(&self) -> &str {
        "Software"
    }

    fn is_available(&self) -> bool {
        true
    }

    fn initialize(&mut self) -> Result<(), String> {
        self.buffer = RenderBuffer::new(self.width, self.height);
        Ok(())
    }

    fn set_size(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.buffer = RenderBuffer::new(width, height);
    }

    fn clear(&mut self, color: (u8, u8, u8)) {
        self.buffer.clear(color);
    }

    fn present(&mut self) {
    }

    fn get_framebuffer(&mut self) -> Option<&mut RenderBuffer> {
        Some(&mut self.buffer)
    }

    fn get_capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            max_texture_size: 4096,
            supports_shaders: false,
            supports_anti_aliasing: false,
            supports_hardware_acceleration: false,
            max_vertex_count: 1000000,
            supports_geometry_shader: false,
            supports_compute_shader: false,
        }
    }
}

impl Default for SoftwareRenderer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SVGRenderer {
    width: usize,
    height: usize,
    svg_content: String,
}

impl SVGRenderer {
    pub fn new() -> Self {
        Self {
            width: 800,
            height: 600,
            svg_content: String::new(),
        }
    }

    pub fn with_size(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            svg_content: format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#, width, height),
        }
    }

    pub fn finalize(&mut self) {
        self.svg_content.push_str("</svg>");
    }

    pub fn get_svg(&self) -> &str {
        &self.svg_content
    }
}

impl RenderBackend for SVGRenderer {
    fn name(&self) -> &str {
        "SVG"
    }

    fn is_available(&self) -> bool {
        true
    }

    fn initialize(&mut self) -> Result<(), String> {
        self.svg_content = format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#, self.width, self.height);
        Ok(())
    }

    fn set_size(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.svg_content = format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#, width, height);
    }

    fn clear(&mut self, color: (u8, u8, u8)) {
        self.svg_content.push_str(&format!(
            r#"<rect width="100%" height="100%" fill="rgb({},{},{})"/>"#,
            color.0, color.1, color.2
        ));
    }

    fn present(&mut self) {
        self.finalize();
    }

    fn get_framebuffer(&mut self) -> Option<&mut RenderBuffer> {
        None
    }

    fn get_capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            max_texture_size: 4096,
            supports_shaders: false,
            supports_anti_aliasing: false,
            supports_hardware_acceleration: false,
            max_vertex_count: 1000000,
            supports_geometry_shader: false,
            supports_compute_shader: false,
        }
    }
}

impl Default for SVGRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_arch = "wasm32")]
pub struct WebGLRenderer;

#[cfg(target_arch = "wasm32")]
impl WebGLRenderer {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(target_arch = "wasm32")]
impl RenderBackend for WebGLRenderer {
    fn name(&self) -> &str {
        "WebGL"
    }

    fn is_available(&self) -> bool {
        false
    }

    fn initialize(&mut self) -> Result<(), String> {
        Err("WebGL not available on this platform".to_string())
    }

    fn set_size(&mut self, _width: usize, _height: usize) {
    }

    fn clear(&mut self, _color: (u8, u8, u8)) {
    }

    fn present(&mut self) {
    }

    fn get_framebuffer(&mut self) -> Option<&mut RenderBuffer> {
        None
    }

    fn get_capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            max_texture_size: 4096,
            supports_shaders: true,
            supports_anti_aliasing: true,
            supports_hardware_acceleration: true,
            max_vertex_count: 1000000,
            supports_geometry_shader: false,
            supports_compute_shader: false,
        }
    }
}

#[cfg(target_os = "windows")]
pub struct Direct2DRenderer;

#[cfg(target_os = "windows")]
impl Direct2DRenderer {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(target_os = "windows")]
impl RenderBackend for Direct2DRenderer {
    fn name(&self) -> &str {
        "Direct2D"
    }

    fn is_available(&self) -> bool {
        false
    }

    fn initialize(&mut self) -> Result<(), String> {
        Err("Direct2D not available on this platform".to_string())
    }

    fn set_size(&mut self, _width: usize, _height: usize) {
    }

    fn clear(&mut self, _color: (u8, u8, u8)) {
    }

    fn present(&mut self) {
    }

    fn get_framebuffer(&mut self) -> Option<&mut RenderBuffer> {
        None
    }

    fn get_capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            max_texture_size: 16384,
            supports_shaders: true,
            supports_anti_aliasing: true,
            supports_hardware_acceleration: true,
            max_vertex_count: 10000000,
            supports_geometry_shader: true,
            supports_compute_shader: true,
        }
    }
}

#[cfg(target_os = "macos")]
pub struct MetalRenderer;

#[cfg(target_os = "macos")]
impl MetalRenderer {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(target_os = "macos")]
impl RenderBackend for MetalRenderer {
    fn name(&self) -> &str {
        "Metal"
    }

    fn is_available(&self) -> bool {
        false
    }

    fn initialize(&mut self) -> Result<(), String> {
        Err("Metal not available on this platform".to_string())
    }

    fn set_size(&mut self, _width: usize, _height: usize) {
    }

    fn clear(&mut self, _color: (u8, u8, u8)) {
    }

    fn present(&mut self) {
    }

    fn get_framebuffer(&mut self) -> Option<&mut RenderBuffer> {
        None
    }

    fn get_capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            max_texture_size: 16384,
            supports_shaders: true,
            supports_anti_aliasing: true,
            supports_hardware_acceleration: true,
            max_vertex_count: 10000000,
            supports_geometry_shader: true,
            supports_compute_shader: true,
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
pub struct OpenGLRenderer;

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
impl OpenGLRenderer {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
impl RenderBackend for OpenGLRenderer {
    fn name(&self) -> &str {
        "OpenGL"
    }

    fn is_available(&self) -> bool {
        false
    }

    fn initialize(&mut self) -> Result<(), String> {
        Err("OpenGL not available on this platform".to_string())
    }

    fn set_size(&mut self, _width: usize, _height: usize) {
    }

    fn clear(&mut self, _color: (u8, u8, u8)) {
    }

    fn present(&mut self) {
    }

    fn get_framebuffer(&mut self) -> Option<&mut RenderBuffer> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_software_renderer_creation() {
        let renderer = SoftwareRenderer::new();
        assert_eq!(renderer.name(), "Software");
        assert!(renderer.is_available());
    }

    #[test]
    fn test_software_renderer_with_size() {
        let renderer = SoftwareRenderer::with_size(1024, 768);
        assert!(renderer.is_available());
    }

    #[test]
    fn test_svg_renderer_creation() {
        let renderer = SVGRenderer::new();
        assert_eq!(renderer.name(), "SVG");
        assert!(renderer.is_available());
    }
}
