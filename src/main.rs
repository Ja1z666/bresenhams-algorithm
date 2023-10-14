use bresenhams_algorithm::{Circle, Ellipse, Line, Scene, Vector, HEIGHT, WIDTH};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

enum Tool {
    Line,
    Circle,
    Ellipse,
}

#[derive(Clone)]
struct Mouse(Vector, Vector);

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = WindowBuilder::new()
        .with_title("Bresenhams algorithm")
        .with_inner_size(LogicalSize::new(WIDTH as u32, HEIGHT as u32))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut scene = Scene::new(Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap());
    let mut tool = Tool::Line;

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            scene.clear();
            for figure in &scene.scene {
                figure.draw(scene.screen.frame_mut());
            }

            if let Err(_) = scene.screen.render() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        let (mouse_cell, _) = input
            .mouse()
            .map(|(mx, my)| {
                let (dx, dy) = input.mouse_diff();
                let prev_x = mx - dx;
                let prev_y = my - dy;

                let (mx_i, my_i) = scene
                    .screen
                    .window_pos_to_pixel((mx, my))
                    .unwrap_or_else(|pos| scene.screen.clamp_pixel_pos(pos));

                let (px_i, py_i) = scene
                    .screen
                    .window_pos_to_pixel((prev_x, prev_y))
                    .unwrap_or_else(|pos| scene.screen.clamp_pixel_pos(pos));

                (
                    (mx_i as isize, my_i as isize),
                    (px_i as isize, py_i as isize),
                )
            })
            .unwrap_or_default();

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Q) {
                tool = Tool::Line;
            }

            if input.key_pressed(VirtualKeyCode::W) {
                tool = Tool::Circle;
            }

            if input.key_pressed(VirtualKeyCode::E) {
                tool = Tool::Ellipse;
            }

            if input.key_pressed(VirtualKeyCode::C) {
                scene.scene = Vec::new();
            }

            if input.mouse_pressed(0) {
                match tool {
                    Tool::Line => scene.push(Line::new(
                        Vector::new(mouse_cell.0 as i32, mouse_cell.1 as i32),
                        Vector::new(mouse_cell.0 as i32, mouse_cell.1 as i32),
                    )),
                    Tool::Circle => scene.push(Circle::new(
                        Vector::new(mouse_cell.0 as i32, mouse_cell.1 as i32),
                        0,
                    )),
                    Tool::Ellipse => scene.push(Ellipse::new(
                        Vector::new(mouse_cell.0 as i32, mouse_cell.1 as i32),
                        Vector::new(0, 0),
                    )),
                }
            }

            if input.mouse_held(0) {
                let len = scene.scene.len();
                scene.scene[len - 1]
                    .endpoint(Vector::new(mouse_cell.0 as i32, mouse_cell.1 as i32));
            }
        }

        window.request_redraw();
    });
}
