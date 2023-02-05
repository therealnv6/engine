use typed_builder::TypedBuilder;

use super::{
    mesh::{Mesh, MeshRender, RawMesh},
    raw::{IntoRawBinder, RawBinder, RawBindingRender},
    vertex::{Transform, TransformRaw},
};

#[derive(TypedBuilder)]
pub struct MeshBundle<T: IntoRawBinder> {
    pub mesh: Mesh,
    #[builder(default, setter(strip_option))]
    pub transform: Option<Transform>,
    pub material: T,
}

pub struct RawMeshBundle<T: RawBinder> {
    pub(crate) mesh: RawMesh,
    pub(crate) transform: Option<TransformRaw>,
    pub(crate) material: T,
}

impl<T: RawBinder> RawBinder for RawMeshBundle<T> {
    fn bind_to_pass<'a>(&'a self, idx: u32, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.bind_raw(idx, &self.material);

        match &self.transform {
            Some(transform) => {
                render_pass.render_instanced_mesh(0, transform, &self.mesh);
            }
            None => {
                render_pass.render_single_mesh(0, &self.mesh);
            }
        }
    }
}

impl<T: IntoRawBinder> IntoRawBinder for MeshBundle<T> {
    type RawBinder = RawMeshBundle<<T as IntoRawBinder>::RawBinder>;

    fn into_raw(&self, params: &super::raw::RawParams) -> Self::RawBinder {
        let raw_mesh = self.mesh.to_raw(params.device);
        let raw_mat = self.material.into_raw(params);
        let raw_transform = self
            .transform
            .as_ref()
            .map(|transform| transform.to_raw(params));

        RawMeshBundle {
            mesh: raw_mesh,
            material: raw_mat,
            transform: raw_transform,
        }
    }
}
