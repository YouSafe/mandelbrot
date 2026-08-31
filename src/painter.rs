use eframe::{
    egui_wgpu::{Callback, CallbackTrait, RenderState},
    wgpu::{self, BindGroup, Buffer, RenderPipeline, util::DeviceExt as _},
};
use egui::Rect;

pub struct FractalPainter {
    bind_group: BindGroup,
    uniform_buffer: Buffer,
}

impl FractalPainter {
    pub fn new(render_state: &RenderState) -> Self {
        let shader_module =
            render_state
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: None,
                    source: wgpu::ShaderSource::Wgsl(include_str!("shaders/fractal.wgsl").into()),
                });

        let uniform_bind_group_layout =
            render_state
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

        let pipeline_layout =
            render_state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[Some(&uniform_bind_group_layout)],
                    immediate_size: 0,
                });

        let uniform_buffer =
            render_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&[PainterUniform {
                        c: [0.0; 2],
                        size: [0.0; 2],
                        translation: [0.0; 2],
                        scale: 2.0,
                        max_iters: 100,
                        _pad: [0.0; 3],
                        is_julia: 0,
                    }]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        let bind_group = render_state
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &uniform_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(
                        uniform_buffer.as_entire_buffer_binding(),
                    ),
                }],
            });

        let pipeline =
            render_state
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: None,
                    layout: Some(&pipeline_layout),
                    cache: None,
                    vertex: wgpu::VertexState {
                        module: &shader_module,
                        entry_point: Some("vs_main"),
                        buffers: &[],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader_module,
                        entry_point: Some("fs_main"),
                        targets: &[Some(render_state.target_format.into())],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: None,
                        ..Default::default()
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                    multiview_mask: None,
                });

        render_state
            .renderer
            .write()
            .callback_resources
            .insert(PainterResources { pipeline });

        Self {
            bind_group,
            uniform_buffer,
        }
    }

    pub fn paint(&self, ui: &egui::Ui, rect: Rect, uniform: PainterUniform) {
        let cb = Callback::new_paint_callback(
            rect,
            RenderCallback {
                bind_group: self.bind_group.clone(),
                uniform_buffer: self.uniform_buffer.clone(),
                uniform,
            },
        );
        ui.painter().add(cb);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PainterUniform {
    pub c: [f32; 2],
    pub size: [f32; 2],
    pub translation: [f32; 2],
    pub scale: f32,
    pub max_iters: u32,
    pub _pad: [f32; 3],
    pub is_julia: u32,
}

struct PainterResources {
    pipeline: RenderPipeline,
}

struct RenderCallback {
    uniform_buffer: Buffer,
    bind_group: BindGroup,
    uniform: PainterUniform,
}

impl CallbackTrait for RenderCallback {
    fn prepare(
        &self,
        _device: &eframe::wgpu::Device,
        queue: &eframe::wgpu::Queue,
        _screen_descriptor: &eframe::egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut eframe::wgpu::CommandEncoder,
        _callback_resources: &mut eframe::egui_wgpu::CallbackResources,
    ) -> Vec<eframe::wgpu::CommandBuffer> {
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniform]),
        );

        vec![]
    }

    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut eframe::wgpu::RenderPass<'static>,
        callback_resources: &eframe::egui_wgpu::CallbackResources,
    ) {
        let resources = callback_resources
            .get::<PainterResources>()
            .expect("resources set by FractalPainter");
        render_pass.set_pipeline(&resources.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }
}
