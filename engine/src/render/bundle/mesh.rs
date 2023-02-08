use std::collections::{hash_map::Iter, HashMap};

use typed_builder::TypedBuilder;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferUsages,
};

use crate::render::{
    handle::HandleId,
    mesh::{Mesh, MeshRender, RawMesh},
    raw::{IntoRawBinder, RawBinder, RawBindingRender, RawParams},
    vertex::{Transform, TransformRaw},
};

#[derive(Debug)]
pub struct Bundles<T: IntoRawBinder> {
    pub(crate) queued_bundles: HashMap<HandleId, MeshBundle<T>>,
    pub(crate) queued_instances: HashMap<HandleId, Transform>,
    pub(crate) bundles: HashMap<HandleId, RawMeshBundle<T::RawBinder>>,
}

impl<T: IntoRawBinder> Default for Bundles<T> {
    fn default() -> Self {
        Self {
            queued_bundles: HashMap::default(),
            queued_instances: HashMap::default(),
            bundles: HashMap::default(),
        }
    }
}

impl<T: IntoRawBinder> Bundles<T> {
    pub fn add(&mut self, bundle: MeshBundle<T>) -> HandleId {
        let id: HandleId = self.queued_bundles.len().into();
        self.queued_bundles.insert(id, bundle);
        id
    }

    pub fn instance(&mut self, id: HandleId, transform: Transform) {
        self.queued_instances.insert(id, transform);
    }

    pub fn process_queue(&mut self, params: &RawParams) {
        for (id, bundle) in self.queued_bundles.drain() {
            let raw = bundle.into_raw(params);
            let id = id;

            self.bundles.insert(id, raw);
        }

        for (id, transform) in self.queued_instances.drain() {
            let raw = transform.to_raw(params);
            let id = id;

            let raw_bundle = self.bundles.get_mut(&id);

            if let Some(raw_bundle) = raw_bundle {
                raw_bundle.instance(&params, raw);
            }
        }
    }

    pub fn iter(&self) -> Iter<'_, usize, RawMeshBundle<<T as IntoRawBinder>::RawBinder>> {
        self.bundles.iter()
    }
}

#[derive(TypedBuilder, Debug)]
pub struct MeshBundle<T: IntoRawBinder> {
    pub mesh: Mesh,
    #[builder(default, setter(strip_option))]
    pub transform: Option<Transform>,
    pub material: T,
}

#[derive(Debug)]
pub struct RawMeshBundle<T: RawBinder> {
    pub(crate) mesh: RawMesh,
    pub(crate) material: T,
    pub(crate) instances: Vec<TransformRaw>,
    pub(crate) instance_buffer: Option<wgpu::Buffer>,
}

impl<T: RawBinder> RawMeshBundle<T> {
    pub fn instance(&mut self, params: &RawParams, instance: TransformRaw) {
        self.instances.push(instance);
        self.update_buffer(params);
    }

    pub fn update_buffer(&mut self, params: &RawParams) {
        let buffer = params
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("mesh instance buffer"),
                usage: BufferUsages::VERTEX,
                contents: bytemuck::cast_slice(&self.instances),
            });

        self.instance_buffer = Some(buffer);
    }
}

impl<T: RawBinder> RawBinder for RawMeshBundle<T> {
    fn bind_to_pass<'a>(&'a self, idx: u32, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.bind_raw(idx, &self.material);

        match &self.instance_buffer {
            Some(buffer) => {
                render_pass.render_instanced_mesh(0, &self.instances, buffer, &self.mesh)
            }
            None => render_pass.render_single_mesh(0, &self.mesh),
        };
    }
}

impl<T: IntoRawBinder> IntoRawBinder for MeshBundle<T> {
    type RawBinder = RawMeshBundle<<T as IntoRawBinder>::RawBinder>;

    fn into_raw(&self, params: &RawParams) -> Self::RawBinder {
        let raw_mesh = self.mesh.to_raw(params.device);
        let raw_mat = self.material.into_raw(params);

        RawMeshBundle {
            mesh: raw_mesh,
            material: raw_mat,
            instances: Vec::new(),
            instance_buffer: None,
        }
    }
}
