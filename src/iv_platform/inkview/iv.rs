use std::{ffi::{CStr, c_void}, os::raw::{c_int, c_char, c_uint}, mem};
use std::fmt::Debug;


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

#[derive(Debug)]
pub struct VecI32 {
    pub x: i32,
    pub y: i32
}

#[derive(Debug)]
pub struct VecUSize {
    pub x: usize,
    pub y: usize
}

#[derive(Debug)]
pub struct Rect {
    pub pos: VecI32,
    pub size: VecUSize,
}

#[repr(C)]
pub struct Canvas<'a> {
	pub width: usize,
	pub height: usize,
    pub scanline: usize,
    pub depth: usize,
    pub clip_rect: Rect,
    pub pixels: &'a mut [u8],
}


impl<'a> Debug for Canvas<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Canvas")
            .field("width", &self.width)
            .field("height", &self.height)
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
        width: cvs.width as usize, 
        height: cvs.height as usize, 
        scanline: cvs.scanline as usize,
        depth: cvs.depth as usize, 
        clip_rect: Rect { 
            pos: VecI32 { 
                x: cvs.clipx1, 
                y: cvs.clipy1 
            }, 
            size: VecUSize { 
                x: (cvs.clipx2 - cvs.clipx1) as usize, 
                y: (cvs.clipy2 - cvs.clipy1) as usize 
            } 
        },
        pixels: unsafe { 
            core::slice::from_raw_parts_mut(cvs.addr, (cvs.width * cvs.height * cvs.depth).try_into().unwrap()) 
        }
    }
}

pub mod update_type {
    pub struct Normal;
    pub struct HQ;
    pub struct Black;
    pub struct BW;
    pub struct DU4;
}

pub enum FullSoftUpdateType {
    Normal(update_type::Normal),
    HQ(update_type::HQ)
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
    Normal(update_type::Normal),
    HQ(update_type::HQ),
    Black(update_type::Black),
    BW(update_type::BW),
    DU4(update_type::DU4)
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
    Normal(update_type::Normal),
    BW(update_type::BW),
}

pub fn dynamic_update(tp: &DynamicUpdateType, x: usize, y: usize, w: usize, h: usize) {
    match tp {
        DynamicUpdateType::Normal(_) => unsafe { c_iv::DynamicUpdate(x as c_int, y as c_int, w as c_int, h as c_int) },
        DynamicUpdateType::BW(_) => unsafe { c_iv::DynamicUpdateBW(x as c_int, y as c_int, w as c_int, h as c_int) },
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Color32(pub u32);

impl Color32 {
    pub const BLACK: Color32 = Color32(0x000000);
    pub const GRAY: Color32 = Color32(0x888888);
    pub const WHITE: Color32 = Color32(0xffffff);

    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self(((blue as u32) << 16) + ((green as u32) << 8) + red as u32)
    }
}

pub fn draw_circle(position: &VecI32, radius: i32, color: Color32) {
    unsafe { c_iv::DrawCircle(position.x, position.y, radius, color.0 as c_int) }
}

pub fn draw_circle_quarter(pos: &VecI32, radius: u32, style: Style, thickness: u32, color: Color32, bg_color: Color32) {
    unsafe { 
        c_iv::DrawCircleQuarter(
            pos.x, 
            pos.y, 
            radius as c_int, 
            style.bits as c_int, 
            thickness as c_int, 
            color.0 as c_int, 
            bg_color.0 as c_int
        )
    }
}

pub fn fill_area(rect: &Rect, color: Color32) {
    unsafe { c_iv::FillArea(rect.pos.x, rect.pos.y, rect.size.x as c_int, rect.size.y as c_int, color.0 as c_int) }
}

pub fn draw_pixel(x: c_int, y: c_int, color: c_int) {

}

pub fn draw_line(x1: c_int, y1: c_int, x2: c_int, y2: c_int, color: c_int) {

}

pub fn draw_line_ex(x1: c_int, y1: c_int, x2: c_int, y2: c_int, color: c_int, step: c_int) {

}

pub fn draw_dash_line(x1: c_int, y1: c_int, x2: c_int, y2: c_int, color: c_int, fill: c_uint, space: c_uint) {

}

pub fn draw_rect(rect: Rect, color: Color32) {
    unsafe { c_iv::DrawRect(rect.pos.x, rect.pos.y, rect.size.x as c_int, rect.size.y as c_int, color.0 as c_int) }
}

pub fn draw_frame_certified_ex(
    rect: &Rect, 
    thickness: c_int /*enum edef_thickness*/, 
    sides: &Side, 
    style: &Style,
    radius: usize, 
    color: Color32, 
    bg_color: Color32
) {
    unsafe {
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
        )
    }
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

pub fn draw_string_r(x: c_int, y: c_int, s: *const c_char) {

}

pub fn text_rect_height(width: usize, s: &str, flags: i32) -> c_int {
    unsafe { c_iv::TextRectHeight(width as c_int, s .as_ptr() as *const c_char, flags) }
}

pub fn text_rect_height_ex(size: VecUSize, s: &str, flags: i32) -> c_int {
    unsafe { c_iv::TextRectHeightEx(size.x as c_int, size.y as c_int, s.as_ptr() as *const c_char, flags) }
}

pub fn minimal_text_rect_width(w: usize, s: &str) -> usize {
    unsafe { c_iv::MinimalTextRectWidth(w as c_int, s.as_ptr() as *const c_char) as usize }
}

pub fn draw_text_rect(rect: Rect, s: &str, flags: i32) -> &CStr {
    unsafe {         
        CStr::from_ptr(c_iv::DrawTextRect(rect.pos.x, rect.pos.y, rect.size.x as c_int, rect.size.y as c_int, s.as_ptr() as *const c_char, flags))
    }
}