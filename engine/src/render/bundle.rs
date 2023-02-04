use std::marker::PhantomData;

use typed_builder::TypedBuilder;

use super::{
    material::color::{RawStaticColorMaterial, StaticColorMaterial},
    mesh::{Mesh, MeshRender, RawMesh},
    raw::{IntoRawBinder, RawBinder, RawBindingRender},
};

#[derive(TypedBuilder)]
pub struct MeshBundle<T: IntoRawBinder> {
    pub mesh: Mesh,
    pub material: T,
}

pub struct RawMeshBundle<T: RawBinder> {
    pub(crate) mesh: RawMesh,
    pub(crate) material: T,
}

impl<T: RawBinder> RawBinder for RawMeshBundle<T> {
    fn bind_to_pass<'a>(&'a self, idx: u32, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.bind_raw(idx, &self.material);
        render_pass.render_mesh(0, &self.mesh);
    }
}

impl<T: IntoRawBinder> IntoRawBinder for MeshBundle<T> {
    type RawBinder = RawMeshBundle<<T as IntoRawBinder>::RawBinder>;

    fn into_raw(&self, params: &super::raw::RawParams) -> Self::RawBinder {
        let raw_mesh = self.mesh.to_raw(params.device);
        let raw_mat = self.material.into_raw(params);

        RawMeshBundle {
            mesh: raw_mesh,
            material: raw_mat,
        }
    }
}
