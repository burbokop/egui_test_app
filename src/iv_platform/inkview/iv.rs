use std::{ffi::{CStr, c_void}, os::raw::{c_int, c_char, c_uint}, mem, hash::Hash};
use std::fmt::Debug;
use std::cmp::Eq;

use super::Event;
#[allow(unsafe_code)]

use super::c_iv;

pub fn current_task() -> i32 {
    unsafe { c_iv::GetCurrentTask() }
}

pub fn open_screen() {
    unsafe { c_iv::OpenScreen() }
}

pub fn is_screen_opened() -> bool {
    unsafe { c_iv::IsScreenOpened() > 0 }
}

pub fn device_model() -> &'static str {
    unsafe { CStr::from_ptr(c_iv::GetDeviceModel()) }.to_str().unwrap() 
}

pub fn hardware_type() -> &'static str {
    unsafe { CStr::from_ptr(c_iv::GetHardwareType()) }.to_str().unwrap() 
}

pub fn software_version() -> &'static str {
    unsafe { CStr::from_ptr(c_iv::GetSoftwareVersion()) }.to_str().unwrap() 
}



extern "C" fn event_handler(evt: c_int, arg0: c_int, arg1: c_int) -> c_int {
    if let Some(f) = unsafe { &mut EVENT_HANDLER_CONTEXT } {
        println!("event: evt: {}, arg0: {}, arg1: {}", evt, arg0, arg1);
        f(Event::from_c_iv_event(evt, arg0, arg1).unwrap()).into()
    } else {
        -1
    }
}

static mut EVENT_HANDLER_CONTEXT: Option<Box<dyn FnMut(Event) -> bool>> = None;

pub fn prepare_for_loop<F: 'static + FnMut(Event) -> bool>(f: F) {
    unsafe { 
        if EVENT_HANDLER_CONTEXT.is_none() {
            let context: Box<dyn FnMut(Event) -> bool> = Box::new(f);
            EVENT_HANDLER_CONTEXT = Some(context);
            c_iv::PrepareForLoop(Some(event_handler)) 
        }
    }
}


extern "C" fn prepare_for_loop_ex_handler(ctx: *mut c_void, evt: c_int, arg0: c_int, arg1: c_int) -> c_int {
    let closure: &mut Box<dyn FnMut(Event) -> bool> = unsafe { mem::transmute(ctx) };
    //println!("event_ex: evt: {}, arg0: {}, arg1: {}", evt, arg0, arg1);
    closure(Event::from_c_iv_event(evt, arg0, arg1).unwrap()) as c_int
}

pub fn prepare_for_loop_ex<F: FnMut(Event) -> bool /* true if accepted */>(f: F) {
    let cb: Box<Box<dyn FnMut(Event) -> bool>> = Box::new(Box::new(f));
    unsafe { c_iv::PrepareForLoopEx(Box::into_raw(cb) as *mut _, Some(prepare_for_loop_ex_handler)) }
}

pub fn process_event_loop() { unsafe { c_iv::ProcessEventLoop() } }
pub fn clear_on_exit() { unsafe { EVENT_HANDLER_CONTEXT = None; c_iv::ClearOnExit() } }

pub fn screen_width() -> Result<u32, <i32 as TryInto<u32>>::Error> { unsafe { c_iv::ScreenWidth() }.try_into() }
pub fn screen_height() -> Result<u32, <i32 as TryInto<u32>>::Error> { unsafe { c_iv::ScreenHeight() }.try_into() }

extern "C" fn set_hard_timer_handler(ctx: *mut c_void) {
    let closure: &mut &mut dyn FnMut() = unsafe { mem::transmute(ctx) };
    closure()
}

pub fn set_hard_timer<F: FnMut()>(name: &str, f: F, ms: u32) {
    println!("OOOOOOOOOOOOOOOOOOO");

    let mut f_mut = f;
    let mut cb: &mut dyn FnMut() = &mut f_mut;
    let cb = &mut cb;

    unsafe { c_iv::SetHardTimerEx(name.as_ptr() as *const c_char, Some(set_hard_timer_handler), cb as *mut _ as *mut c_void, ms as i32) }
}

#[derive(Debug, Clone, Copy)]
pub struct VecI32 {
    pub x: i32,
    pub y: i32
}


impl Eq for VecI32 {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialEq for VecI32 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for VecI32 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl VecI32 {
    pub fn min(self, other: VecI32) -> VecI32 {
        VecI32 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }
    pub fn max(self, other: VecI32) -> VecI32 {
        VecI32 {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
    pub fn to_u32(self) -> Option<VecU32> {
        if self.x >= 0 && self.y >= 0 {
            Some(VecU32 { x: self.x as u32, y: self.y as u32 })
        } else {
            None
        }
    }
    pub fn add_scalar(self, s: i32) -> Self {
        Self { x: self.x + s, y: self.y + s }
    }
    pub fn sub_scalar(self, s: i32) -> Self {
        Self { x: self.x - s, y: self.y - s }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VecU32 {
    pub x: u32,
    pub y: u32
}


impl Eq for VecU32 {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialEq for VecU32 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for VecU32 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl VecU32 {
    pub fn is_zero(&self) -> bool { self.x == 0 && self.y == 0 }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub pos: VecI32,
    pub size: VecU32,
}

impl Eq for Rect {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.size == other.size
    }
}

impl Hash for Rect {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.size.hash(state);
    }
}

impl Rect {
    pub fn from_points_auto_flip(pos0: VecI32, pos1: VecI32) -> Self {
        let size = VecU32 {
            x: (pos0.x - pos1.x).abs() as u32,
            y: (pos0.y - pos1.y).abs() as u32
        };
        let pos = VecI32 {
            x: pos0.x.min(pos1.x),
            y: pos0.y.min(pos1.y),
        };
        Rect { pos: pos, size: size }
    }

    pub fn from_radius(center: VecI32, radius: u32) -> Rect {
        Rect {
            pos: VecI32 { x: center.x - radius as i32, y: center.y - radius as i32 },
            size: VecU32 { x: radius * 2, y: radius * 2 }
        }
    }

    pub fn from_points(pos0: VecI32, pos1: VecI32) -> Option<Self> {
        VecI32 {
            x: pos1.x - pos0.x,
            y: pos1.y - pos0.y
        }.to_u32().map(|size| Rect { pos: pos0, size: size })
    }


    pub fn lt(self) -> VecI32 { self.pos }
    pub fn rt(self) -> VecI32 { VecI32 { x: self.pos.x + self.size.x as i32, y: self.pos.y } }
    pub fn lb(self) -> VecI32 { VecI32 { x: self.pos.x, y: self.pos.y + self.size.y as i32 } }
    pub fn rb(self) -> VecI32 { VecI32 { x: self.pos.x + self.size.x as i32, y: self.pos.y + self.size.y as i32 } }

    pub fn clip(&self, clip_rect: Rect) -> Option<Rect> {
        Rect::from_points(
            self.pos.max(clip_rect.pos), 
            self.rb().min(clip_rect.rb())
        )
    }
}

#[repr(C)]
pub struct Canvas<'a> {
    pub size: VecU32,
    pub scanline: usize,
    pub depth: usize,
    pub clip_rect: Rect,
    pub pixels: &'a mut [u8],
}

impl<'a> Canvas<'a> {
    pub fn depth_bytes(&self) -> usize {
        self.depth / 8
    }

    pub fn foreach_mut<F: FnMut(&mut u8, u32, u32)>(&mut self, rect: Rect, mut f: F) -> Option<Rect> {
        if let Some(rect) = rect.clip(self.clip_rect) {
            let y_mul = self.scanline;
            let x_mul = self.depth_bytes();
            for y in rect.pos.y as u32..(rect.pos.y as u32 + rect.size.y) {
                for x in rect.pos.x as u32..(rect.pos.x as u32 + rect.size.x) {
                     f(&mut self.pixels[x as usize * x_mul + y as usize * y_mul], x, y);
                }
            }
            Some(rect)
        } else {
            None
        }
    }

    pub fn foreach_line_mut<F: FnMut(&mut [u8], u32)>(&mut self, rect: Rect, mut f: F) -> Option<Rect> {
        if let Some(rect) = rect.clip(self.clip_rect) {
            let y_mul = self.scanline;
            let start_x = rect.pos.x as usize * self.depth_bytes();
            let end_x = (rect.pos.x as usize + rect.size.x as usize) * self.depth_bytes();
            for y in rect.pos.y as u32..(rect.pos.y as u32 + rect.size.y) {
                let m = y as usize * y_mul;
                f(&mut self.pixels[m + start_x..m + end_x], y);
            }
            Some(rect)
        } else {
            None
        }
    }
}


impl<'a> Debug for Canvas<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Canvas")
            .field("size", &self.size)
            .field("scanline", &self.scanline)
            .field("depth", &self.depth)
            .field("clip_rect", &self.clip_rect)
            .field("pixels", &String::from(format!("[row data (len: {})]", &self.pixels.len())))
            .finish()
    }
}

use bitflags::bitflags;

bitflags! {
    pub struct Style: u16 {
        const ROUND_NONE = 0;
        const ROUND_TOP_LEFT = 1;
        const ROUND_TOP_RIGHT = 2;
        const ROUND_BOTTOM_LEFT = 4;
        const ROUND_BOTTOM_RIGHT = 8;
        const ROUND_TOP = Self::ROUND_TOP_LEFT.bits | Self::ROUND_TOP_RIGHT.bits; // 3
        const ROUND_DOWN = Self::ROUND_BOTTOM_LEFT.bits | Self::ROUND_BOTTOM_RIGHT.bits; // 12
        const ROUND_LEFT = Self::ROUND_TOP_LEFT.bits | Self::ROUND_BOTTOM_LEFT.bits; // 5
        const ROUND_RIGHT = Self::ROUND_TOP_RIGHT.bits | Self::ROUND_BOTTOM_RIGHT.bits; // 10
        const ROUND_ALL_SIDES = Self::ROUND_TOP.bits | Self::ROUND_DOWN.bits; // 15
        // fill
        const FILL_INSIDE = 16;
        const FILL_OUTSIDE_BG = 32;
        // blend
        const BLEND_SRC_INSIDE = 64; // blend with source
        const BLEND_SRC_OUTSIDE = 128; // blend with source
        const DRAW_CIRCLE_BLACK_WHITE = 256; // draw all circle pixels black or white
    }
}

impl Default for Style {
    fn default() -> Self { Self::ROUND_ALL_SIDES }
}

bitflags! {
    pub struct Side: u8 {
        const NONE = 0;
        const LEFT = 1;
        const RIGHT = 2;
        const TOP = 4;
        const BOTTOM = 8;
    }
}

#[derive(Debug)]
pub enum Error {
    ScreenNotOpened
}

impl Default for Side {
    fn default() -> Self { Self::all() }
}

pub fn get_screen_dpi() -> Result<usize, Error> {
    if is_screen_opened() {
        unsafe { Ok(c_iv::get_screen_dpi() as usize) }
    } else {
        Err(Error::ScreenNotOpened)
    }
}

pub fn get_screen_scale_factor() -> Result<f64, Error> {
    if is_screen_opened() {
        unsafe { Ok(c_iv::get_screen_scale_factor()) }
    } else {
        Err(Error::ScreenNotOpened)
    }
}

pub fn get_canvas() -> Canvas<'static> {
    let cvs = unsafe { &mut*c_iv::GetCanvas() };
    Canvas { 
        size: VecU32 {
            x: cvs.width as u32, 
            y: cvs.height as u32,     
        },
        scanline: cvs.scanline as usize,
        depth: cvs.depth as usize, 
        clip_rect: Rect { 
            pos: VecI32 { 
                x: cvs.clipx1, 
                y: cvs.clipy1 
            }, 
            size: VecU32 { 
                x: (cvs.clipx2 - cvs.clipx1) as u32, 
                y: (cvs.clipy2 - cvs.clipy1) as u32 
            } 
        },
        pixels: unsafe { 
            core::slice::from_raw_parts_mut(cvs.addr, (cvs.width * cvs.height * cvs.depth).try_into().unwrap()) 
        }
    }
}

pub mod update {
    #[derive(Debug, Clone, Copy)] pub struct Normal;
    #[derive(Debug, Clone, Copy)] pub struct HQ;
    #[derive(Debug, Clone, Copy)] pub struct Black;
    #[derive(Debug, Clone, Copy)] pub struct BW;
    #[derive(Debug, Clone, Copy)] pub struct DU4;
    #[derive(Debug, Clone, Copy)] pub struct A2;
}

pub enum FullSoftUpdateType {
    Normal(update::Normal),
    HQ(update::HQ)
}

pub fn full_update(tp: &FullSoftUpdateType) {
    match tp {
        FullSoftUpdateType::Normal(_) => unsafe { c_iv::FullUpdate() },
        FullSoftUpdateType::HQ(_) => unsafe { c_iv::FullUpdateHQ() },
    }
}

pub fn soft_update(tp: &FullSoftUpdateType) {
    match tp {
        FullSoftUpdateType::Normal(_) => unsafe { c_iv::SoftUpdate() },
        FullSoftUpdateType::HQ(_) => unsafe { c_iv::SoftUpdateHQ() },
    }
}

pub enum PartialUpdateType {
    Normal(update::Normal),
    HQ(update::HQ),
    Black(update::Black),
    BW(update::BW),
    DU4(update::DU4)
}

pub fn partial_update(tp: &PartialUpdateType, x: usize, y: usize, w: usize, h: usize) {
    match tp {
        PartialUpdateType::Normal(_) => unsafe { c_iv::PartialUpdate(x as c_int, y as c_int, w as c_int, h as c_int) },
        PartialUpdateType::HQ(_) => unsafe { c_iv::PartialUpdateHQ(x as c_int, y as c_int, w as c_int, h as c_int) },
        PartialUpdateType::Black(_) => unsafe { c_iv::PartialUpdateBlack(x as c_int, y as c_int, w as c_int, h as c_int) },
        PartialUpdateType::BW(_) => unsafe { c_iv::PartialUpdateBW(x as c_int, y as c_int, w as c_int, h as c_int) },
        PartialUpdateType::DU4(_) => unsafe { c_iv::PartialUpdateDU4(x as c_int, y as c_int, w as c_int, h as c_int) },
    }
}

pub enum DynamicUpdateType {
    Normal(update::Normal),
    BW(update::BW),
    A2(update::A2)
}

impl From<update::Normal> for DynamicUpdateType {
    fn from(_: update::Normal) -> Self { DynamicUpdateType::Normal(update::Normal) }
}

impl From<update::BW> for DynamicUpdateType {
    fn from(_: update::BW) -> Self { DynamicUpdateType::BW(update::BW) }
}

impl From<update::A2> for DynamicUpdateType {
    fn from(_: update::A2) -> Self { DynamicUpdateType::A2(update::A2) }
}


pub fn dynamic_update(tp: DynamicUpdateType, update_rect: Rect) {
    match tp {
        DynamicUpdateType::Normal(_) => unsafe { 
            c_iv::DynamicUpdate(
                update_rect.pos.x as c_int, 
                update_rect.pos.y as c_int, 
                update_rect.size.x as c_int, 
                update_rect.size.y as c_int
            ) 
        },
        DynamicUpdateType::BW(_) => unsafe { 
            c_iv::DynamicUpdateBW(
                update_rect.pos.x as c_int, 
                update_rect.pos.y as c_int, 
                update_rect.size.x as c_int, 
                update_rect.size.y as c_int
            ) 
        },
        DynamicUpdateType::A2(_) => unsafe {
            c_iv::DynamicUpdateA2(
                update_rect.pos.x as c_int, 
                update_rect.pos.y as c_int, 
                update_rect.size.x as c_int, 
                update_rect.size.y as c_int
            )
        }
    }
}

pub fn exit_update_a2() {
    unsafe { c_iv::ExitUpdateA2() }
}

pub fn is_in_a2_update() -> bool {
    unsafe { c_iv::IsInA2Update() > 0 }
}



/// ARGB32 format
#[derive(Debug, Copy, Clone)]
pub struct Color32(pub u32);

impl Color32 {
    pub const BLACK: Color32 = Color32(0xff000000);
    pub const GRAY: Color32 = Color32(0xff888888);
    pub const WHITE: Color32 = Color32(0xffffffff);
    
    pub const TRANSPERENT: Color32 = Color32(0x00000000);

    pub const fn none_if_transperent(self) -> Option<Color32> { if self.is_transperent() { None } else { Some(self) } }

    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::argb(0xff, red, green, blue)
    }

    pub const fn argb(alpha: u8, red: u8, green: u8, blue: u8) -> Self {
        Self(((alpha as u32) << 24) + ((red as u32) << 16) + ((green as u32) << 8) + blue as u32)
    }

    pub const fn a(&self) -> u8 { (self.0 >> 24) as u8 }
    pub const fn r(&self) -> u8 { (self.0 >> 16) as u8 }
    pub const fn g(&self) -> u8 { (self.0 >> 8) as u8 }
    pub const fn b(&self) -> u8 { (self.0 >> 0) as u8 }

    pub const fn avr(&self) -> u8 { ((self.r() as u16 + self.g() as u16 + self.b() as u16) / 3) as u8 }
    pub const fn a_avr(&self) -> u8 { ((self.a() as u16 + self.r() as u16 + self.g() as u16 + self.b() as u16) / 4) as u8 }

    pub const fn is_transperent(&self) -> bool { self.a() == 0 }
}

pub fn draw_circle(position: VecI32, radius: i32, color: Color32) {
    unsafe { c_iv::DrawCircle(position.x, position.y, radius, color.0 as c_int) }
}

pub fn draw_circle_quarter(center: VecI32, radius: u32, style: Style, thickness: u32, color: Color32, bg_color: Color32) -> Rect {
    unsafe { 
        c_iv::DrawCircleQuarter(
            center.x, 
            center.y, 
            radius as c_int, 
            style.bits as c_int, 
            thickness as c_int, 
            color.0 as c_int, 
            bg_color.0 as c_int
        );
    }
    Rect { 
        pos: VecI32 { x: center.x - radius as i32, y: center.y - radius as i32 }, 
        size: VecU32 { x: radius * 2, y: radius * 2 } 
    }
}

pub fn fill_area(rect: Rect, color: Color32) {
    unsafe { c_iv::FillArea(rect.pos.x, rect.pos.y, rect.size.x as c_int, rect.size.y as c_int, color.0 as c_int) }
}

pub fn draw_pixel(pos: VecI32, color: Color32) {
    unsafe { c_iv::DrawPixel(pos.x, pos.y, color.0 as c_int) }
}

pub fn draw_line(pos0: VecI32, pos1: VecI32, color: Color32) -> Rect {
    unsafe { c_iv::DrawLine(pos0.x, pos0.y, pos1.x, pos1.y, color.0 as c_int); }
    Rect::from_points_auto_flip(pos0, pos1)
}

pub fn draw_line_ex(pos0: VecI32, pos1: VecI32, color: Color32, step: u32) -> Rect {
    unsafe { c_iv::DrawLineEx(pos0.x, pos0.y, pos1.x, pos1.y, color.0 as c_int, step as c_int) }
    Rect::from_points_auto_flip(pos0, pos1)
}

pub fn draw_dash_line(pos0: VecI32, pos1: VecI32, color: Color32, fill: c_uint, space: c_uint) -> Rect {
    unsafe { c_iv::DrawDashLine(pos0.x, pos0.y, pos1.x, pos1.y, color.0 as c_int, fill, space) }
    Rect::from_points_auto_flip(pos0, pos1)
}

pub fn draw_rect(rect: Rect, color: Color32) {
    unsafe { c_iv::DrawRect(rect.pos.x, rect.pos.y, rect.size.x as c_int, rect.size.y as c_int, color.0 as c_int) }
}

pub fn draw_frame_certified_ex(
    rect: Rect, 
    thickness: c_int /*enum edef_thickness*/, 
    sides: Side, 
    style: Style,
    radius: usize, 
    color: Color32, 
    bg_color: Color32
) -> Rect {
    unsafe {
        c_iv::FillArea(
            rect.pos.x + thickness, 
            rect.pos.y + thickness, 
            rect.size.x as c_int - thickness * 2, 
            rect.size.y as c_int - thickness * 2, 
            bg_color.0 as c_int
        );
        c_iv::DrawFrameCertifiedEx(
            rect.pos.x, 
            rect.pos.y, 
            rect.size.x as c_int, 
            rect.size.y as c_int, 
            thickness,
            sides.bits as c_int, 
            style.bits as c_int, 
            radius as c_int, 
            color.0 as c_int, 
            bg_color.0 as c_int
        );
    }
    rect
}

#[derive(Debug)]
pub struct Font<'a> {
    pub name: &'a str,
    pub family: &'a str,
    pub size: usize,
    pub aa: u8,
    pub isbold: bool,
    pub isitalic: bool,
    pub _r1: u8,
    pub charset: u16,
    pub _r2: u16,
    pub color: Color32,
    pub height: usize,
    pub linespacing: usize,
    pub baseline: usize,

    pub(crate) c_data: *mut c_iv::c_ifont
}

// DEFAULT FONTS
pub enum FontType {
    Std = 0,
    Bold = 1,
    Italic = 2,
    Bolditalic = 3,
    Mono = 4,
    Family = 5,
}

pub fn get_default_font(fonttype: FontType) -> &'static str {
    unsafe { CStr::from_ptr(c_iv::iv_get_default_font(fonttype as c_int)).to_str().unwrap() }
}

pub fn open_font(name: &str, size: usize, aa: u8) -> Font<'static> {
    unsafe { 
        let font = &mut*c_iv::OpenFont(name.as_ptr() as *const c_char, size as c_int, aa as c_int);
        Font { 
            name: CStr::from_ptr(font.name).to_str().unwrap(), 
            family: CStr::from_ptr(font.family).to_str().unwrap(), 
            size: font.size as usize, 
            aa: font.aa, 
            isbold: font.isbold > 0, 
            isitalic: font.isitalic > 0, 
            _r1: font._r1, 
            charset: font.charset, 
            _r2: font._r2, 
            color: Color32(font.color as u32), 
            height: font.height as usize, 
            linespacing: font.linespacing as usize, 
            baseline: font.baseline as usize, 
            c_data: font
        }
    }
}

pub fn set_font(font: &Font<'_>, color: Color32) {
    unsafe { c_iv::SetFont(font.c_data, color.0 as c_int) }
}

pub fn draw_string(pos: VecI32, s: &str) {
    unsafe { c_iv::DrawString(pos.x, pos.y, s.as_ptr() as *const c_char) }
}

pub fn draw_string_r(pos: VecI32, s: &str) {
    unsafe { c_iv::DrawStringR(pos.x, pos.y, s.as_ptr() as *const c_char) } 
}

pub fn text_rect_height(width: usize, s: &str, flags: i32) -> c_int {
    unsafe { c_iv::TextRectHeight(width as c_int, s .as_ptr() as *const c_char, flags) }
}

pub fn text_rect_height_ex(size: VecU32, s: &str, flags: i32) -> c_int {
    unsafe { c_iv::TextRectHeightEx(size.x as c_int, size.y as c_int, s.as_ptr() as *const c_char, flags) }
}

pub fn minimal_text_rect_width(w: usize, s: &str) -> usize {
    unsafe { c_iv::MinimalTextRectWidth(w as c_int, s.as_ptr() as *const c_char) as usize }
}

pub fn draw_text_rect(rect: Rect, s: &str, flags: i32) -> (Rect, &CStr) {
    unsafe {         
        (rect, CStr::from_ptr(c_iv::DrawTextRect(rect.pos.x, rect.pos.y, rect.size.x as c_int, rect.size.y as c_int, s.as_ptr() as *const c_char, flags)))
    }    
}


pub fn pixels_per_point32() -> Result<f32, Error> {
    get_screen_scale_factor().and_then(|sf| get_screen_dpi().map(|dpi| sf as f32 * dpi as f32 / 160.))
}

pub fn pixels_per_point64() -> Result<f64, Error> {
    get_screen_scale_factor().and_then(|sf| get_screen_dpi().map(|dpi| sf * dpi as f64 / 160.))
}

pub fn dp_to_pix32(v: f32) -> Result<f32, Error> {
    pixels_per_point32().map(|ppp| ppp * v)
}
pub fn dp_to_pix64(v: f64) -> Result<f64, Error> {
    pixels_per_point64().map(|ppp| ppp * v)
}
