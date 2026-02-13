#[cfg(feature = "gpu")]
use wgpu::{util::DeviceExt, Device, Queue, RenderPipeline, PipelineLayout, BindGroup, Buffer, Texture};
#[cfg(feature = "gpu")]
use std::num::NonZeroU32;

#[cfg(feature = "gpu")]
const VERTEX_SHADER_SOURCE: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(input.position, 0.0, 1.0);
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}
"#;

#[cfg(feature = "gpu")]
const LINE_VERTEX_SHADER: &str = r#"
struct Uniforms {
    screen_size: vec2<f32>,
    line_width: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    let clip_pos = input.position / uniforms.screen_size * 2.0 - 1.0;
    output.position = vec4<f32>(clip_pos.x, -clip_pos.y, 0.0, 1.0);
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}
"#;

#[cfg(feature = "gpu")]
#[derive(Debug)]
pub struct WGPURenderer {
    device: Option<Device>,
    queue: Option<Queue>,
    surface: Option<wgpu::Surface>,
    context: Option<wgpu::SurfaceContext>,
    config: Option<wgpu::SurfaceConfiguration>,
    render_pipeline: Option<RenderPipeline>,
    vertex_buffer: Option<Buffer>,
    uniform_buffer: Option<Buffer>,
    current_width: u32,
    current_height: u32,
    pending_vertices: Vec<RenderVertex>,
}

#[cfg(feature = "gpu")]
#[derive(Debug, Clone, Copy)]
struct RenderVertex {
    position: [f32; 2],
    color: [f32; 4],
}

#[cfg(feature = "gpu")]
#[derive(Debug)]
struct Uniforms {
    screen_size: [f32; 2],
    line_width: f32,
}

#[cfg(feature = "gpu")]
impl WGPURenderer {
    pub async fn new(canvas: &web_sys::HtmlCanvasElement) -> Result<Self, String> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL,
            flags: wgpu::InstanceFlags::default(),
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
            gles_minor_version: Default::default(),
        });

        let surface = instance.create_surface_from_canvas(canvas)
            .map_err(|e| format!("Failed to create surface: {:?}", e))?;

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await
            .ok_or("No suitable GPU adapter found")?;

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("CAD GPU Device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
        }, None).await
            .map_err(|e| format!("Failed to request device: {:?}", e))?;

        let surface_capabilities = surface.get_capabilities(&adapter);
        let format = surface_capabilities.formats.iter()
            .find(|f| f.describe().srgb)
            .copied()
            .unwrap_or(surface_capabilities.formats[0]);

        let width = canvas.width().max(1);
        let height = canvas.height().max(1);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("CAD Shader"),
            source: wgpu::ShaderSource::Wgsl(VERTEX_SHADER_SOURCE.into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("CAD Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("CAD Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<RenderVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                            shader_location: 1,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Ok(Self {
            device: Some(device),
            queue: Some(queue),
            surface: Some(surface),
            context: None,
            config: Some(config),
            render_pipeline: Some(render_pipeline),
            vertex_buffer: None,
            uniform_buffer: None,
            current_width: width,
            current_height: height,
            pending_vertices: Vec::new(),
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if let (Some(device), Some(surface), Some(config)) = (&self.device, &self.surface, &mut self.config) {
            self.current_width = width.max(1);
            self.current_height = height.max(1);
            config.width = self.current_width;
            config.height = self.current_height;
            surface.configure(device, config);
        }
    }

    pub fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.pending_vertices.clear();
    }

    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: &[f32; 4]) {
        let v1 = RenderVertex {
            position: [x1, y1],
            color: *color,
        };
        let v2 = RenderVertex {
            position: [x2, y2],
            color: *color,
        };
        self.pending_vertices.push(v1);
        self.pending_vertices.push(v2);
    }

    pub fn draw_circle(&mut self, cx: f32, cy: f32, r: f32, color: &[f32; 4], segments: u32) {
        let segments = segments.max(32);

        for i in 0..segments {
            let theta1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let theta2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;

            let x1 = cx + theta1.cos() * r;
            let y1 = cy + theta1.sin() * r;
            let x2 = cx + theta2.cos() * r;
            let y2 = cy + theta2.sin() * r;

            self.draw_line(x1, y1, x2, y2, color);
        }
    }

    pub fn present(&mut self) {
        if self.pending_vertices.is_empty() {
            return;
        }

        let (device, queue, surface, config) = match (
            self.device.take(),
            self.queue.take(),
            self.surface.take(),
            self.config.take(),
        ) {
            (Some(d), Some(q), Some(s), Some(c)) => (d, q, s, c),
            _ => return,
        };

        let frame = match surface.get_current_texture() {
            Ok(frame) => frame,
            Err(_) => {
                self.device = Some(device);
                self.queue = Some(queue);
                self.surface = Some(surface);
                self.config = Some(config);
                return;
            }
        };

        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("CAD Render Encoder"),
        });

        let vertex_data: &[f32] = &self.pending_vertices.iter()
            .flat_map(|v| v.position.iter().chain(v.color.iter()))
            .copied()
            .collect::<Vec<f32>>();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("CAD Vertex Buffer"),
            contents: bytemuck::cast_slice(vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("CAD Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        if let Some(pipeline) = &self.render_pipeline {
            render_pass.set_pipeline(pipeline);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.draw(0..(self.pending_vertices.len() as u32), 0..1);
        }

        drop(render_pass);

        queue.submit(std::iter::once(encoder.finish()));

        frame.present();

        self.device = Some(device);
        self.queue = Some(queue);
        self.surface = Some(surface);
        self.config = Some(config);
    }

    pub fn flush(&mut self) {
        self.present();
    }

    pub fn is_available() -> bool {
        true
    }
}

#[cfg(not(feature = "gpu"))]
#[derive(Debug)]
pub struct WGPURenderer;

#[cfg(not(feature = "gpu"))]
impl WGPURenderer {
    pub fn new() -> Self {
        Self
    }

    pub async fn new_canvas(_canvas: &web_sys::HtmlCanvasElement) -> Result<Self, String> {
        Err("GPU rendering requires 'gpu' feature".to_string())
    }

    pub fn resize(&mut self, _width: u32, _height: u32) {}
    pub fn clear(&mut self, _r: f32, _g: f32, _b: f32, _a: f32) {}
    pub fn draw_line(&mut self, _x1: f32, _y1: f32, _x2: f32, _y2: f32, _color: &[f32; 4]) {}
    pub fn draw_circle(&mut self, _cx: f32, _cy: f32, _r: f32, _color: &[f32; 4], _segments: u32) {}
    pub fn present(&mut self) {}
    pub fn flush(&mut self) {}
    pub fn is_available() -> bool { false }
}
