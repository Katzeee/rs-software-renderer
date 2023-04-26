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

    let col1 = *Attributes::default().set_color(Vec3::new(217., 238., 185.));
    let col2 = *Attributes::default().set_color(Vec3::new(185., 217., 238.));

    let mut vertices = Vec::new();
    vertices.push(*Vertex::new(Vec3::new(2., 0., -2.)).set_attr(col1));
    vertices.push(*Vertex::new(Vec3::new(0., 2., -2.)).set_attr(col1));
    vertices.push(*Vertex::new(Vec3::new(-2., 0., -2.)).set_attr(col1));
    vertices.push(*Vertex::new(Vec3::new(3.4, -1., -5.)).set_attr(col2));
    vertices.push(*Vertex::new(Vec3::new(2.5, 1.5, -5.)).set_attr(col2));
    vertices.push(*Vertex::new(Vec3::new(-1., 0.5, -5.)).set_attr(col2));
    let indices = vec![0, 1, 2, 3, 4, 5];

    // renderer.should_show_depth = true;
    renderer.should_draw_line = false;

    wind.draw(move |_| {
        renderer.clear();
        renderer.draw(&mut vertices, &indices);
        swap_context(renderer.get_color_attachment())
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}
