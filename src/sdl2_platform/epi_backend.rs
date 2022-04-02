

use std::{ops::DerefMut, convert::identity};

use egui::{ClippedMesh};
use epaint::ClippedShape;
use epi;

use super::painter::Painter;


pub fn create_storage(_app_name: &str) -> Option<Box<dyn epi::Storage>> {
    None
}


pub fn clipped_mesh_from_shape(shape: ClippedShape) -> Option<ClippedMesh> {
    match shape.1 {
        egui::Shape::Mesh(mesh) => Some(ClippedMesh(shape.0, mesh)),
        _ => None
    }
}

pub fn run_native<A: epi::App>(mut app: Box<A>, native_options: epi::NativeOptions) -> ! {
    //println!("debug: {:?} -> {:?}", app, native_options.initial_window_pos);


    let mut integration = super::epi_integration::EpiIntegration::new(None, Some(false));


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let w = 800;
    let h = 600;

    let window = video_subsystem.window("rust-sdl2 demo", w, h)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();


    let mut painter = Painter::new().unwrap();
    

    loop {
        let events: Vec<egui::Event> = event_pump
            .poll_iter()
            .map(|event| integration.on_event(app.deref_mut(), &event))
            .filter_map(identity)
            .collect();
            
        let egui::FullOutput {
            platform_output,
            needs_repaint,
            textures_delta,
            shapes,
        } = integration.update(app.as_mut(), w, h, events);

        if needs_repaint {
            painter.paint_and_update_textures(shapes, &textures_delta, &mut canvas);
        }

        std::thread::sleep(std::time::Duration::from_millis(1000 / 30));
    }
    //inkview::clear_on_exit()
}