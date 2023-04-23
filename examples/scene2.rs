use fltk::{app, prelude::*, window::Window};
use rs_software_renderer::{
    buffer_attachment::BufferAttachment, line::*, math::*, renderer::*, shader::*,
};

const W: i32 = 1400;
const H: i32 = 787;

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
    let mut renderer = Renderer::new(W as usize, H as usize);
    let mut wind = Window::new(100, 100, W, H, "Draw Triangle");

    let red = *Attributes::default().set_color(Vec3::new(255., 0., 0.));
    let blue = *Attributes::default().set_color(Vec3::new(0., 0., 255.));
    let green = *Attributes::default().set_color(Vec3::new(0., 255., 0.));
    let v1 = *Vertex::new(Vec3::new(-0.5, 0., 0.)).set_attr(green);
    let v2 = *Vertex::new(Vec3::new(0.5, 0., 0.)).set_attr(blue);
    let v3 = *Vertex::new(Vec3::new(0., 0.5, 0.)).set_attr(red);
    let triangle = [v1, v2, v3];

    wind.draw(move |_| {
        renderer.clear();
        renderer.draw_triangle(triangle);
        swap_context(renderer.get_color_attachment())
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}
