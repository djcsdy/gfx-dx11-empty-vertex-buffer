#[macro_use]
extern crate gfx;
extern crate gfx_window_dxgi;
extern crate winit;

use gfx::traits::FactoryExt;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
    }
}

fn main() {
    let window_builder = winit::WindowBuilder::new();

    let events_loop = winit::EventsLoop::new();

    let (_, _, mut factory, _) =
        gfx_window_dxgi::init::<gfx::format::Srgba8>(window_builder, &events_loop).unwrap();

    // OK.
    factory.create_vertex_buffer::<Vertex>(&[Vertex { pos: [0.0, 0.0] }]);

    // Panics.
    factory.create_vertex_buffer::<Vertex>(&[]);
}
