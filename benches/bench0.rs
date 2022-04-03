
#![feature(test)]


extern crate test;
use bench::Canvas;
use rand::Rng;
use test::Bencher;
extern crate rand;

use bench::*;

#[bench]
fn rect(b: &mut Bencher) {
    let size = VecU32 { x: 1000, y: 1000 };
    let mut vec: Vec<u8> = vec![42; (size.x * size.y).try_into().unwrap()];

    let mut canvas = Canvas {
        size: size,
        scanline: size.y as usize,
        depth: 8,
        clip_rect: Rect { pos: VecI32 { x: 0, y: 0 }, size: size },
        pixels: &mut vec[..],
    };


    b.iter(|| {
        canvas.fill_area(canvas.clip_rect, Color32(0xffffff));
    })
}

#[bench]
fn setup_random_hashmap(b: &mut Bencher) {
    let mut val : u32 = 0;
    let mut rng = rand::thread_rng();
    let mut map = std::collections::HashMap::new();

    b.iter(|| { map.insert(rng.gen::<u8>() as usize, val); val += 1; })
}
