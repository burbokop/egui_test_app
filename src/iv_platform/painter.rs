#![allow(unsafe_code)]


use egui::{
    emath::Rect,
    epaint::{Color32, Mesh, Vertex},
};
use epaint::ClippedShape;

use super::{inkview as iv, epi_backend::pixels_per_point32};
use super::convert::{from_iv, to_iv};

pub struct Painter {
    max_texture_side: usize,
    srgb_support: bool,
    pixels_per_point: f32,
    destroyed: bool,
}

impl Painter {
    pub fn new(pixels_per_point: f32) -> Self {
        Self { 
            destroyed: false, 
            max_texture_side: 1000, 
            srgb_support: true, 
            pixels_per_point: pixels_per_point 
        }
    }

    pub fn max_texture_side(&self) -> usize {
        self.max_texture_side
    }

    pub fn paint_shape<'f, D: iv::Draw>(&mut self, draw: &mut D, shape: ClippedShape, font: &iv::Font<'f>) -> Option<iv::Rect> {
        match shape.1 {
            egui::Shape::Noop => todo!(),
            egui::Shape::Vec(_) => todo!(),
            egui::Shape::Circle(circle) => {
                println!("circle.fill: {:?}", circle.fill);

                Some(iv::draw_circle_quarter(
                    to_iv::emath_pos(circle.center, self.pixels_per_point), 
                    circle.radius as u32, 
                    iv::Style::from_bits_truncate(iv::Style::default().bits() | iv::Style::FILL_INSIDE.bits()), 
                    circle.stroke.width as u32, 
                    to_iv::epaint_color(circle.stroke.color), 
                    iv::Color32::BLACK //to_iv::epaint_color(circle.fill)
                ))
            },
            egui::Shape::LineSegment { points, stroke } => {
                Some(iv::draw_line(
                    to_iv::emath_pos(points[0], self.pixels_per_point), 
                    to_iv::emath_pos(points[1], self.pixels_per_point), 
                    to_iv::epaint_color(stroke.color)
                ))
            },
            egui::Shape::Path(path) => todo!(),
            egui::Shape::Rect(rect) => {

                Some(draw.draw_rect(
                    to_iv::emath_rect(rect.rect, self.pixels_per_point), 
                    8,//rect.stroke.width as u32,
                    5, //rect.rounding.nw as u32,
                    12, //rect.rounding.ne as u32,
                    15, //rect.rounding.sw as u32,
                    20, //rect.rounding.se as u32,
                    iv::Color32(0xff222222),// to_iv::epaint_color(rect.fill),
                    iv::Color32(0xff888888),// to_iv::epaint_color(rect.stroke.color), 
                    iv::Color32(0xffaaaaaa),
                ))
/*
                Some(iv::draw_frame_certified_ex(
                    to_iv::emath_rect(rect.rect, self.pixels_per_point), 
                    8, // rect.stroke.width as i32, 
                    iv::Side::default(),
                    iv::Style::from_bits_truncate(iv::Style::default().bits() | iv::Style::FILL_INSIDE.bits()), 
                    8, 
                    to_iv::epaint_color(rect.stroke.color), 
                    to_iv::epaint_color(rect.fill)
                ))
                 */
            }
            egui::Shape::Text(text) => {
                let galley = text.galley.as_ref();
                let job = galley.job.as_ref();

                let translated_rect = galley.rect.translate(text.pos.to_vec2());
               
                if job.sections.len() > 0 {
                    let f =  &job.sections[0].format.font_id;

                    //println!("f: {:?}", font);

                    //println!("f.family: {}", f.family);

                    iv::set_font(font, to_iv::epaint_color(text.override_text_color.unwrap_or(Color32::from_rgb(255, 255, 255))));
                
                    Some(iv::draw_text_rect(to_iv::emath_rect(translated_rect, self.pixels_per_point), job.text.as_str(), 0).0)
                } else {
                    None
                }
               
                //iv::draw_string(Self::emath_pos_to_iv_vec(text.pos), job.text.as_str());
            },
            egui::Shape::Mesh(_) => todo!(),
            egui::Shape::QuadraticBezier(_) => todo!(),
            egui::Shape::CubicBezier(_) => todo!(),
        }
    }

    
    pub fn paint_and_update_textures<'f, D: iv::Draw>(
        &mut self,
        draw: &mut D,
        clipped_shapes: Vec<epaint::ClippedShape>,
        textures_delta: &egui::TexturesDelta,
        font: &iv::Font<'f>
    ) {

        for (id, image_delta) in &textures_delta.set {
            let p = &image_delta.pos;

            let s = match &image_delta.image {
                egui::ImageData::Color(image) => {

                    //depth = 32
                    println!("\tcolor image: {:?}", image.size);
                    [image.width(), image.height()]
                },
                egui::ImageData::Alpha(image) => {
                    println!("\talpha image: {:?}", image.size);
                    [image.width(), image.height()]

/*
                    let unwraped_pos = image_delta.pos.unwrap_or([0, 0]);

                    let end_pos = [
                        canvas.width.min(image.width() + unwraped_pos[0]),
                        canvas.height.min(image.height() + unwraped_pos[1]),
                    ];

                    println!("before {}, {}, {}, {}",unwraped_pos[0], unwraped_pos[1], end_pos[0] - unwraped_pos[0], end_pos[1] - unwraped_pos[1]);
                    for y in unwraped_pos[1]..end_pos[1] {
                        for x in unwraped_pos[0]..end_pos[0] {
                        let uu = image.pixels[x + y * image.width()];
                            canvas.pixels[x + y * canvas.width] = uu;

                        }
                    }
                    println!("after {}, {}, {}, {}",unwraped_pos[0], unwraped_pos[1], end_pos[0] - unwraped_pos[0], end_pos[1] - unwraped_pos[1]);

                    inkview::dynamic_update(
                        inkview::DynamicUpdateType::Normal(inkview::update_type::Normal), 
                        unwraped_pos[0], 
                        unwraped_pos[1], 
                        end_pos[0] - unwraped_pos[0], 
                        end_pos[1] - unwraped_pos[1]
                    );

                    [image.width(), image.height()]
                     */
                },
            };

            
            

            println!("\timage_delta: {:?}, {:?}", p, s)
        }

        for s in clipped_shapes {
            //println!("\tshape: {:?}");
            if let Some(update_rect) = self.paint_shape(draw, s, font) {
                iv::dynamic_update(iv::DynamicUpdateType::A2(iv::update_type::A2), update_rect)
            }
        }
        
        //self.paint_meshes(gl, inner_size, pixels_per_point, clipped_meshes);

        //for &id in &textures_delta.free {
        //    self.free_texture(gl, id);
        //}
    }
}

impl Drop for Painter {
    fn drop(&mut self) {
        if !self.destroyed {
            panic!(
                "You forgot to call destroy() on the egui glow painter. Resources will leak!"
            );
        }
    }
}

#[cfg(feature = "epi")]
impl epi::NativeTexture for Painter {
    type Texture = glow::Texture;

    fn register_native_texture(&mut self, native: Self::Texture) -> egui::TextureId {
        self.assert_not_destroyed();
        let id = egui::TextureId::User(self.next_native_tex_id);
        self.next_native_tex_id += 1;
        self.textures.insert(id, native);
        id
    }

    fn replace_native_texture(&mut self, id: egui::TextureId, replacing: Self::Texture) {
        if let Some(old_tex) = self.textures.insert(id, replacing) {
            self.textures_to_destroy.push(old_tex);
        }
    }
}