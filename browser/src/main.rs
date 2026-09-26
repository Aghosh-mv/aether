use aether_css::parse_stylesheet;
use aether_html::parse;
use aether_layout::layout_with_styles;
use aether_paint::paint;
use softbuffer::{Context, Surface};
mod bookmarks;
mod downloads;
mod history;
mod navigation;
mod passwords;
mod preferences;
mod storage;
mod tabs;
use std::env;
use std::num::NonZeroU32;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

fn fetch(url: &str) -> Result<String, String> {
    ureq::get(url)
        .call()
        .map_err(|e| e.to_string())?
        .body_mut()
        .read_to_string()
        .map_err(|e| e.to_string())
}
struct App {
    window: Option<std::sync::Arc<Window>>,
    html: String,
    styles: Vec<aether_css::Rule>,
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = std::sync::Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Aether")
                        .with_inner_size(winit::dpi::LogicalSize::new(900, 650)),
                )
                .unwrap(),
        );
        self.window = Some(window);
        self.window.as_ref().unwrap().request_redraw();
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let Some(window) = &self.window else { return };
                let size = window.inner_size();
                let context = Context::new(window.clone()).unwrap();
                let mut surface = Surface::new(&context, window.clone()).unwrap();
                surface
                    .resize(
                        NonZeroU32::new(size.width.max(1)).unwrap(),
                        NonZeroU32::new(size.height.max(1)).unwrap(),
                    )
                    .unwrap();
                let doc = parse(&self.html);
                let runs = layout_with_styles(&doc, size.width, &self.styles);
                let frame = paint(&runs, size.width, size.height);
                let mut buffer = surface.buffer_mut().unwrap();
                buffer.copy_from_slice(&frame.pixels);
                buffer.present().unwrap();
            }
            _ => {}
        }
    }
}
fn main() {
    let url = env::args()
        .nth(1)
        .unwrap_or_else(|| "https://example.com".into());
    let html = fetch(&url)
        .unwrap_or_else(|error| format!("<h1>Aether navigation error</h1><p>{error}</p>"));
    let styles = html
        .split("<style")
        .skip(1)
        .filter_map(|chunk| chunk.split_once('>'))
        .filter_map(|(_, body)| body.split_once("</style>"))
        .flat_map(|(body, _)| parse_stylesheet(body))
        .collect();
    EventLoop::new()
        .unwrap()
        .run_app(&mut App {
            window: None,
            html,
            styles,
        })
        .unwrap();
}
