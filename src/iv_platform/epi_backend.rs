
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


pub fn pixels_per_point32() -> f32 {
    inkview::get_screen_scale_factor().unwrap() as f32 * inkview::get_screen_dpi().unwrap() as f32 / 160.
}

pub fn pixels_per_point64() -> f64 {
    inkview::get_screen_scale_factor().unwrap() * inkview::get_screen_dpi().unwrap() as f64 / 160.
}

pub fn dp_to_pix32(v: f32) -> f32 {
    v * pixels_per_point32()
}
pub fn dp_to_pix64(v: f64) -> f64 {
    v * pixels_per_point64()
}

pub fn run_native<A: epi::App>(mut app: Box<A>, native_options: epi::NativeOptions) -> ! {
    //println!("debug: {:?} -> {:?}", app, native_options.initial_window_pos);

    inkview::open_screen();

    println!("AAAAA");
    println!("))): pixels_per_point32: {}", pixels_per_point32());

    let mut integration = super::epi_integration::EpiIntegration::new(None, pixels_per_point32());


    inkview::prepare_for_loop_ex(|event| -> bool {
        integration.on_event(app.deref_mut(), &event)
    });

    //inkview::set_hard_timer("fffff", || println!("GOGADODA"), 1000);

    let mut canvas = inkview::get_canvas();

    println!("screen size: {{ {}, {} }}", inkview::screen_width().unwrap(), inkview::screen_height().unwrap());

    println!("canvas: {:?}", canvas);

    println!("inkview::get_screen_scale_factor2(): {:?}", inkview::get_screen_scale_factor());
    println!("inkview::get_screen_dpi2(): {:?}", inkview::get_screen_dpi());

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
        }

        inkview::process_event_loop();
        std::thread::sleep(std::time::Duration::from_millis(1000 / 30));
    }
}