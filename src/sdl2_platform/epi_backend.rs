

use std::ops::DerefMut;

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

pub fn run_native<A: epi::App>(app: Box<A>, native_options: epi::NativeOptions) -> ! {
    //println!("debug: {:?} -> {:?}", app, native_options.initial_window_pos);

    let mut app_mut = app;

    let mut integration = super::epi_integration::EpiIntegration::new(None);


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
        for event in event_pump.poll_iter() {
            integration.on_event(app_mut.deref_mut(), &event)
        }



        let egui::FullOutput {
            platform_output,
            needs_repaint,
            textures_delta,
            shapes,
        } = integration.update(app_mut.as_mut(), w, h);


        if needs_repaint {

            painter.paint_and_update_textures(shapes, &textures_delta, &mut canvas);

/*
            for (id, image_delta) in &textures_delta.set {
                let p = &image_delta.pos;
    
                match &image_delta.image {
                    egui::ImageData::Color(image) => {
                        println!("\tcolor image: {:?}", image.size);
                    },
                    egui::ImageData::Alpha(image) => {
                        println!("\talpha image: {:?}", image.size);

                        let a = shapes.iter().cloned().map(clipped_mesh_from_shape).filter_map(identity);

                        //painter.paint_jobs(Some(Color32::GRAY), a.collect(), 0, &image)


                    }
                }
            }
*/

        }

        println!("needs_repaint: {:?}", needs_repaint);

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    //inkview::clear_on_exit()
}