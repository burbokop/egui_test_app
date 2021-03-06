

pub mod from_iv {
    use super::super::inkview as iv;

    #[inline]
    pub fn emath_pos(vec: iv::VecI32, pixels_per_point: iv::NonZeroF32) -> emath::Pos2 {
        emath::Pos2::new(vec.x as f32 / pixels_per_point, vec.y as f32 / pixels_per_point)
    }
    
    #[inline]
    pub fn emath_rect(rect: iv::Rect, pixels_per_point: iv::NonZeroF32) -> emath::Rect {
        emath::Rect { 
            min: emath_pos(rect.pos, pixels_per_point), 
            max: emath_pos(iv::VecI32{ x: rect.pos.x + rect.size.x as i32, y: rect.pos.y + rect.size.y as i32 }, pixels_per_point) 
        }
    }
    
    #[inline]
    pub fn epaint_color(color: iv::Color32) -> epaint::Color32 {
        epaint::Color32::from_rgb(color.r(), color.g(), color.b())
    }
}

pub mod to_iv {
    use super::super::inkview as iv;

    #[inline]
    pub fn emath_pos(pos: emath::Pos2, pixels_per_point: f32) -> iv::VecI32 {
        iv::VecI32 { x: (pos.x * pixels_per_point) as i32, y: (pos.y * pixels_per_point) as i32 }
    }
    
    #[inline]
    pub fn emath_rect(rect: emath::Rect, pixels_per_point: f32) -> iv::Rect {
        let min = emath_pos(rect.min, pixels_per_point);
        let max = emath_pos(rect.max, pixels_per_point);
        iv::Rect { 
            pos: min, 
            size: iv::VecU32 { x: (max.x - min.x) as u32, y: (max.y - min.y) as u32 } 
        }
    }

    #[inline]
    pub fn epaint_color(color: epaint::Color32) -> iv::Color32 {
        iv::Color32::argb(color.a(), color.r(), color.g(), color.b())
    }

}