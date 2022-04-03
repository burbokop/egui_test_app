
#![feature(test)]


extern crate test;
use egui_app::iv_platform::inkview as iv;
use iv::Draw;
use test::Bencher;


#[bench]
fn rect(b: &mut Bencher) {
    let size = iv::VecU32 { x: 1000, y: 1000 };
    let mut vec: Vec<u8> = vec![42; (size.x * size.y).try_into().unwrap()];

    let mut canvas = iv::Canvas {
        size: size,
        scanline: size.y as usize,
        depth: 8,
        clip_rect: iv::Rect { pos: iv::VecI32 { x: 0, y: 0 }, size: size },
        pixels: &mut vec[..],
    };

    b.iter(|| {
        canvas.fill_area(canvas.clip_rect, iv::Color32(0xffffff));
    })
}