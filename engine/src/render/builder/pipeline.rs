use typed_builder::TypedBuilder;
use wgpu::{BindGroupLayout, Device, PipelineLayout, RenderPipeline, SurfaceConfiguration};

#[derive(TypedBuilder)]
pub struct PipelineBuilder<'a> {
    pipeline_layout: Option<&'a PipelineLayout>,
    #[builder(default)]
    depth_format: Option<wgpu::TextureFormat>,
    vertex_layouts: &'a [wgpu::VertexBufferLayout<'a>],
    shader_module: wgpu::ShaderModule,
    #[builder(default, setter(strip_option))]
    label: Option<&'a str>,
    #[builder(default)]
    fragment: bool,
}

#[derive(TypedBuilder)]
pub struct PipelineLayoutBuilder<'a> {
    #[builder(default, setter(strip_option))]
    label: Option<&'a str>,
    bind_group_layouts: &'a [&'a BindGroupLayout],
}

impl<'a> PipelineBuilder<'a> {
    pub fn into_pipeline(self, device: &Device, config: &SurfaceConfiguration) -> RenderPipeline {
        let frag_state = wgpu::FragmentState {
            module: &self.shader_module,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        };

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: self.label,
            layout: self.pipeline_layout,
            vertex: wgpu::VertexState {
                module: &self.shader_module,
                entry_point: "vs_main",
                buffers: self.vertex_layouts,
            },
            fragment: if self.fragment {
                Some(frag_state)
            } else {
                None
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
                unclipped_depth: true,
            },
            depth_stencil: self.depth_format.map(|format| wgpu::DepthStencilState {
                format,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }
}

impl<'a> PipelineLayoutBuilder<'a> {
    pub fn into_pipeline_layout(&self, device: &wgpu::Device) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: self.label,
            bind_group_layouts: self.bind_group_layouts,
            push_constant_ranges: &[],
        })
    }
}
