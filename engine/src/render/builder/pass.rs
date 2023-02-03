use typed_builder::TypedBuilder;
use wgpu::{Color, RenderPassColorAttachment, TextureView};

#[derive(TypedBuilder)]
pub struct RenderPassBuilder<'a> {
    #[builder(default, setter(strip_option))]
    label: Option<&'a str>,
    color_attachments: &'a [Option<RenderPassColorAttachment<'a>>],
    #[builder(default, setter(strip_option))]
    depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'a>>,
}

#[derive(TypedBuilder)]
pub struct RenderPassColorAttachmentBuilder<'a> {
    #[builder(default, setter(strip_option))]
    resolve_target: Option<&'a TextureView>,
    ops: wgpu::Operations<Color>,
}

impl<'a> RenderPassBuilder<'a> {
    pub fn begin(self, encoder: &'a mut wgpu::CommandEncoder) -> wgpu::RenderPass {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor::<'a, 'a> {
            label: self.label,
            color_attachments: self.color_attachments,
            depth_stencil_attachment: self.depth_stencil_attachment.clone(),
        })
    }
}

impl<'a> RenderPassColorAttachmentBuilder<'a> {
    pub fn into_color_attachment(self, view: &'a TextureView) -> RenderPassColorAttachment<'a> {
        RenderPassColorAttachment {
            view,
            resolve_target: self.resolve_target,
            ops: self.ops,
        }
    }

    pub fn into_color_attachment_opt(
        self,
        view: &'a TextureView,
    ) -> Option<RenderPassColorAttachment<'a>> {
        Some(self.into_color_attachment(view))
    }
}
