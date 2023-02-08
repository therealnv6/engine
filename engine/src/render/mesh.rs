use typed_builder::TypedBuilder;
use wgpu::{util::DeviceExt, RenderPass};

use super::vertex::{TransformRaw, Vertex};

#[derive(TypedBuilder, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    #[builder(default, setter(strip_option))]
    pub indices: Option<Vec<u32>>,
}

impl Mesh {
    /// Creates a `RawMesh` from a `Mesh` object.
    ///
    /// # Performance Considerations
    ///
    /// This method creates a new GPU buffer for the `Mesh`, which can be expensive. It's recommended to avoid calling this method multiple times if possible.
    ///
    /// # Parameters
    ///
    /// * `device` - The `wgpu::Device` to use when creating the GPU buffers.
    /// * `arena` - The `BufferArena` to use for managing the GPU buffers.
    ///
    /// # Returns
    ///
    /// A `RawMesh` representing the inner `Mesh`.
    pub fn to_raw(&self, device: &wgpu::Device) -> RawMesh {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("voxel_chunk_vertices"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let indices = self.indices.clone().map(|indices| {
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("voxel_chunk_indices"),
                contents: bytemuck::cast_slice(indices.as_ref()),
                usage: wgpu::BufferUsages::INDEX,
            });

            (buffer, indices.len())
        });

        let (index_buffer, index_count) = match indices {
            Some((index_buffer, index_count)) => (Some(index_buffer), index_count),
            None => (None, 0),
        };

        RawMesh {
            index_buffer,
            vertex_buffer,
            num_vertices: self.vertices.len() * 3,
            num_indices: index_count,
        }
    }
}

#[derive(Debug)]
pub struct RawMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: Option<wgpu::Buffer>,
    num_vertices: usize,
    num_indices: usize,
}

pub trait UntypedMeshRender<'a> {
    fn render_raw_mesh(
        &mut self,
        idx: u32,
        vertices: &'a wgpu::Buffer,
        indices: Option<&'a wgpu::Buffer>,
        num_indices: usize,
        num_vertices: usize,
    );

    fn render_instanced_raw_mesh(
        &mut self,
        idx: u32,
        vertices: &'a wgpu::Buffer,
        indices: Option<&'a wgpu::Buffer>,
        instance: &'a Vec<TransformRaw>,
        instance_buffer: &'a wgpu::Buffer,
        num_indices: usize,
        num_vertices: usize,
    );
}

pub trait MeshRender<'a> {
    fn render_single_mesh(&mut self, idx: u32, mesh: &'a RawMesh);
    fn render_instanced_mesh(
        &mut self,
        idx: u32,
        instances: &'a Vec<TransformRaw>,
        instance_buffer: &'a wgpu::Buffer,
        mesh: &'a RawMesh,
    );
}

impl<'a> UntypedMeshRender<'a> for RenderPass<'a> {
    fn render_raw_mesh(
        &mut self,
        idx: u32,
        vertices: &'a wgpu::Buffer,
        indices: Option<&'a wgpu::Buffer>,
        num_indices: usize,
        num_vertices: usize,
    ) {
        self.set_vertex_buffer(idx, vertices.slice(..));

        if let Some(index_buffer) = &indices {
            self.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        }

        if num_indices != 0 {
            self.draw_indexed(0..(num_indices as u32), 0, 0..1);
        } else {
            self.draw(0..(num_vertices as u32), 0..1);
        }
    }

    fn render_instanced_raw_mesh(
        &mut self,
        idx: u32,
        vertices: &'a wgpu::Buffer,
        indices: Option<&'a wgpu::Buffer>,
        instance: &'a Vec<TransformRaw>,
        instance_buffer: &'a wgpu::Buffer,
        num_indices: usize,
        num_vertices: usize,
    ) {
        self.set_vertex_buffer(idx, vertices.slice(..));
        self.set_vertex_buffer(idx + 1, instance_buffer.slice(..));

        if let Some(index_buffer) = &indices {
            self.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        }

        if num_indices != 0 {
            self.draw_indexed(0..(num_indices as u32), 0, 0..instance.len() as u32);
        } else {
            self.draw(0..(num_vertices as u32), 0..instance.len() as u32);
        }
    }
}

impl<'a> MeshRender<'a> for RenderPass<'a> {
    fn render_single_mesh(&mut self, idx: u32, mesh: &'a RawMesh) {
        self.render_raw_mesh(
            idx,
            &mesh.vertex_buffer,
            mesh.index_buffer.as_ref(),
            mesh.num_indices,
            mesh.num_vertices,
        );
    }

    fn render_instanced_mesh(
        &mut self,
        idx: u32,
        instances: &'a Vec<TransformRaw>,
        instance_buffer: &'a wgpu::Buffer,
        mesh: &'a RawMesh,
    ) {
        self.render_instanced_raw_mesh(
            idx,
            &mesh.vertex_buffer,
            mesh.index_buffer.as_ref(),
            instances,
            instance_buffer,
            mesh.num_indices,
            mesh.num_vertices,
        );
    }
}
