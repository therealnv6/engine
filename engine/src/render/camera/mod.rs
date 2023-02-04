use glam::{Mat4, Vec3, Vec4};
use typed_builder::TypedBuilder;
use wgpu::{util::DeviceExt, RenderPass};

#[rustfmt::skip]
const TRANSLATION_MATRIX: Mat4 = Mat4::from_cols(
    Vec4::new(1.0, 0.0, 0.0, 0.0),
    Vec4::new(0.0, 1.0, 0.0, 0.0),
    Vec4::new(0.0, 0.0, 0.5, 0.0),
    Vec4::new(0.0, 0.0, 0.5, 1.0),
);

#[derive(TypedBuilder)]
pub struct Camera {
    eye: Vec3,
    target: Vec3,
    up: Vec3,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraPerspective {
    pub(crate) view_projection: [[f32; 4]; 4],
}

pub struct CameraBind {
    pub(crate) bind_group: wgpu::BindGroup,
}

impl Camera {
    pub fn build_view_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = Mat4::perspective_rh(self.fovy.to_radians(), self.aspect, self.znear, self.zfar);

        return TRANSLATION_MATRIX * proj * view;
    }
}

impl CameraPerspective {
    pub fn new() -> Self {
        Self {
            view_projection: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_projection = camera.build_view_matrix().to_cols_array_2d();
    }

    pub fn create_raw_bind<'a>(&self, device: &wgpu::Device, contents: &'a [u8]) -> CameraBind {
        CameraBind::new(device, contents)
    }
}

impl CameraBind {
    pub fn new<'a>(device: &wgpu::Device, contents: &'a [u8]) -> Self {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("camera_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("camera bind"),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            contents,
        });

        Self {
            bind_group: device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
                label: Some("camera_bind_group"),
            }),
        }
    }

    pub fn bind_to_pass<'a>(&'a self, idx: u32, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_bind_group(idx, &self.bind_group, &[]);
    }
}

pub trait CameraRender<'a> {
    fn bind_camera(&mut self, idx: u32, camera: &'a CameraBind);
}

impl<'a> CameraRender<'a> for RenderPass<'a> {
    fn bind_camera(&mut self, idx: u32, camera: &'a CameraBind) {
        camera.bind_to_pass(idx, self);
    }
}
