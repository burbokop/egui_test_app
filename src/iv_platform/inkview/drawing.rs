use epaint::Pos2;

use super::{Rect, Color32, VecI32, Canvas};



pub enum Quarter {
    LeftTop,
    RightTop,
    LeftBottom,
    RightBottom,
}
pub trait Draw {
    fn draw_rect(
        &mut self,
        rect: Rect, 
        border_width: u32,
        lt_radius: u32,
        rt_radius: u32,
        lb_radius: u32,
        rb_radius: u32,
        fill_color: Color32,
        border_color: Color32,
        external_color: Color32
    ) -> Rect;

    fn draw_circle_quarter(
        &mut self,
        pos: VecI32, 
        quarter: Quarter,
        border_width: u32,
        radius: u32,
        internal_color: Color32, 
        line_color: Color32, 
        external_color: Color32
    ) -> Option<Rect>;
}

impl<'a> Draw for Canvas<'a> {
    fn draw_rect(
        &mut self,
        rect: Rect, 
        border_width: u32,
        lt_radius: u32,
        rt_radius: u32,
        lb_radius: u32,
        rb_radius: u32,
        fill_color: Color32,
        border_color: Color32,
        external_color: Color32
    ) -> Rect {
        if lt_radius > 0 {
            self.draw_circle_quarter(rect.lt(), Quarter::LeftTop, border_width, lt_radius, fill_color, border_color, external_color);
        }
        if rt_radius > 0 {
            self.draw_circle_quarter(rect.rt(), Quarter::RightTop, border_width, rt_radius, fill_color, border_color, external_color);
        }
        if lb_radius > 0 {
            self.draw_circle_quarter(rect.lb(), Quarter::LeftBottom, border_width, lb_radius, fill_color, border_color, external_color);
        }
        if rb_radius > 0 {
            self.draw_circle_quarter(rect.rb(), Quarter::RightBottom, border_width, rb_radius, fill_color, border_color, external_color);
        }
        rect
    }

    fn draw_circle_quarter(
        &mut self,
        pos: VecI32,
        quarter: Quarter,
        border_width: u32,
        radius: u32,
        internal_color: Color32,
        line_color: Color32,
        external_color: Color32
    ) -> Option<Rect> {
        let center = match quarter {
            Quarter::LeftTop => VecI32 { x: pos.x + radius as i32, y: pos.y + radius as i32 },
            Quarter::RightTop => VecI32 { x: pos.x - radius as i32, y: pos.y + radius as i32 },
            Quarter::LeftBottom => VecI32 { x: pos.x + radius as i32, y: pos.y - radius as i32 },
            Quarter::RightBottom => VecI32 { x: pos.x - radius as i32, y: pos.y - radius as i32 },
        };

        let orect = Rect::from_points_auto_flip(pos, center);

        if let Some(rect) = orect.clip(self.clip_rect) {
            println!("self.clip_rect: {:?}, orig.rect: {:?} -> {:?}", self.clip_rect, orect, rect);
        
            let internal_radous = if radius > border_width { radius - border_width } else { 0 };
            let rq0 = internal_radous * internal_radous;
            let rq1 = radius * radius;

            let y_mul = self.scanline;
            let x_mul = self.x_mul();

        

        //self.clip_rect: Rect { pos: VecI32 { x: 0, y: 0 }, size: VecU32 { x: 1071, y: 1447 } }, 
        //     orig.rect: Rect { pos: VecI32 { x: 0, y: 2005 }, size: VecU32 { x: 5, y: 5 } }
        //        result: Rect { pos: VecI32 { x: 0, y: 1447 }, size: VecU32 { x: 5, y: 558 } }


            for y in rect.pos.y as u32..(rect.pos.y as u32 + rect.size.y) {
                let yq = ((y as i32 - center.y) * (y as i32 - center.y)) as u32;
                for x in rect.pos.x as u32..(rect.pos.x as u32 + rect.size.x) {
                    let xq = ((x as i32 - center.x) * (x as i32 - center.x)) as u32;

                    println!("x: {}, y: {}, : {} : {} | {} | {}", x, y, yq, xq, rq0, rq1);

                    if (xq + yq) < rq0 {
                        println!("\tint");
                        if !internal_color.is_transperent() {
                            self.pixels[x as usize * x_mul + y as usize * y_mul] = internal_color.avr();
                        }
                    } else if (xq + yq) < rq1 {
                        println!("\tmid");
                        if !line_color.is_transperent() {
                            self.pixels[x as usize * x_mul + y as usize * y_mul] = line_color.avr();
                        }
                    } else {
                        println!("\text");
                        if !external_color.is_transperent() {
                            self.pixels[x as usize * x_mul + y as usize * y_mul] = external_color.avr();
                        }
                    }
                }
            }
            Some(rect)
        } else { None }
    }
}