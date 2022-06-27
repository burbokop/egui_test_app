
use std::{ops::DerefMut, time::Instant};

use egui::{ClippedMesh};
use epaint::ClippedShape;
use epi;



use crate::iv_platform::{painter::Painter, inkview::NonZeroF32};

use super::inkview as iv;


pub fn run_native<A: epi::App>(mut app: Box<A>, _: epi::NativeOptions) {
    iv::open_screen();

    let mut integration = super::epi_integration::EpiIntegration::new(
        NonZeroF32::from_f32(iv::pixels_per_point32().unwrap()).unwrap(),
        Some(false)
    );

    iv::prepare_for_loop_ex(|event| -> bool {
        integration.on_event(app.deref_mut(), &event)
    });

    let mut canvas = iv::get_canvas();
    let mut painter = Painter::new(integration.pixels_per_point().to_f32());

    let join_handle = std::thread::spawn(|| {

    });

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

        iv::process_event_loop();
        //std::thread::sleep(std::time::Duration::from_millis(1000 / 30));
    }
    join_handle.join().unwrap();
    iv::clear_on_exit();
}