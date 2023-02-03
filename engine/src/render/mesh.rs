use anyhow::Context;
use typed_builder::TypedBuilder;
use wgpu::{util::DeviceExt, RenderPass};

use super::{vertex::Vertex, BufferArena};

#[derive(TypedBuilder)]
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
    pub fn to_raw_arena(&self, device: &wgpu::Device, arena: &mut BufferArena) -> ArenaRawMesh {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("voxel_chunk_vertices"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let vertex_index = arena.arena.insert(vertex_buffer);

        let indices = self.indices.clone().map(|indices| {
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("voxel_chunk_indices"),
                contents: bytemuck::cast_slice(indices.as_ref()),
                usage: wgpu::BufferUsages::INDEX,
            });

            (arena.arena.insert(buffer), indices.len())
        });

        let (index_buffer, index_count) = match indices {
            Some((index_buffer, index_count)) => (Some(index_buffer), index_count),
            None => (None, 0),
        };

        ArenaRawMesh {
            index_buffer,
            vertex_buffer: vertex_index,
            num_vertices: self.vertices.len() * 3,
            num_indices: index_count,
        }
    }
}

pub struct RawMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: Option<wgpu::Buffer>,
    num_vertices: usize,
    num_indices: usize,
}

pub struct ArenaRawMesh {
    vertex_buffer: generational_arena::Index,
    index_buffer: Option<generational_arena::Index>,
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
}

pub trait MeshRender<'a> {
    fn render_mesh(&mut self, idx: u32, mesh: &'a RawMesh);
}

pub trait ArenaMeshRender<'a> {
    fn render_mesh_arena(&mut self, arena: &'a BufferArena, idx: u32, mesh: &'a ArenaRawMesh);
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
}

impl<'a> MeshRender<'a> for RenderPass<'a> {
    fn render_mesh(&mut self, idx: u32, mesh: &'a RawMesh) {
        self.render_raw_mesh(
            idx,
            &mesh.vertex_buffer,
            mesh.index_buffer.as_ref(),
            mesh.num_indices,
            mesh.num_vertices,
        );
    }
}

impl<'a> ArenaMeshRender<'a> for RenderPass<'a> {
    fn render_mesh_arena(&mut self, arena: &'a BufferArena, idx: u32, mesh: &'a ArenaRawMesh) {
        let index_buffer = match mesh.index_buffer {
            Some(index_buffer) => arena
                .arena
                .get(index_buffer)
                .context("Unable to retrieve ibuffer!")
                .ok(),
            None => None,
        };

        self.render_raw_mesh(
            idx,
            arena
                .arena
                .get(mesh.vertex_buffer)
                .context("Unable to retrieve vbuffer!")
                .unwrap(),
            index_buffer,
            mesh.num_indices,
            mesh.num_vertices,
        );
    }
}
