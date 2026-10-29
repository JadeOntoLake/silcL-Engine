/*use wgpu::util::DeviceExt;
use cgmath::{Matrix4, SquareMatrix};

use crate::math_kernel::MathKernel;
use crate::control::OperationManager;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub view_proj: [[f32; 4]; 4],
}

pub struct SceneData {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub edge_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub num_edges: u32,
    pub uniform_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub triangle_pipeline: wgpu::RenderPipeline,
    pub wireframe_pipeline: wgpu::RenderPipeline,
    pub shader: wgpu::ShaderModule,
}

pub struct SilcLApp {
    pub kernel: MathKernel,
    pub manager: OperationManager,
    pub scene_data: Option<std::sync::Arc<SceneData>>,
    pub translate_x: f64,
    pub translate_y: f64,
    pub translate_z: f64,
    pub view_width: f32,
    pub view_height: f32,
}

impl SilcLApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        kernel: MathKernel,
        manager: OperationManager,
    ) -> Self {
        let mut app = Self {
            kernel,
            manager,
            scene_data: None,
            translate_x: 0.0,
            translate_y: 0.0,
            translate_z: 0.0,
            view_width: 800.0,
            view_height: 600.0,
        };

        if let Some(state) = &cc.wgpu_render_state {
            app.scene_data = Some(std::sync::Arc::new(app.create_scene_data(state)));
        }

        app
    }

    fn create_scene_data(&self, state: &egui_wgpu::RenderState) -> SceneData {
        let device = &state.device;
        let target_format = state.target_format;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[Uniforms {
                view_proj: Matrix4::identity().into(),
            }]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let vertices: Vec<Vertex> = self.kernel.vertex_list
            .iter()
            .map(|v| Vertex { position: [v[0] as f32, v[1] as f32, v[2] as f32] })
            .collect();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let indices: Vec<u32> = self.kernel.render_faces_list
            .iter()
            .flat_map(|tri| tri.iter().map(|i| *i as u32))
            .collect();

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let edge_indices: Vec<u32> = self.kernel.edge_list
            .iter()
            .flat_map(|&[a, b]| vec![a as u32, b as u32])
            .collect();

        let edge_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&edge_indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = indices.len() as u32;
        let num_edges = edge_indices.len() as u32;

        let triangle_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Always,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let wireframe_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "ed_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Always,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        SceneData {
            vertex_buffer,
            index_buffer,
            edge_buffer,
            num_indices,
            num_edges,
            uniform_buffer,
            bind_group,
            triangle_pipeline,
            wireframe_pipeline,
            shader,
        }
    }
}

impl eframe::App for SilcLApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut need_update = false;
        let mut view_width = 0.0;
        let mut view_height = 0.0;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("SilcL 引擎控制");
            ui.separator();

            ui.heading("平移控制");
            ui.horizontal(|ui| {
                ui.label("X:");
                ui.add(egui::DragValue::new(&mut self.translate_x).speed(0.1));
                ui.label("Y:");
                ui.add(egui::DragValue::new(&mut self.translate_y).speed(0.1));
                ui.label("Z:");
                ui.add(egui::DragValue::new(&mut self.translate_z).speed(0.1));
                if ui.button("执行平移").clicked() {
                    self.manager.translate(self.translate_x, self.translate_y, self.translate_z);
                    need_update = true;
                }
            });

            ui.horizontal(|ui| {
                if ui.button("撤销").clicked() {
                    self.manager.undo();
                    need_update = true;
                }
                if ui.button("重做").clicked() {
                    self.manager.redo();
                    need_update = true;
                }
            });

            ui.separator();
            ui.label(format!("顶点: {}", self.kernel.vertex_list.len()));

            ui.separator();

            let available = ui.available_rect_before_wrap();
            view_width = available.width();
            view_height = available.height();
            self.view_width = view_width;
            self.view_height = view_height;

            if available.width() > 0.0 && available.height() > 0.0 {
                let data = self.scene_data.clone();
                if let Some(data) = data {
                    let callback = egui::PaintCallback {
                        rect: available,
                        callback: std::sync::Arc::new(
                            move |_info: egui::PaintCallbackInfo,
                                  render_pass: &mut wgpu::RenderPass,
                                  _descriptor: &egui_wgpu::ScreenDescriptor| {
                                render_pass.set_pipeline(&data.triangle_pipeline);
                                render_pass.set_bind_group(0, &data.bind_group, &[]);
                                render_pass.set_vertex_buffer(0, data.vertex_buffer.slice(..));
                                render_pass.set_index_buffer(
                                    data.index_buffer.slice(..),
                                    wgpu::IndexFormat::Uint32,
                                );
                                if data.num_indices > 0 {
                                    render_pass.draw_indexed(0..data.num_indices, 0, 0..1);
                                }

                                render_pass.set_pipeline(&data.wireframe_pipeline);
                                render_pass.set_vertex_buffer(0, data.vertex_buffer.slice(..));
                                render_pass.set_index_buffer(
                                    data.edge_buffer.slice(..),
                                    wgpu::IndexFormat::Uint32,
                                );
                                if data.num_edges > 0 {
                                    render_pass.draw_indexed(0..data.num_edges, 0, 0..1);
                                }
                            },
                        ),
                    };
                    ui.painter().add(callback);
                }
            } else {
                ui.label("视口太小");
            }
        });

        // 更新 uniform
        if let Some(state) = _frame.wgpu_render_state() {
            if let Some(ref data) = self.scene_data {
                let aspect = if view_height > 0.0 {
                    view_width / view_height
                } else {
                    1.6
                };

                let view = cgmath::Matrix4::look_at_rh(
                    cgmath::Point3::new(4.0, 3.0, 2.0),
                    cgmath::Point3::new(0.0, 0.0, 0.0),
                    cgmath::Vector3::new(0.0, 1.0, 0.0),
                );
                let proj = cgmath::perspective(cgmath::Deg(45.0), aspect, 0.1, 100.0);
                let view_proj = proj * view;
                let uniforms = Uniforms {
                    view_proj: view_proj.into(),
                };

                state.queue.write_buffer(
                    &data.uniform_buffer,
                    0,
                    bytemuck::cast_slice(&[uniforms]),
                );
            }

            if need_update {
                let vertices: Vec<Vertex> = self.kernel.vertex_list
                    .iter()
                    .map(|v| Vertex { position: [v[0] as f32, v[1] as f32, v[2] as f32] })
                    .collect();
                if let Some(ref data) = self.scene_data {
                    state.queue.write_buffer(&data.vertex_buffer, 0, bytemuck::cast_slice(&vertices));

                    let indices: Vec<u32> = self.kernel.render_faces_list
                        .iter()
                        .flat_map(|tri| tri.iter().map(|i| *i as u32))
                        .collect();
                    state.queue.write_buffer(&data.index_buffer, 0, bytemuck::cast_slice(&indices));

                    let edge_indices: Vec<u32> = self.kernel.edge_list
                        .iter()
                        .flat_map(|&[a, b]| vec![a as u32, b as u32])
                        .collect();
                    state.queue.write_buffer(&data.edge_buffer, 0, bytemuck::cast_slice(&edge_indices));
                }
            }
        }

        ctx.request_repaint();
    }
}*/

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;
use winit::window::Window;
use std::sync::Arc;

use cgmath::{Matrix4, perspective, Deg};
use cgmath::SquareMatrix;
use cgmath::prelude::*;

use egui_wgpu::Renderer as EguiRenderer;
use egui_winit::State as EguiState;

use crate::math_kernel::MathKernel;
use crate::control::OperationManager;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    view_proj: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}

pub struct Renderer<'window> {
    pub surface: wgpu::Surface<'window>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub triangle_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub uniform_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub wireframe_pipeline: wgpu::RenderPipeline,
    pub edge_buffer: wgpu::Buffer,
    pub num_edge: u32,
    pub egui_ctx: egui::Context,
    pub egui_renderer: EguiRenderer,
    pub egui_state: EguiState,
    pub window: Arc<Window>,
    pub depth_texture: wgpu::Texture,
    pub depth_view: wgpu::TextureView,
    pub x_axis_pipeline: wgpu::RenderPipeline,
    pub y_axis_pipeline: wgpu::RenderPipeline,
    pub z_axis_pipeline: wgpu::RenderPipeline,
}

impl<'window> Renderer<'window> {
    pub async fn new(window: Arc<Window>, kernel: &MathKernel) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth_texture"),
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&Default::default());

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[Uniforms {
                view_proj: cgmath::Matrix4::identity().into(),
            }]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let triangle_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Always,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let wireframe_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "ed_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Always,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let vertices: Vec<Vertex> = kernel.vertex_list
            .iter()
            .map(|v| Vertex { position: [v[0] as f32, v[1] as f32, v[2] as f32] })
            .collect();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let indices: Vec<u32> = kernel.render_faces_list
            .iter()
            .flat_map(|tri| tri.iter().map(|i| *i as u32))
            .collect();

        let edge_indices: Vec<u32> = kernel.edge_list
            .iter()
            .flat_map(|&[a,b]| vec![a as u32,b as u32])
            .collect();

        let edge_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&edge_indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });
        let num_edge = edge_indices.len() as u32;

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });

        let x_axis_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "x_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend:Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::Zero,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Always,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let y_axis_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "y_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend:Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::Zero,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Always,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let z_axis_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "z_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend:Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::Zero,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Always,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });
        let egui_ctx = egui::Context::default();
        let egui_renderer = EguiRenderer::new(&device, config.format, Some(wgpu::TextureFormat::Depth32Float), 1);
        let egui_state = EguiState::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
        );

        Self {
            surface,
            device,
            queue,
            config,
            triangle_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
            uniform_buffer,
            bind_group,
            wireframe_pipeline,
            edge_buffer,
            num_edge,
            egui_ctx,
            egui_renderer,
            egui_state,
            window: window.clone(),
            depth_texture,
            depth_view,
            x_axis_pipeline,
            y_axis_pipeline,
            z_axis_pipeline,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);

        self.depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("depth_texture"),
            size: wgpu::Extent3d {
                width: new_size.width,
                height: new_size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        self.depth_view = self.depth_texture.create_view(&Default::default());
    }

    pub fn update_buffers(&mut self, kernel: &MathKernel) {
        let vertices: Vec<Vertex> = kernel.vertex_list
            .iter()
            .map(|v| Vertex { position: [v[0] as f32, v[1] as f32, v[2] as f32] })
            .collect();

        self.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));

        let indices: Vec<u32> = kernel.render_faces_list
            .iter()
            .flat_map(|tri| tri.iter().map(|i| *i as u32))
            .collect();

        self.queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&indices));

        let edge_indices: Vec<u32> = kernel.edge_list
            .iter()
            .flat_map(|&[a,b]| vec![a as u32,b as u32])
            .collect();

        self.queue.write_buffer(&self.edge_buffer, 0, bytemuck::cast_slice(&edge_indices));

        self.num_indices = indices.len() as u32;
        self.num_edge = edge_indices.len() as u32;
    }

    pub fn render(&mut self, manager: &mut OperationManager) -> Result<(), wgpu::SurfaceError> {
        
        let mut center = [0.0, 0.0, 0.0];
        for v in &manager.kernel.vertex_list {
            center[0] += v[0];
            center[1] += v[1];
            center[2] += v[2];
        }
        let n = manager.kernel.vertex_list.len() as f64;
        let center = [center[0] / n, center[1] / n, center[2] / n];

        let view_mat = Matrix4::look_at_rh(
            cgmath::Point3::new(4.0_f32, 3.0_f32, 2.0_f32),
            cgmath::Point3::new(center[0] as f32, center[1] as f32, center[2] as f32),
            cgmath::Vector3::new(0.0_f32, 1.0_f32, 0.0_f32),
        );

        let proj = perspective(Deg(45.0), self.config.width as f32 / self.config.height as f32, 0.1, 100.0);
        let view_proj = proj * view_mat;

        let uniforms = Uniforms {
            view_proj: view_proj.into(),
        };
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        let output = match self.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(wgpu::SurfaceError::Lost) => {
                println!("Surface Lost, resizing");
                self.resize(self.window.inner_size());
                return Ok(());
            }
            Err(_) => {
                return Ok(());
            }
        };

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let raw_input = self.egui_state.take_egui_input(&self.window);

        let mut translate_x = 0.0;
        let mut translate_y = 0.0;
        let mut translate_z = 0.0;
        let mut need_update = false;

        let full_output = self.egui_ctx.run(raw_input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.painter().rect_filled(
                    ui.available_rect_before_wrap(),
                    0.0,
                    egui::Color32::from_rgb(255, 0, 0),
                );
                ui.heading("SilcL 引擎控制");
                ui.separator();
                ui.heading("平移控制");
                ui.horizontal(|ui| {
                    ui.label("X:");
                    ui.add(egui::DragValue::new(&mut translate_x).speed(0.1));
                    ui.label("Y:");
                    ui.add(egui::DragValue::new(&mut translate_y).speed(0.1));
                    ui.label("Z:");
                    ui.add(egui::DragValue::new(&mut translate_z).speed(0.1));
                    if ui.button("执行平移").clicked() {
                        manager.translate(translate_x, translate_y, translate_z);
                        need_update = true;
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("撤销").clicked() {
                        manager.undo();
                        need_update = true;
                    }
                    if ui.button("重做").clicked() {
                        manager.redo();
                        need_update = true;
                    }
                });
            });
        });

        //println!("shapes count: {}", full_output.shapes.len());

        if need_update {
            self.update_buffers(&manager.kernel);
        }

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: self.window.scale_factor() as f32,
        };

        let paint_jobs = self.egui_ctx.tessellate(full_output.shapes, full_output.pixels_per_point);
        self.egui_renderer.update_buffers(
            &self.device,
            &self.queue,
            &mut encoder,
            &paint_jobs,
            &screen_descriptor,
        );
        //println!("update_buffers done");

        {
            let axis_length = 100.0;
            let axis_vertices: [Vertex; 4] = [
                Vertex { position: [0.0, 0.0, 0.0] },
                Vertex { position: [0.0, 0.0, axis_length] },
                Vertex { position: [axis_length, 0.0, 0.0] },
                Vertex { position: [0.0, axis_length, 0.0] },
            ];
            let axis_indices: [u16; 6] = [0, 1, 0, 2, 0, 3];

            let axis_vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&axis_vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let axis_index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&axis_indices),
                usage: wgpu::BufferUsages::INDEX,
            });
            
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.12,
                            g: 0.12,
                            b: 0.15,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.egui_renderer.render(&mut render_pass, &paint_jobs, &screen_descriptor);

            render_pass.set_pipeline(&self.triangle_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);

            render_pass.set_pipeline(&self.wireframe_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.edge_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.num_edge, 0, 0..1);

            // ⭐ 坐标轴（新增）
            

            // 画 X 轴（红色）
            render_pass.set_pipeline(&self.x_axis_pipeline);
            render_pass.set_vertex_buffer(0, axis_vertex_buffer.slice(..));
            render_pass.set_index_buffer(axis_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..2, 0, 0..1);  // 只画 X 轴（索引 0,1）

            // 画 Y 轴（绿色）
            render_pass.set_pipeline(&self.y_axis_pipeline);
            render_pass.set_vertex_buffer(0, axis_vertex_buffer.slice(..));
            render_pass.set_index_buffer(axis_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(2..4, 0, 0..1);  // 只画 Y 轴（索引 2,3）

            // 画 Z 轴（蓝色）
            render_pass.set_pipeline(&self.z_axis_pipeline);
            render_pass.set_vertex_buffer(0, axis_vertex_buffer.slice(..));
            render_pass.set_index_buffer(axis_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(4..6, 0, 0..1);  // 只画 Z 轴（索引 4,5）

            drop(render_pass);
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();

        self.egui_ctx.request_repaint();

        Ok(())
    }
    
}