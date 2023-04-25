use fltk::{app, prelude::*, window::Window};
use rs_software_renderer::{buffer_attachment::BufferAttachment, math::*, renderer::*, shader::*};

const W: i32 = 1000;
const H: i32 = 800;

fn swap_context(img: &BufferAttachment<Vec3<u8>>) {
    let data: Vec<u8> = img
        .buffer()
        .into_iter()
        .flat_map(|v| vec![v.x, v.y, v.z])
        .collect();
    fltk::draw::draw_image(&data, 0, 0, W, H, fltk::enums::ColorDepth::Rgb8).unwrap();
    // dbg!(data);
}

fn main() {
    let app = app::App::default();
    let mut renderer = Renderer::new(W, H);
    let mut wind = Window::new(100, 100, W, H, "Draw Triangle");

    let red = *Attributes::default().set_color(Vec3::new(255., 0., 0.));
    let blue = *Attributes::default().set_color(Vec3::new(0., 0., 255.));
    let green = *Attributes::default().set_color(Vec3::new(0., 255., 0.));
    let v1 = *Vertex::new(Vec3::new(-0.5, 0., 1.)).set_attr(green);
    let v2 = *Vertex::new(Vec3::new(0.5, 0., 0.)).set_attr(blue);
    let v3 = *Vertex::new(Vec3::new(0., 0.5, 0.5)).set_attr(red);
    let triangle1 = [v1, v2, v3];
    let v1 = *Vertex::new(Vec3::new(-0.5, 0.5, -0.8)).set_attr(green);
    let v2 = *Vertex::new(Vec3::new(0., 0., 0.8)).set_attr(red);
    let v3 = *Vertex::new(Vec3::new(0.5, 0.5, 0.5)).set_attr(blue);
    let triangle2 = [v1, v2, v3];

    // renderer.should_show_depth = true;

    wind.draw(move |_| {
        renderer.clear();
        renderer.draw_triangle(triangle1);
        renderer.draw_triangle(triangle2);
        swap_context(renderer.get_color_attachment())
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}
