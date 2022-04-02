#![allow(unsafe_code)]


use std::convert::identity;
use std::time::Instant;

use egui::{
    epaint::Color32,
};
use epaint::ClippedShape;

use super::{inkview as iv};
use super::convert::{to_iv};

pub struct PainterOptions {
    paint_only_changed: bool,
    update_pref_frame_rects: bool
}

pub struct Painter {
    max_texture_side: usize,
    pixels_per_point: f32,
    destroyed: bool,
    last_frame_clipped_shapes: Vec<ClippedShape>,
    last_frame_update_rects: Vec<iv::Rect>,

}

impl Painter {
    pub fn new(pixels_per_point: f32) -> Self {
        Self { 
            destroyed: false, 
            max_texture_side: 1000, 
            pixels_per_point: pixels_per_point,
            last_frame_clipped_shapes: Vec::default(),
            last_frame_update_rects: Vec::default(),
        }
    }

    pub fn max_texture_side(&self) -> usize {
        self.max_texture_side
    }

    pub fn mark_shape_dirty(shapes: Vec<ClippedShape>, exclude: &mut Vec<ClippedShape>, ditry_count: &mut usize) -> Vec<(ClippedShape, bool)> {
        shapes.into_iter().map(|s| {
            if let Some(p) = exclude.iter().position(|e| { let a = e.eq(&s); a }) {
                exclude.remove(p);
                (s, false)
            } else {
                *ditry_count += 1;
                (s, true)
            }
        }).collect()
    }


    pub fn paint_shape<'f, D: iv::Draw>(draw: &mut D, shape: ClippedShape, dirty: bool, pixels_per_point: f32, font: &iv::Font<'f>) -> Option<iv::Rect> {
        let ur = match &shape.1 {
            egui::Shape::Noop => todo!(),
            egui::Shape::Vec(_) => todo!(),
            egui::Shape::Circle(circle) => {
                draw.draw_circle(
                    to_iv::emath_pos(circle.center, pixels_per_point), 
                    (circle.stroke.width * pixels_per_point) as u32,
                    (circle.radius * pixels_per_point) as u32,
                    Some(to_iv::epaint_color(circle.fill)),
                    Some(to_iv::epaint_color(circle.stroke.color)),
                    None
                )
            },
            egui::Shape::LineSegment { points, stroke } => {
                Some(iv::draw_line(
                    to_iv::emath_pos(points[0], pixels_per_point), 
                    to_iv::emath_pos(points[1], pixels_per_point), 
                    to_iv::epaint_color(stroke.color)
                ))
            },
            egui::Shape::Path(_) => todo!(),
            egui::Shape::Rect(rect) => {
                draw.draw_rect(
                    to_iv::emath_rect(rect.rect, pixels_per_point), 
                    rect.stroke.width as u32,
                    (rect.rounding.nw * pixels_per_point) as u32,
                    (rect.rounding.ne * pixels_per_point) as u32,
                    (rect.rounding.sw * pixels_per_point) as u32,
                    (rect.rounding.se * pixels_per_point) as u32,
                    Some(to_iv::epaint_color(rect.fill)),
                    Some(to_iv::epaint_color(rect.stroke.color)), 
                    None,
                )
            }
            egui::Shape::Text(text) => {
                let galley = text.galley.as_ref();
                let job = galley.job.as_ref();
                let translated_rect = galley.rect.translate(text.pos.to_vec2());
              
                //println!("&job.sections: {:?}", &job.sections);
                if job.sections.len() > 0 {
                    let fid =  &job.sections[0].format.font_id;
                    let color =  &job.sections[0].format.color;


                    //LayoutSection { 
                    //    leading_space: 0.0, 
                    //    byte_range: 0..19, 
                    //    format: TextFormat { 
                    //    font_id: FontId { 
                    //        size: 20.0, 
                    //        family: Proportional 
                    //    }, 
                    //    color: Color32([64, 254, 0, 255]), 
                    //    background: Color32([0, 0, 0, 0]), 
                    //    italics: false, 
                    //    underline: Stroke { 
                    //        width: 0.0, 
                    //        color: Color32([0, 0, 0, 0]) }, 
                    //        strikethrough: Stroke { 
                    //            width: 0.0, 
                    //            color: Color32([0, 0, 0, 0]) 
                    //        }, 
                    //        valign: Center 
                    //    } 
                    //}

                    //println!("f: {:?}, default: {}", fid, iv::get_default_font(iv::FontType::Std));

                    //println!("f.family: {}", f.family);

                    let new_font = iv::open_font(iv::get_default_font(iv::FontType::Std), (fid.size * pixels_per_point) as usize, 1);

                    iv::set_font(&new_font, to_iv::epaint_color(text.override_text_color.unwrap_or(*color)));
                
                    let translated_rect = to_iv::emath_rect(translated_rect, pixels_per_point);

                    //draw.draw_rect(translated_rect, 4, 0, 0, 0, 0, None, Some(iv::Color32(0xff888888)), None);

                    Some(iv::draw_text_rect(translated_rect, job.text.as_str(), 0).0)
                } else {
                    None
                }
               
                //iv::draw_string(Self::emath_pos_to_iv_vec(text.pos), job.text.as_str());
            },
            egui::Shape::Mesh(_) => todo!(),
            egui::Shape::QuadraticBezier(_) => todo!(),
            egui::Shape::CubicBezier(_) => todo!(),
        };

        //match shape.1 {
        //    epaint::Shape::Circle(s) => println!("\tdirty: {:?}, shape.Circle: {:?}", dirty, ur),
        //    epaint::Shape::Rect(s) =>  println!("\tdirty: {:?}, shape.Rect: {:?}", dirty, ur),
        //    _ => {}
        //}

        ur.and_then(|rect| if dirty { Some(rect) } else { None })



    }

    
    pub fn paint_and_update_textures<'f, D: iv::Draw>(
        &mut self,
        draw: &mut D,
        clipped_shapes: Vec<epaint::ClippedShape>,
        textures_delta: &egui::TexturesDelta,
        font: &iv::Font<'f>
    ) {

        for (_, image_delta) in &textures_delta.set {
            let p = &image_delta.pos;

            let s = match &image_delta.image {
                egui::ImageData::Color(image) => {

                    //depth = 32
                    //println!("\tcolor image: {:?}", image.size);
                    [image.width(), image.height()]
                },
                egui::ImageData::Alpha(image) => {
                    //println!("\talpha image: {:?}", image.size);
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

            
            

        }

        println!("PAINT");

        let painting_start = Instant::now();

        let update_type = iv::update::A2;

        let mut dirty_count: usize = 0;
        let dirty_shapes = Self::mark_shape_dirty(clipped_shapes.clone(), &mut self.last_frame_clipped_shapes, &mut dirty_count);
        if dirty_count > 0 {


            let paint_shapes_start = Instant::now();

            let update_rects: Vec<_> = dirty_shapes.into_iter()
                .map(|s| Self::paint_shape(draw, s.0, s.1, self.pixels_per_point, font))
                .filter_map(identity).collect();

            println!("\tpaint shapes duration: {:?}", Instant::now() - paint_shapes_start);

            //println!("UPDATE LAST DIRTY RECTS");

            for rect in &self.last_frame_update_rects {
                //println!("\tlfur: {:?}", rect);
                iv::dynamic_update(update_type.into(), *rect);
            }

            //println!("UPDATE CURRENT DIRTY RECTS");

            self.last_frame_update_rects = update_rects.into_iter().map(|rect| {
                iv::dynamic_update(update_type.into(), rect);
                rect
            }).collect();

            self.last_frame_clipped_shapes = clipped_shapes;

        }

        println!("\tpainting duration: {:?}", Instant::now() - painting_start)

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