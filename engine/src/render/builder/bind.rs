use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct BindGroupLayoutBuilder<'a> {
    #[builder(default, setter(strip_option))]
    label: Option<&'a str>,
    binding_location: u32,
    visibility: wgpu::ShaderStages,
}

impl<'a> BindGroupLayoutBuilder<'a> {
    pub fn into_bind_group_layout(&self, device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: self.label,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: self.binding_location,
                visibility: self.visibility,
                count: None,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            }],
        })
    }
}
