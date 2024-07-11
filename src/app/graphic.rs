use std::sync::Arc;
use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};
use winit::window::Window;
use crate::control::navigation::NavigationEvent;
use crate::draw::geometry::GeometryPainter;
use crate::draw::painter::{Painter, PainterLibrary};
use crate::geometry::size::Size;

pub(crate) struct GraphicContext {
    pub(crate) window: Arc<Window>,
    pub(crate) surface: Surface<'static>,
    pub(crate) surface_config: SurfaceConfiguration,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) painter_library: PainterLibrary,
}

impl GraphicContext {
    pub async fn with_window(window: Window) -> GraphicContext {
        let instance = Instance::default();
        let win = Arc::new(window);
        let surface = instance.create_surface(win.clone()).unwrap();

        let (adapter, device, queue) = Self::init_device(instance, &surface).await;

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let mut size = win.inner_size();
        size.width = size.width.max(100);
        size.height = size.height.max(100);

        let geometry_painter = GeometryPainter::create(
            swapchain_format.into(),
            &device,
            size.width,
            size.height
        );

        let surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        surface.configure(&device, &surface_config);

        GraphicContext {
            window: win,
            surface,
            surface_config,
            device,
            queue,
            painter_library: PainterLibrary::Geometry(geometry_painter),
        }
    }

    async fn init_device(instance: Instance, surface: &Surface<'_>) -> (Adapter, Device, Queue) {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        (adapter, device, queue)
    }

    pub(crate) fn draw(&mut self) {
        let frame = self.surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.painter_library.draw(&mut self.queue, &self.device, &view);

        frame.present();
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) -> Size {
        let width = width.clamp(100, 4_000);
        let height = height.clamp(100, 4_000);
        self.surface_config.width = width;
        self.surface_config.height = height;

        self.surface.configure(&self.device, &self.surface_config);

        let size = Size::new_uint(width, height);
        self.painter_library.update_size(size);

        // On macos the window needs to be redrawn manually after resizing
        self.window.request_redraw();

        size
    }

    pub(crate) fn process_navigation_event(&mut self, navigation_event: NavigationEvent) {
        self.painter_library.navigation_event(navigation_event);
    }

}