
use super::{Rect, Color32, VecI32, Canvas};

pub enum Quarter {
    LeftTop,
    RightTop,
    LeftBottom,
    RightBottom,
}
pub trait Draw {
    fn fill_area(
        &mut self,
        rect: Rect, 
        fill_color: Color32,
    ) -> Option<Rect>;

    fn draw_rect(
        &mut self,
        rect: Rect, 
        border_width: u32,
        lt_radius: u32,
        rt_radius: u32,
        lb_radius: u32,
        rb_radius: u32,
        fill_color: Option<Color32>,
        border_color: Option<Color32>,
        external_color: Option<Color32>
    ) -> Option<Rect>;

    fn draw_circle_quarter(
        &mut self,
        pos: VecI32, 
        quarter: Quarter,
        border_width: u32,
        radius: u32,
        internal_color: Option<Color32>, 
        line_color: Option<Color32>, 
        external_color: Option<Color32>
    ) -> Option<Rect>;

    fn draw_circle(
        &mut self,
        center: VecI32, 
        border_width: u32,
        radius: u32,
        internal_color: Option<Color32>, 
        line_color: Option<Color32>, 
        external_color: Option<Color32>
    ) -> Option<Rect>;
}

impl<'a> Draw for Canvas<'a> {
    fn fill_area(
        &mut self,
        rect: Rect, 
        fill_color: Color32,
    ) -> Option<Rect> {
        self.foreach_mut(rect, |pix, _, _| *pix = fill_color.avr())        
    }

    fn draw_rect(
        &mut self,
        rect: Rect, 
        border_width: u32,
        lt_radius: u32,
        rt_radius: u32,
        lb_radius: u32,
        rb_radius: u32,
        fill_color: Option<Color32>,
        border_color: Option<Color32>,
        external_color: Option<Color32>
    ) -> Option<Rect> {
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
        // left 
        {
            let p0 = VecI32 { x: rect.lt().x, y: rect.lt().y + lt_radius as i32 };
            let p1 = VecI32 { x: rect.lb().x + border_width as i32, y: rect.lb().y - lb_radius as i32 };
        
            let p2 = VecI32 { x: rect.lt().x + border_width as i32, y: rect.lt().y + lt_radius as i32 };
            let p3 = VecI32 { x: rect.lb().x + lb_radius.max(lt_radius) as i32, y: rect.lb().y - lb_radius as i32 };
        
            Rect::from_points(p0, p1).map(|r| border_color.map(|c| self.fill_area(r, c)));
            Rect::from_points(p2, p3).map(|r| fill_color.map(|c| self.fill_area(r, c)));
        }
        // right 
        let rt_point = {
            let p0 = VecI32 { x: rect.rt().x - rt_radius.max(rb_radius) as i32, y: rect.rt().y + rt_radius as i32 };
            let p1 = VecI32 { x: rect.rb().x - border_width as i32, y: rect.rb().y - rb_radius as i32 };
        
            let p2 = VecI32 { x: rect.rt().x - border_width as i32, y: rect.rt().y + rt_radius as i32 };
            let p3 = VecI32 { x: rect.rb().x, y: rect.rb().y - rb_radius as i32 };
        
            Rect::from_points(p0, p1).map(|r| fill_color.map(|c| self.fill_area(r, c)));
            Rect::from_points(p2, p3).map(|r| border_color.map(|c| self.fill_area(r, c)));
            p0
        };
        // top
        {
            let p0 = VecI32 { x: rect.lt().x + lt_radius as i32, y: rect.lt().y };
            let p1 = VecI32 { x: rect.rt().x - rt_radius as i32, y: rect.rt().y + border_width as i32 };

            let p2 = VecI32 { x: rect.lt().x + lt_radius as i32, y: rect.lt().y + border_width as i32 };
            let p3 = VecI32 { x: rect.rt().x - rt_radius as i32, y: rect.rt().y + rt_radius.max(lt_radius) as i32 };

            Rect::from_points(p0, p1).map(|r| border_color.map(|c| self.fill_area(r, c)));
            Rect::from_points(p2, p3).map(|r| fill_color.map(|c| self.fill_area(r, c)));
        }
        // bottom
        let lb_point = {
            let p0 = VecI32 { x: rect.lb().x + lb_radius as i32, y: rect.lb().y - lb_radius.max(rb_radius) as i32 };
            let p1 = VecI32 { x: rect.rb().x - rb_radius as i32, y: rect.rb().y - border_width as i32 };

            let p2 = VecI32 { x: rect.lb().x + lb_radius as i32, y: rect.lb().y - border_width as i32 };
            let p3 = VecI32 { x: rect.rb().x - rb_radius as i32, y: rect.rb().y as i32 };

            Rect::from_points(p0, p1).map(|r| fill_color.map(|c| self.fill_area(r, c)));
            Rect::from_points(p2, p3).map(|r| border_color.map(|c| self.fill_area(r, c)));
            p0
        };
        fill_color.map(|c| self.fill_area(Rect::from_points_auto_flip(rt_point, lb_point), c));
        Some(rect)
    }

    fn draw_circle_quarter(
        &mut self,
        pos: VecI32,
        quarter: Quarter,
        border_width: u32,
        radius: u32,
        internal_color: Option<Color32>,
        line_color: Option<Color32>,
        external_color: Option<Color32>
    ) -> Option<Rect> {
        let center = match quarter {
            Quarter::LeftTop => VecI32 { x: pos.x + radius as i32, y: pos.y + radius as i32 },
            Quarter::RightTop => VecI32 { x: pos.x - radius as i32, y: pos.y + radius as i32 },
            Quarter::LeftBottom => VecI32 { x: pos.x + radius as i32, y: pos.y - radius as i32 },
            Quarter::RightBottom => VecI32 { x: pos.x - radius as i32, y: pos.y - radius as i32 },
        };

        let orect = Rect::from_points_auto_flip(pos, center);

        if let Some(rect) = orect.clip(self.clip_rect) {
            let internal_radous = if radius > border_width { radius - border_width } else { 0 };
            let rq0 = internal_radous * internal_radous;
            let rq1 = radius * radius;

            let y_mul = self.scanline;
            let x_mul = self.depth_bytes();

            for y in rect.pos.y as u32..(rect.pos.y as u32 + rect.size.y) {
                let yq = ((y as i32 - center.y) * (y as i32 - center.y)) as u32;
                for x in rect.pos.x as u32..(rect.pos.x as u32 + rect.size.x) {
                    let xq = ((x as i32 - center.x) * (x as i32 - center.x)) as u32;

                    if (xq + yq) < rq0 {
                        if let Some(color) = internal_color {
                            self.pixels[x as usize * x_mul + y as usize * y_mul] = color.avr();
                        }
                    } else if (xq + yq) < rq1 {
                        if let Some(color) = line_color {
                            self.pixels[x as usize * x_mul + y as usize * y_mul] = color.avr();
                        }
                    } else {
                        if let Some(color) = external_color {
                            self.pixels[x as usize * x_mul + y as usize * y_mul] = color.avr();
                        }
                    }
                }
            }
            Some(rect)
        } else { None }
    }

    fn draw_circle(
        &mut self,
        center: VecI32, 
        border_width: u32,
        radius: u32,
        internal_color: Option<Color32>, 
        line_color: Option<Color32>, 
        external_color: Option<Color32>
    ) -> Option<Rect> {
        if radius > 0 {
            let rect = Rect::from_radius(center, radius);
            self.draw_circle_quarter(rect.lt(), Quarter::LeftTop, border_width, radius, internal_color, line_color, external_color);
            self.draw_circle_quarter(rect.rt(), Quarter::RightTop, border_width, radius, internal_color, line_color, external_color);
            self.draw_circle_quarter(rect.lb(), Quarter::LeftBottom, border_width, radius, internal_color, line_color, external_color);
            self.draw_circle_quarter(rect.rb(), Quarter::RightBottom, border_width, radius, internal_color, line_color, external_color);
            Some(rect)
        } else {
            None
        }
    }
}