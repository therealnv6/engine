use engine::render::{
    self,
    builder::pass::{RenderPassBuilder, RenderPassColorAttachmentBuilder},
    camera::{Camera, CameraBind, CameraPerspective, CameraRender},
    color::Color,
    framework::{EventLoop, Framework},
    material::color::{RawStaticColorMaterial, StaticColorMaterial},
    mesh::{Mesh, MeshRender, RawMesh},
    raw::{IntoRawBinder, RawBindingRender, RawParams},
    vertex::Vertex,
};

use glam::Vec3;
use winit::window::Window;

pub mod chunk;

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    render::framework::run::<VoxelFramework>(window, event_loop);
}

pub struct VoxelFramework {
    tri_raw_mesh: RawMesh,
    tri_mat: RawStaticColorMaterial,
    // camera
    camera: Camera,
    bind_camera: CameraPerspective,
    raw_bind_camera: CameraBind,
}

const VERTICES: &[([f32; 3], [f32; 3])] = &[
    ([-0.0868241, 0.49240386, 0.0], [0.5, 0.0, 0.5]), // A
    ([-0.49513406, 0.06958647, 0.0], [0.5, 0.0, 0.5]), // B
    ([-0.21918549, -0.44939706, 0.0], [0.5, 0.0, 0.5]), // C
    ([0.35966998, -0.3473291, 0.0], [0.5, 0.0, 0.5]), // D
    ([0.44147372, 0.2347359, 0.0], [0.5, 0.0, 0.5]),  // E
];

impl Framework for VoxelFramework {
    fn init(
        config: &wgpu::SurfaceConfiguration,
        _: &wgpu::Adapter,
        device: &wgpu::Device,
        _: &wgpu::Queue,
    ) -> Self {
        let tri_mesh = Mesh::builder()
            .vertices(
                VERTICES
                    .iter()
                    .map(|v| {
                        Vertex::builder()
                            .position(v.0)
                            .normal([0.0, 0.0, 0.0])
                            .build()
                    })
                    .collect::<Vec<_>>(),
            )
            .indices(vec![0, 1, 4, 1, 2, 4, 2, 3, 4, 0])
            .build();

        let camera = Camera::builder()
            .eye([1.0, 1.0, 2.0].into())
            .target([0.0, 0.0, 0.0].into())
            .up(Vec3::Y)
            .aspect(config.width as f32 / config.height as f32)
            .fovy(45.0)
            .znear(0.1)
            .zfar(100.0)
            .build();

        let bind_camera = CameraPerspective::new();

        let raw_bind_camera =
            bind_camera.create_raw_bind(device, bytemuck::cast_slice(&[bind_camera]));

        let params: RawParams = (device, config, &raw_bind_camera).into();

        // we want to get the "raw" mesh here, so we don't create new buffers every single time we make a new raw mesh.
        let tri_raw_mesh = tri_mesh.to_raw(device);
        let tri_mat = StaticColorMaterial::builder()
            .color([1.0, 0.0, 1.0, 0.3].into())
            .build()
            .into_raw(&params);
        Self {
            tri_raw_mesh,
            tri_mat,
            // camewa uwu
            camera,
            bind_camera,
            raw_bind_camera,
        }
    }

    fn render(
        &mut self,
        time: &render::time::Time,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        let clear_attachment = RenderPassColorAttachmentBuilder::builder()
            .ops(wgpu::Operations {
                load: wgpu::LoadOp::Clear(Color::from([0.5, 0.2, 0.3, 1.0]).wgpu()),
                store: true,
            })
            .build()
            .into_color_attachment_opt(view);

        let tri_attachments = [clear_attachment];
        let mut render_pass = RenderPassBuilder::builder()
            .label("voxel pass")
            .color_attachments(&tri_attachments)
            .build()
            .begin(encoder);

        self.camera.eye.x += 5.0 * time.delta_seconds_f32();
        self.bind_camera.update_view_proj(&self.camera);
        self.raw_bind_camera
            .update_buffer(queue, bytemuck::cast_slice(&[self.bind_camera]));

        render_pass.bind_raw(0, &self.tri_mat);
        render_pass.bind_camera(1, &self.raw_bind_camera);
        render_pass.render_mesh(0, &self.tri_raw_mesh);
    }
}
