use tao::event::{Event::WindowEvent, WindowEvent::CloseRequested};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::WindowBuilder;
#[cfg(target_os = "linux")]
use {tao::platform::unix::WindowExtUnix, wry::WebViewBuilderExtUnix};

fn main() -> wry::Result<()> {
    let el = EventLoop::new();
    let win = WindowBuilder::new()
        .with_title("Neko Forum")
        .with_inner_size(tao::dpi::LogicalSize::new(1024.0, 768.0))
        .build(&el)
        .unwrap();
    let b = wry::WebViewBuilder::new().with_url("https://fr.neko-void-linux.online/");
    #[cfg(target_os = "linux")]
    let _w = b.build_gtk(win.default_vbox().unwrap())?;
    #[cfg(not(target_os = "linux"))]
    let _w = b.build(&win)?;
    // ponytail: en Windows usar WebViewExtWindows::set_memory_usage_level en WindowEvent::Focused
    el.run(move |e, _, cf| {
        *cf = ControlFlow::Wait;
        if let WindowEvent {
            event: CloseRequested,
            ..
        } = e
        {
            *cf = ControlFlow::Exit;
        }
    });
}
