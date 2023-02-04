pub mod color;

pub trait ToRawMaterial<T: RawMaterial> {
    fn to_raw(&self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> T;
}

pub trait RawMaterial {
    fn bind_to_pass<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}

pub trait RawMaterialRender<'a, T: RawMaterial> {
    fn bind_material(&mut self, material: &'a T);
}

impl<'a, T: RawMaterial> RawMaterialRender<'a, T> for wgpu::RenderPass<'a> {
    fn bind_material(&mut self, material: &'a T) {
        material.bind_to_pass(self);
    }
}
