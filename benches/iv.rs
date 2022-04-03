#![feature(test)]

extern crate test;
use egui_app::iv_platform::inkview as iv;
use test::Bencher;



#[bench]
fn iv_rect_native(b: &mut Bencher) {
    iv::open_screen();
    let canvas = iv::get_canvas();

    b.iter(|| {
        iv::fill_area(canvas.clip_rect, iv::Color32(0xffffff));
    })
}