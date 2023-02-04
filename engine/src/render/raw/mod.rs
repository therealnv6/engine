use typed_builder::TypedBuilder;

use super::camera::CameraBind;

pub mod storage;

#[derive(TypedBuilder)]
pub struct RawParams<'a> {
    pub device: &'a wgpu::Device,
    pub config: &'a wgpu::SurfaceConfiguration,
    pub raw_camera: &'a CameraBind,
}

impl<'a>
    From<(
        &'a wgpu::Device,
        &'a wgpu::SurfaceConfiguration,
        &'a CameraBind,
    )> for RawParams<'a>
{
    fn from(
        value: (
            &'a wgpu::Device,
            &'a wgpu::SurfaceConfiguration,
            &'a CameraBind,
        ),
    ) -> Self {
        Self {
            device: value.0,
            config: value.1,
            raw_camera: value.2,
        }
    }
}

pub trait RawBinder<T: IntoRawBinder<Self>>
where
    Self: Sized,
{
    fn bind_to_pass<'a>(&'a self, idx: u32, render_pass: &mut wgpu::RenderPass<'a>);
}

pub trait IntoRawBinder<T: RawBinder<Self>>
where
    Self: Sized,
{
    fn into_raw(&self, params: &RawParams) -> T;
}

pub trait RawBindingRender<'a, T: RawBinder<U>, U: IntoRawBinder<T>> {
    fn bind_raw(&mut self, idx: u32, value: &'a T);
}

impl<'a, T: RawBinder<U>, U: IntoRawBinder<T>> RawBindingRender<'a, T, U> for wgpu::RenderPass<'a> {
    fn bind_raw(&mut self, idx: u32, value: &'a T) {
        value.bind_to_pass(idx, self);
    }
}
