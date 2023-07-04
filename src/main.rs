mod model;
mod renderer;
mod service;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let window = winit::window::WindowBuilder::new()
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let mut service = service::Service::new();
    let mut renderer = pollster::block_on(renderer::Renderer::new_async(&window));
    let mut input = winit_input_helper::WinitInputHelper::new();

    use winit::event::Event;
    use winit::event::WindowEvent;
    event_loop.run(move |event, _, control_flow| {
        input.update(&event);

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                service.update(&input);
                renderer.draw(&service);
            }
            Event::RedrawEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                _ => {}
            },
            _ => {}
        }
    })
}
