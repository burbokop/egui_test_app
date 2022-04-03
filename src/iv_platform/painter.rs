#![allow(unsafe_code)]


use std::convert::identity;
use std::time::Instant;


use epaint::ClippedShape;
use itertools::Itertools;

use super::inkview as iv;
use super::convert::to_iv;

pub struct Painter {
    pixels_per_point: f32,
    last_frame_clipped_shapes: Vec<ClippedShape>,
    last_frame_update_rects: Vec<iv::Rect>,
}

impl Painter {
    pub fn new(pixels_per_point: f32) -> Self {
        Self { 
            pixels_per_point: pixels_per_point,
            last_frame_clipped_shapes: Vec::default(),
            last_frame_update_rects: Vec::default(),
        }
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


    pub fn paint_shape<'f, D: iv::Draw>(draw: &mut D, shape: ClippedShape, dirty: bool, pixels_per_point: f32) -> Option<iv::Rect> {
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

        ur.and_then(|rect| if dirty { Some(rect) } else { None })
    }

    
    pub fn paint_and_update_textures<'f, D: iv::Draw>(
        &mut self,
        draw: &mut D,
        clipped_shapes: Vec<epaint::ClippedShape>,
        textures_delta: &egui::TexturesDelta,
    ) {
        for (_, image_delta) in &textures_delta.set {            
            match &image_delta.image {
                egui::ImageData::Color(_) => {},
                egui::ImageData::Alpha(_) => {},
            };
        }

        println!("PAINT");

        let painting_start = Instant::now();

        let update_type = iv::update::A2;

        let mut dirty_count: usize = 0;
        let dirty_shapes = Self::mark_shape_dirty(clipped_shapes.clone(), &mut self.last_frame_clipped_shapes, &mut dirty_count);
        if dirty_count > 0 {


            let paint_shapes_start = Instant::now();

            let update_rects: Vec<_> = dirty_shapes
                .into_iter()
                .map(|s| Self::paint_shape(draw, s.0, s.1, self.pixels_per_point))
                .filter_map(identity)
                .collect();

            println!("\tpaint shapes duration: {:?}", Instant::now() - paint_shapes_start);

            //println!("UPDATE LAST DIRTY RECTS");

            let updating_prev_start = Instant::now();


            let actualy_updated_count = update_rects
                .clone()
                .into_iter()
                .chain(self.last_frame_update_rects.clone().into_iter())
                .unique()
                .map(|rect|{
                iv::dynamic_update(update_type.into(), rect);
            }).count();

            println!(
                "\tupdating screen duration: {:?} (count: {} merge {} = {})", 
                Instant::now() - updating_prev_start, 
                self.last_frame_update_rects.len(),
                update_rects.len(),
                actualy_updated_count
            );

            self.last_frame_update_rects = update_rects;

            self.last_frame_clipped_shapes = clipped_shapes;

        }
        println!("\tpainting duration: {:?}", Instant::now() - painting_start)
    }
}
