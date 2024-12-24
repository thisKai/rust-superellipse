use std::num::NonZeroU32;

use skia_safe::{Canvas, Color, ImageInfo, Paint};
use softbuffer::{Context, Surface};
use superellipse::{Point, Superellipse};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event_loop::{EventLoop, OwnedDisplayHandle},
    window::{Window, WindowAttributes},
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

#[derive(Default)]
struct App {
    state: Option<AppState>,
}
impl App {
    fn draw(&mut self) {
        self.unwrap_state_mut().draw(|canvas, size| {
            canvas.clear(Color::BLACK);

            let half_width = size.width as f32 / 2.0;
            let half_height = size.height as f32 / 2.0;

            let superellipse = Superellipse {
                exponent: 4.0,
                center: Point {
                    x: half_width,
                    y: half_height,
                },
                radius: Point {
                    x: half_width - 10.0,
                    y: half_height - 10.0,
                },
            };

            canvas.draw_path(
                &superellipse.skia_path(),
                Paint::default().set_color(Color::CYAN).set_anti_alias(true),
            );
        });
    }
    fn unwrap_state_mut(&mut self) -> &mut AppState {
        self.state.as_mut().unwrap()
    }
    fn unwrap_surface_mut(&mut self) -> &mut Surface<OwnedDisplayHandle, Window> {
        &mut self.state.as_mut().unwrap().surface
    }
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let display = event_loop.owned_display_handle();
        let context = Context::new(display).unwrap();

        let window = event_loop
            .create_window(WindowAttributes::default())
            .unwrap();

        let surface = Surface::new(&context, window).unwrap();

        self.state = Some(AppState { context, surface });
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::Resized(new_size) => {
                let Some((width, height)) =
                    NonZeroU32::new(new_size.width).zip(NonZeroU32::new(new_size.height))
                else {
                    return;
                };
                self.unwrap_surface_mut().resize(width, height).unwrap()
            }
            winit::event::WindowEvent::RedrawRequested => {
                self.draw();
            }
            _ => {}
        }
    }
}

struct AppState {
    context: Context<OwnedDisplayHandle>,
    surface: Surface<OwnedDisplayHandle, Window>,
}
impl AppState {
    fn draw(&mut self, mut f: impl FnMut(&Canvas, PhysicalSize<u32>)) {
        let size = self.surface.window().inner_size();
        let mut buffer = self.surface.buffer_mut().unwrap();
        let (prefix, pixels, suffix) = unsafe { buffer.align_to_mut::<u8>() };
        assert!(prefix.is_empty());
        assert!(suffix.is_empty());

        let mut skia_surface = skia_safe::surfaces::wrap_pixels(
            &ImageInfo::new_n32_premul((size.width as i32, size.height as i32), None),
            pixels,
            None,
            None,
        )
        .unwrap();
        let canvas = skia_surface.canvas();

        f(&canvas, size);

        buffer.present().unwrap();
    }
}
