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
        (device, config, raw_camera): (
            &'a wgpu::Device,
            &'a wgpu::SurfaceConfiguration,
            &'a CameraBind,
        ),
    ) -> Self {
        Self {
            device,
            config,
            raw_camera,
        }
    }
}

pub trait RawBinder
where
    Self: Sized,
{
    fn bind_to_pass<'a>(&'a self, idx: u32, render_pass: &mut wgpu::RenderPass<'a>);
}

pub trait IntoRawBinder
where
    Self: Sized,
{
    type RawBinder: RawBinder;
    fn into_raw(&self, params: &RawParams) -> Self::RawBinder;
}

pub trait RawBindingRender<'a, T: RawBinder> {
    fn bind_raw(&mut self, idx: u32, value: &'a T);
}

impl<'a, T: RawBinder> RawBindingRender<'a, T> for wgpu::RenderPass<'a> {
    fn bind_raw(&mut self, idx: u32, value: &'a T) {
        value.bind_to_pass(idx, self);
    }
}
