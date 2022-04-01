use std::ops::Div;





#[derive(Debug, Clone, Copy)]
pub struct NonZeroF32(f32);


impl NonZeroF32 {
    pub fn from_f32(f: f32) -> Option<Self> {
        if f < f32::EPSILON {
            None
        } else {
            Some(NonZeroF32(f))
        }
    }
    pub fn f32(self) -> f32 { self.0 }
}

impl Div<NonZeroF32> for f32 {
    type Output = f32;
    fn div(self, rhs: NonZeroF32) -> Self::Output { self / rhs.0 }
}

pub struct NonZeroF64(f64);

impl NonZeroF64 {
    pub fn from_f64(f: f64) -> Option<Self> {
        if f < f64::EPSILON {
            None
        } else {
            Some(NonZeroF64(f))
        }
    }
    pub fn f64(self) -> f64 { self.0 }
}

impl Div<NonZeroF64> for f64 {
    type Output = f64;
    fn div(self, rhs: NonZeroF64) -> Self::Output { self / rhs.0 }
}
