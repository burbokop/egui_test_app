
use std::{ops::DerefMut, time::Instant};

use egui::{ClippedMesh};
use epaint::ClippedShape;
use epi;



use crate::iv_platform::{painter::Painter, inkview::NonZeroF32};

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

pub fn run_native<A: epi::App>(mut app: Box<A>, _: epi::NativeOptions) {
    inkview::open_screen();

    let mut integration = super::epi_integration::EpiIntegration::new(
        NonZeroF32::from_f32(pixels_per_point32()).unwrap(),
        Some(false)
    );

    inkview::prepare_for_loop_ex(|event| -> bool {
        integration.on_event(app.deref_mut(), &event)
    });

    let mut canvas = inkview::get_canvas();
    let mut painter = Painter::new(integration.pixels_per_point().to_f32());

    while !integration.should_quit() {
        let egui::FullOutput {
            platform_output,
            needs_repaint,
            textures_delta,
            shapes,
        } = integration.update(app.as_mut());

        if needs_repaint {
            painter.paint_and_update_textures(&mut canvas, shapes, &textures_delta);
        }

        inkview::process_event_loop();
        //std::thread::sleep(std::time::Duration::from_millis(1000 / 30));
    }
    inkview::clear_on_exit();
}