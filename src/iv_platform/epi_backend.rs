
use std::{convert::identity, ops::DerefMut};

use egui::{ClippedMesh, Color32};
use epaint::ClippedShape;
use epi;



use crate::iv_platform::painter::Painter;

use super::inkview;

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

    let mut integration = super::epi_integration::EpiIntegration::new(None);

    inkview::prepare_for_loop_ex(|event| -> bool {
        integration.on_event(app.deref_mut(), &event)
    });

    //inkview::set_hard_timer("fffff", || println!("GOGADODA"), 1000);

    let mut canvas = inkview::get_canvas();

    println!("screen size: {{ {}, {} }}", inkview::screen_width().unwrap(), inkview::screen_height().unwrap());

    println!("canvas: {:?}", canvas);

    println!("inkview::get_screen_scale_factor2(): {:?}", inkview::get_screen_scale_factor2());
    println!("inkview::get_screen_dpi2(): {:?}", inkview::get_screen_dpi2());

    //&mut canvas, 1., ShaderVersion::Default

    let mut painter = Painter::new().unwrap();

    let font = inkview::open_font(inkview::get_default_font(inkview::FontType::Std), 10, 1);
    

    loop {
        let egui::FullOutput {
            platform_output,
            needs_repaint,
            textures_delta,
            shapes,
        } = integration.update(app.as_mut());


        if needs_repaint {

            painter.paint_and_update_textures(shapes, &textures_delta, &mut canvas, &font);

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

        inkview::process_event_loop();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}