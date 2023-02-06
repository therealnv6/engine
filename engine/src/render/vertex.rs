use glam::{Mat4, Quat, Vec3};
use typed_builder::TypedBuilder;
use wgpu::{util::DeviceExt, BufferUsages};

use super::raw::RawParams;

#[repr(C)]
#[derive(TypedBuilder, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

impl VertexDescriptor<2> for Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
    ];

    fn descript<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[derive(TypedBuilder, Debug)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TransformRaw {
    pub model: [[f32; 4]; 4],
    pub normal_matrix: [[f32; 4]; 4],
}

impl Transform {
    pub fn to_raw<'a>(&self, params: &'a RawParams) -> TransformRaw {
        let model = Mat4::from_translation(self.translation) * Mat4::from_quat(self.rotation);
        let normal_matrix = model.inverse().transpose();

        TransformRaw {
            model: model.to_cols_array_2d(),
            normal_matrix: normal_matrix.to_cols_array_2d(),
        }
    }
}

impl VertexDescriptor<8> for TransformRaw {
    const ATTRIBS: [wgpu::VertexAttribute; 8] = wgpu::vertex_attr_array![
        0 => Float32x4,
        1 => Float32x4,
        2 => Float32x4,
        3 => Float32x4,
        4 => Float32x4,
        5 => Float32x4,
        6 => Float32x4,
        7 => Float32x4
    ];

    fn descript<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub trait VertexDescriptor<const C: usize> {
    const ATTRIBS: [wgpu::VertexAttribute; C];
    fn descript<'a>() -> wgpu::VertexBufferLayout<'a>;
}
