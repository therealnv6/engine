use typed_builder::TypedBuilder;

use super::{
    builder::pipeline::PipelineBuilder,
    color::Color,
    vertex::{Vertex, VertexDescriptor},
};

pub mod color;

pub trait ToRawMaterial<T: RawMaterial> {
    fn to_raw(&self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> T;
}

pub trait RawMaterial {
    fn draw_to_pass<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}

pub trait RawMaterialRender<'a, T: RawMaterial> {
    fn draw_material(&mut self, material: &'a T);
}

impl<'a, T: RawMaterial> RawMaterialRender<'a, T> for wgpu::RenderPass<'a> {
    fn draw_material(&mut self, material: &'a T) {
        material.draw_to_pass(self);
    }
}
