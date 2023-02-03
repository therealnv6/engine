use engine::render::{
    self,
    builder::{
        pass::{RenderPassBuilder, RenderPassColorAttachmentBuilder},
    },
    color::Color,
    framework::{EventLoop, Framework},
    material::{
        color::{RawStaticColorMaterial, StaticColorMaterial}, RawMaterialRender, ToRawMaterial,
    },
    mesh::{Mesh, MeshRender, RawMesh},
    vertex::{Vertex},
};

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

        assert_eq!(
            tri_mesh
                .vertices
                .iter()
                .flat_map(|vertex| vertex.position)
                .collect::<Vec<f32>>(),
            VERTICES
                .iter()
                .flat_map(|vertex| vertex.0)
                .collect::<Vec<f32>>()
        );

        // we want to get the "raw" mesh here, so we don't create new buffers every single time we make a new raw mesh.
        let tri_raw_mesh = tri_mesh.to_raw(device);
        let tri_mat = StaticColorMaterial::builder()
            .color([1.0, 0.0, 0.0, 1.0].into())
            .build()
            .to_raw(device, config);

        Self {
            tri_raw_mesh,
            tri_mat,
        }
    }

    fn render(
        &mut self,
        _: &render::time::Time,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        _: &wgpu::Device,
        _: &wgpu::Queue,
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

        render_pass.draw_material(&self.tri_mat);
        render_pass.render_mesh(0, &self.tri_raw_mesh);
    }
}
