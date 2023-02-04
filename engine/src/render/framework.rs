use std::time::{Duration, Instant};

use wgpu::{Backends, InstanceDescriptor};
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, DeviceId, Event, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use super::time::Time;

pub type EventLoop = winit::event_loop::EventLoop<()>;
pub struct WgpuConstruct(
    Window,
    EventLoop,
    wgpu::Instance,
    PhysicalSize<u32>,
    wgpu::Surface,
    wgpu::Adapter,
    wgpu::Device,
    wgpu::Queue,
);

pub enum FramerateLimit {
    Unlimited,
    Limited(usize),
}

pub trait Framework: 'static + Sized {
    fn init(
        config: &wgpu::SurfaceConfiguration,
        adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self;

    fn resize(
        &mut self,
        _config: &wgpu::SurfaceConfiguration,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) {
    }

    fn render(
        &mut self,
        time: &Time,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    );

    fn maximum_framerate(&self) -> FramerateLimit {
        FramerateLimit::Unlimited
    }

    fn input(&mut self, _device_id: DeviceId, _event: DeviceEvent) {}

    fn on_event(&mut self, _event: WindowEvent, _control_flow: &mut ControlFlow) {}
}

// #[autodefault::autodefault]
pub async fn init_wgpu<T: Framework>(window: Window, event_loop: EventLoop) -> WgpuConstruct {
    env_logger::init();

    let instance = wgpu::Instance::new(InstanceDescriptor {
        backends: Backends::all(),
        ..Default::default()
    }); // this will automatically set the Backends to "all", and use the default 'Dx12Compiler'.
    let size = window.inner_size();

    let surface =
        unsafe { instance.create_surface(&window) }.expect("Unable to instantiate surface!");

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::DEPTH_CLIP_CONTROL,
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        )
        .await
        .unwrap();

    WgpuConstruct(
        window, event_loop, instance, size, surface, adapter, device, queue,
    )
}

pub fn run<T: Framework>(window: Window, event_loop: EventLoop) {
    let WgpuConstruct(window, event_loop, instance, size, surface, adapter, device, queue) =
        pollster::block_on(init_wgpu::<T>(window, event_loop));

    let capibilities = surface.get_capabilities(&adapter);
    let surface_format = capibilities
        .formats
        .iter()
        .copied()
        .filter(|f| f.describe().srgb)
        .next()
        .unwrap_or(capibilities.formats[0]);

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: capibilities.present_modes[0],
        alpha_mode: capibilities.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    let mut framework = T::init(&config, &adapter, &device, &queue);
    let mut time = Time::new();

    let mut fps_counter = 0;
    let mut fps_counter_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        fps_counter += 1;

        if fps_counter_time.elapsed().as_secs() >= 1 {
            let fps = fps_counter as f64 / fps_counter_time.elapsed().as_secs_f64();

            window.set_title(&format!("Engine [{:.2} fps] (todo)", fps));

            fps_counter = 0;
            fps_counter_time = Instant::now();
        }

        *control_flow = ControlFlow::Poll;

        let _ = (&instance, &adapter);

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::Resized(size)
                    | WindowEvent::ScaleFactorChanged {
                        new_inner_size: &mut size,
                        ..
                    } => {
                        config.width = size.width.max(1);
                        config.height = size.height.max(1);

                        framework.resize(&config, &device, &queue);
                        surface.configure(&device, &config);
                    }
                    _ => (),
                };
                framework.on_event(event, control_flow);
            }
            Event::DeviceEvent { device_id, event } => framework.input(device_id, event),
            Event::RedrawRequested(_) => {
                let frame = match surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(_) => {
                        surface.configure(&device, &config);
                        surface
                            .get_current_texture()
                            .expect("Failed to acquire next surface texture!")
                    }
                };
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

                framework.render(&time, &mut encoder, &view, &device, &queue);
                queue.submit(std::iter::once(encoder.finish()));

                frame.present();
                time.post_update();
            }
            Event::RedrawEventsCleared => {
                if let FramerateLimit::Limited(fps) = framework.maximum_framerate() {
                    let target_frametime = Duration::from_secs_f64(1.0 / (fps as f64));
                    let now = Instant::now();
                    let delta = time.time_delta();

                    if delta >= target_frametime {
                        window.request_redraw();
                        time.post_update();
                    } else {
                        *control_flow = ControlFlow::WaitUntil(now + target_frametime - delta);
                    }
                }
            }
            _ => (),
        }
    });
}
