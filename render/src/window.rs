use wgpu::winit::{
    ElementState,
    Event as WinitEvent,
    EventsLoop,
    KeyboardInput,
    VirtualKeyCode,
    WindowEvent,
};
use wgpu::winit::Window as WinitWindow;

use wgpu::{Surface, Instance, Device, SwapChain, RenderPipeline, BindGroup};

pub use wgpu::winit::VirtualKeyCode as KeyCode;

pub struct Window {
    events_loop: EventsLoop,
    instance: Instance,
    window: WinitWindow,
    surface: Surface,
    device: Device,
    render_pipeline: RenderPipeline,
    bind_group: BindGroup,
    swap_chain: SwapChain
}

impl Window {
    pub fn new() -> Self {
        let mut events_loop = EventsLoop::new();

        let instance = wgpu::Instance::new();

        let window = WinitWindow::new(&events_loop).expect("Expected events_loop");
        let size = window
            .get_inner_size()
            .unwrap()
            .to_physical(window.get_hidpi_factor());
        
        let surface = instance.create_surface(&window);

        let adapter = instance.get_adapter(&wgpu::AdapterDescriptor {
            power_preference: wgpu::PowerPreference::LowPower,
        });

        let mut device = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        });

        let vs_bytes = include_bytes!("shader.vert.spv");
        let vs_module = device.create_shader_module(vs_bytes);
        let fs_bytes = include_bytes!("shader.frag.spv");
        let fs_module = device.create_shader_module(fs_bytes);

        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { bindings: &[] });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: wgpu::PipelineStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::PipelineStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            },
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: wgpu::TextureFormat::Bgra8Unorm,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[],
            sample_count: 1,
        });

        

        let mut swap_chain = device.create_swap_chain(
            &surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8Unorm,
                width: size.width.round() as u32,
                height: size.height.round() as u32,
                present_mode: wgpu::PresentMode::Vsync,
            },
        );

        Window {
            events_loop,
            instance,
            window,
            surface,
            device,
            render_pipeline,
            bind_group,
            swap_chain
        }
    }

    pub fn poll_events<F>(&mut self, mut callback: F) where F: FnMut(Event) {
        self.events_loop.poll_events(|event| {
            callback(Event::from(event))
        });

    }

    pub fn draw(&mut self) {
        let frame = self.swap_chain.get_next_texture();
        let mut encoder =
            self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::GREEN,
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.draw(0 .. 3, 0 .. 1);
        }

        self.device.get_queue().submit(&[encoder.finish()]);
    }
}

#[derive(Debug)]
pub enum Event {
    KeyPressed(KeyCode),
    KeyReleased(KeyCode),
    Close,
    Unknown
}

impl From<WinitEvent> for Event {
    fn from(item: WinitEvent) -> Event {
        match item {
            WinitEvent::WindowEvent {event, ..} => match event {
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        virtual_keycode: Some(code),
                        state,
                        ..
                    },
                    ..
                } => match state {
                    ElementState::Pressed => Event::KeyPressed(code),
                    ElementState::Released => Event::KeyReleased(code)
                },
                WindowEvent::CloseRequested => Event::Close,
                _ => Event::Unknown
            },
            _ => Event::Unknown
        }
    }

}
