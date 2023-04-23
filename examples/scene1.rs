use fltk::{app, prelude::*, window::Window};
use rs_software_renderer::{
    buffer_attachment::BufferAttachment, line::*, math::*, renderer::*, shader::*,
};

const W: usize = 800;
const H: usize = 600;

fn swap_context(img: &BufferAttachment<Vec3<u8>>) {
    let data: Vec<u8> = img
        .buffer()
        .into_iter()
        .flat_map(|v| vec![v.x, v.y, v.z])
        .collect();
    fltk::draw::draw_image(
        &data,
        0,
        0,
        W as i32,
        H as i32,
        fltk::enums::ColorDepth::Rgb8,
    )
    .unwrap();
    // dbg!(data);
}

fn main() {
    let app = app::App::default();
    let mut renderer = Renderer::new(W, H);
    let mut wind = Window::new(100, 100, W as i32, H as i32, "Hello from rust");
    let origin = Vertex::new(Vec3::from(0.));

    let mut lines = Vec::new();
    let mut red = Attributes::default();
    red.set_color(Vec3::new(255., 0., 0.));
    let mut blue = Attributes::default();
    blue.set_color(Vec3::new(0., 0., 255.));
    lines.push(Line::new(origin, Vertex::new(Vec3::new(1., 0.3, 0.))));
    lines.push(Line::new(origin, Vertex::new(Vec3::new(-1., 0.3, 0.))));
    lines.push(Line::new(origin, Vertex::new(Vec3::new(1., -0.3, 0.))));
    lines.push(Line::new(origin, Vertex::new(Vec3::new(-1., -0.3, 0.))));
    lines.push(Line::new(origin, Vertex::new(Vec3::new(0.3, 1., 0.))));
    lines.push(Line::new(origin, Vertex::new(Vec3::new(0.3, -1., 0.))));
    lines.push(Line::new(origin, Vertex::new(Vec3::new(-0.3, 1., 0.))));
    lines.push(Line::new(origin, Vertex::new(Vec3::new(-0.3, -1., 0.))));
    lines.push(Line::new(
        *Vertex::new(Vec3::new(-1., 0., 0.)).set_attr(red),
        *Vertex::new(Vec3::new(1., 0., 0.)).set_attr(blue),
    ));
    lines.push(Line::new(
        Vertex::new(Vec3::new(0., -1., 0.)),
        Vertex::new(Vec3::new(0., 1., 0.)),
    ));

    wind.draw(move |_| {
        renderer.clear();
        for line in lines.iter() {
            renderer.draw_line(line);
        }
        swap_context(renderer.get_color_attachment())
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}
