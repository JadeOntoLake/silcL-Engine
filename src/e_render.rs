use eframe::egui;
use egui::Color32;
use crate::math_kernel::MathKernel;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
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

pub struct ERenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    edge_buffer: wgpu::Buffer,
    num_indices: u32,
    num_edges: u32,
    pipeline: wgpu::RenderPipeline,
    edge_pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl ERenderer {
    pub fn new(cc: &eframe::CreationContext<'_>, kernel: &MathKernel) -> Option<Self> {
        let wgpu_renderer = cc.wgpu_renderer.as_ref()?;
        let device = wgpu_renderer.device().clone();
        let queue = wgpu_renderer.queue().clone();

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let vertices: Vec<Vertex> = kernel.vertex_list
            .iter()
            .map(|v| Vertex { position: [v[0] as f32, v[1] as f32, v[2] as f32] })
            .collect();

        let indices: Vec<u32> = kernel.render_faces_list
            .iter()
            .flat_map(|tri| tri.iter().map(|i| *i as u32))
            .collect();

        let edges: Vec<u32> = kernel.edge_list
            .iter()
            .flat_map(|e| vec![e[0] as u32, e[1] as u32])
            .collect();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex_buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index_buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let edge_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("edge_buffer"),
            contents: bytemuck::cast_slice(&edges),
            usage: wgpu::BufferUsages::INDEX,
        });

        #[repr(C)]
        #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
        struct Uniforms {
            view_proj: [[f32; 4]; 4],
        }

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniform_buffer"),
            contents: bytemuck::cast_slice(&[Uniforms {
                view_proj: cgmath::Matrix4::identity().into(),
            }]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bind_group_layout"),
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
            label: Some("bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("pipeline"),
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
                    format: wgpu::TextureFormat::Bgra8Unorm,
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
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let edge_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("edge_pipeline"),
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
                    format: wgpu::TextureFormat::Bgra8Unorm,
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
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Some(ERenderer {
            device,
            queue,
            vertex_buffer,
            index_buffer,
            edge_buffer,
            num_indices: indices.len() as u32,
            num_edges: edges.len() as u32,
            pipeline,
            edge_pipeline,
            uniform_buffer,
            bind_group,
            bind_group_layout,
        })
    }

    pub fn update_buffers(&mut self, kernel: &MathKernel) {
        let vertices: Vec<Vertex> = kernel.vertex_list
            .iter()
            .map(|v| Vertex { position: [v[0] as f32, v[1] as f32, v[2] as f32] })
            .collect();

        self.queue.write_buffer(
            &self.vertex_buffer,
            0,
            bytemuck::cast_slice(&vertices),
        );

        let indices: Vec<u32> = kernel.render_faces_list
            .iter()
            .flat_map(|tri| tri.iter().map(|i| *i as u32))
            .collect();

        self.queue.write_buffer(
            &self.index_buffer,
            0,
            bytemuck::cast_slice(&indices),
        );

        let edges: Vec<u32> = kernel.edge_list
            .iter()
            .flat_map(|e| vec![e[0] as u32, e[1] as u32])
            .collect();

        self.queue.write_buffer(
            &self.edge_buffer,
            0,
            bytemuck::cast_slice(&edges),
        );

        self.num_indices = indices.len() as u32;
        self.num_edges = edges.len() as u32;
    }

    pub fn render(&mut self, kernel: &MathKernel, frame: &mut eframe::Frame) {
        let proj = cgmath::perspective(
            cgmath::Deg(45.0),
            frame.info().window_info.size[0] / frame.info().window_info.size[1],
            0.1,
            100.0,
        );

        let view = cgmath::Matrix4::from_angle_x(cgmath::Rad(0.5))
            * cgmath::Matrix4::from_angle_y(cgmath::Rad(0.8))
            * cgmath::Matrix4::from_translation(cgmath::Vector3::new(0.0, 0.0, -5.0));

        let view_proj = proj * view;

        #[repr(C)]
        #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
        struct Uniforms {
            view_proj: [[f32; 4]; 4],
        }

        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[Uniforms {
                view_proj: view_proj.into(),
            }]),
        );

        // 绘制3D场景
        // eframe不直接暴露surface，需要paint_callback
        // 这里留空，等eframe版本更新或改用其他方式
    }
}