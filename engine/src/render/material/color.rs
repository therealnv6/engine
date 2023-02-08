use typed_builder::TypedBuilder;
use wgpu::{util::DeviceExt, BindGroup};

use crate::render::{
    builder::pipeline::PipelineBuilder,
    color::Color,
    raw::{IntoRawBinder, RawBinder},
    vertex::{TransformRaw, Vertex, VertexDescriptor},
};

#[derive(TypedBuilder, Debug, Clone)]
pub struct StaticColorMaterial {
    color: Color,
}

#[derive(Debug)]
pub struct RawStaticColorMaterial {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
}

impl RawBinder for RawStaticColorMaterial {
    fn bind_to_pass<'a>(&'a self, idx: u32, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(idx, &self.bind_group, &[]);
    }
}

impl IntoRawBinder for StaticColorMaterial {
    type RawBinder = RawStaticColorMaterial;

    fn into_raw(&self, params: &crate::render::raw::RawParams) -> RawStaticColorMaterial {
        let device = params.device;
        let config = params.config;

        let color_tab: [f32; 4] = self.color.into();
        let color_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("voxel_chunk_vertices"),
            contents: bytemuck::cast_slice(&color_tab),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = RawStaticColorMaterial::create_bind_layout(device);
        let bind_group =
            RawStaticColorMaterial::create_bind_group(device, &bind_group_layout, &color_buffer);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout, &params.raw_camera.bind_group_layout],
                push_constant_ranges: &[],
            });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../../../shader.wgsl").into()),
        });

        let pipeline = PipelineBuilder::builder()
            .pipeline_layout(Some(&render_pipeline_layout))
            .vertex_layouts(&[Vertex::descript(), TransformRaw::descript()])
            .shader_module(shader)
            .label("tri")
            .fragment(true)
            .build()
            .into_pipeline(device, config);

        RawStaticColorMaterial {
            pipeline,
            bind_group,
        }
    }
}

impl RawStaticColorMaterial {
    fn create_bind_group(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        buffer: &wgpu::Buffer,
    ) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("static-color-bind"),
        })
    }

    fn create_bind_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("static-color-layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }
}
