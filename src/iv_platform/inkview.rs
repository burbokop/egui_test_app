#[allow(unsafe_code)]

//#[repr(C)]
//struct subtaskinfo {
//    pub id: std::os::raw::c_int,
//    pub name: *mut libc::c_char,
//    pub book: *mut libc::c_char,
//    pub fgindex: std::os::raw::c_int,
//    pub order: std::os::raw::c_int,
//    pub rsv_1s: std::os::raw::c_int,
//};
//
//#[repr(C)]
//struct taskinfo {
//    task: std::os::raw::c_int,
//    nsubtasks: std::os::raw::c_int,
//    flags: std::os::raw::c_uint,
//    fbshmkey: std::os::raw::c_int,
//    fbshmsize: std::os::raw::c_int,
//    pid_t mainpid;
//    char *appname;
//    ibitmap *icon;
//    subtaskinfo *subtasks;
//    fbtempkey: std::os::raw::c_int;
//    rsv_2: std::os::raw::c_int;
//    rsv_3: std::os::raw::c_int;
//    rsv_4: std::os::raw::c_int;
//}

use std::ffi::CStr;
use std::{os::raw::{c_int, c_char, c_uint, c_uchar, c_ushort, c_double}, ffi::{c_void}, mem, fmt::Debug};
use egui::Vec2;
use num_enum::{TryFromPrimitive};


///
/// typedef struct icanvas_s {
///     int width;
///     int height;
///     int scanline;
///     int depth;
///     int clipx1, clipx2;
///     int clipy1, clipy2;
///     unsigned char *addr;
/// } icanvas;
/// 
#[repr(C)]
struct c_canvas {
	width: c_int,
	height: c_int,
    scanline: c_int,
    depth: c_int,
    clipx1: c_int,
    clipx2: c_int,
    clipy1: c_int,
    clipy2: c_int,
    addr: *mut u8,
}


#[repr(C)]
pub(crate) struct c_ifont {
    name: *mut c_char,
    family: *mut c_char,
    size: c_int,
    aa: c_uchar,
    isbold: c_uchar,
    isitalic: c_uchar,
    _r1: c_uchar,
    charset: c_ushort,
    _r2: c_ushort,
    color: c_int,
    height: c_int,
    linespacing: c_int,
    baseline: c_int,
    fdata: *mut c_void,
}

extern "C" {
    fn GetCurrentTask() -> c_int;
    //taskinfo *GetTaskInfo(pid: std::os::raw::c_int);

    fn PrepareForLoop(event_handler: Option<extern "C" fn(c_int, c_int, c_int) -> c_int>);
    fn PrepareForLoopEx(ctx: *mut c_void, event_handler: Option<extern "C" fn(*mut c_void, c_int, c_int, c_int) -> c_int>);
    fn ProcessEventLoop();
    fn ClearOnExit();

    fn GetDeviceModel() -> *mut c_char;
    fn GetHardwareType() -> *mut c_char;
    fn GetSoftwareVersion() -> *mut c_char;

    fn ScreenWidth() -> c_int;
    fn ScreenHeight() -> c_int;

    fn SetHardTimerEx(name: *const c_char, tproc: Option<extern "C" fn(*mut c_void) -> ()>, context: *mut c_void, ms: c_int);

    fn get_screen_dpi() -> c_int;
    fn get_screen_scale_factor() -> c_double;

    fn GetCanvas() -> *mut c_canvas;

    fn FullUpdate();
    fn FullUpdateHQ();
    fn SoftUpdate();
    fn SoftUpdateHQ();
    fn PartialUpdate(x: c_int, y: c_int, w: c_int, h: c_int);
    fn PartialUpdateBlack(x: c_int, y: c_int, w: c_int, h: c_int);
    fn PartialUpdateBW(x: c_int, y: c_int, w: c_int, h: c_int);
    fn PartialUpdateHQ(x: c_int, y: c_int, w: c_int, h: c_int);
    fn PartialUpdateDU4(x: c_int, y: c_int, w: c_int, h: c_int);
    fn DynamicUpdate(x: c_int, y: c_int, w: c_int, h: c_int);
    fn DynamicUpdateBW(x: c_int, y: c_int, w: c_int, h: c_int);

    fn DrawCircle(x0: c_int, y0: c_int, radius: c_int, color: c_int);

    fn DrawCircleQuarter(x0: c_int, y0: c_int, radius: c_int, direction: c_int /* enum estyle */, thickness: c_int, color: c_int, bg_color: c_int);

    fn FillArea(x: c_int, y: c_int, w: c_int, h: c_int, color: c_int);

    fn DrawPixel(x: c_int, y: c_int, color: c_int);
    fn DrawLine(x1: c_int, y1: c_int, x2: c_int, y2: c_int, color: c_int);
    fn DrawLineEx(x1: c_int, y1: c_int, x2: c_int, y2: c_int, color: c_int, step: c_int);
    fn DrawDashLine(x1: c_int, y1: c_int, x2: c_int, y2: c_int, color: c_int, fill: c_uint, space: c_uint);
    fn DrawRect(x: c_int, y: c_int, w: c_int, h: c_int, color: c_int);

    fn DrawFrameCertifiedEx(x: c_int, y: c_int, w: c_int, h: c_int, /*enum edef_thickness*/thickness: c_int, /*eside*/sides: c_int, /*enum estyle*/direction: c_int, radius: c_int, color: c_int, bg_color: c_int);

    fn iv_get_default_font(fonttype: c_int) -> *mut c_char;

    fn OpenFont(name: *const c_char, size: c_int, aa: c_int) -> *mut c_ifont;
    fn SetFont(font: *const c_ifont, color: c_int);

    fn DrawString(x: c_int, y: c_int, s: *const c_char);
    fn DrawStringR(x: c_int, y: c_int, s: *const c_char);
    fn TextRectHeight(width: c_int, s: *const c_char, flags: c_int) -> c_int;
    fn TextRectHeightEx(width: c_int, height: c_int, s: *const c_char, flags: c_int) -> c_int;
    fn MinimalTextRectWidth(w: c_int, s: *const c_char) -> c_int;
    fn DrawTextRect(x: c_int, y: c_int, w: c_int, h: c_int, s: *const c_char, flags: c_int) -> *mut c_char;
}

#[allow(unsafe_code)]
pub fn current_task() -> i32 {
    unsafe { GetCurrentTask() }
}

pub fn device_model() -> &'static str {
    unsafe { CStr::from_ptr(GetDeviceModel()) }.to_str().unwrap() 
}

pub fn hardware_type() -> &'static str {
    unsafe { CStr::from_ptr(GetHardwareType()) }.to_str().unwrap() 
}

pub fn software_version() -> &'static str {
    unsafe { CStr::from_ptr(GetSoftwareVersion()) }.to_str().unwrap() 
}


#[derive(TryFromPrimitive)]
#[repr(i32)]
#[derive(Debug)]
pub enum EventType {
    Init = 21,
    Exit = 22,
    Show = 23,
    Hide = 24,
    KeyPress = 25,
    KeyRelease = 26,
    KeyRepeat = 28,

    KeyPressExt = 40,
    KeyReleaseExt = 41,
    KeyRepeatExt = 42,
    PointerUp = 29,
    PointerDown = 30,
    PointerMove = 31,

//comes from inkview only after calling AddScrolledArea function
    Scroll = 33, //par1 is (irect *) -- scrolled area from wich scrolling was started, par2 is (deltaX (highest word) and deltaY(lowest word))
    PointerLong = 34,
    PointerHold = 35,
    PointerDrag = 44, //like EVT_POINTERMOVE, but has non sensitive zone, which smooths finger touch bounce.
    PointerCancel = 45,
    PointerChanged = 46,

    Orientation = 32,
    Focus = 36,
    Unfocus = 37,
    Activate = 38,
    MtSync = 39,
    TouchUp = 47,
    TouchDown = 48,
    TouchMove = 49,
    Repaint = 43,

    QnMove = 51,

    QnReleaseEASE = 52,

    QnBorder = 53,

    Snapshot = 71,
    Fsincoming = 72,
    Fschanged = 73,

    MpStatechanged = 81,
    MpTrackchanged = 82,

    Prevpage = 91,
    Nextpage = 92,
    Opendic = 93,
    ControlPanelAboutToOpen = 94,
    Update = 95,

    PanelBluetoothA2dp = 118,

    Tab = 119,
    Panel = 120,
    PanelIcon = 121,
    PanelText = 122,
    PanelProgress = 123,
    PanelMplayer = 124,
    PanelUsbdrive = 125,
    PanelNetwork = 126,
    PanelClock = 127,
    PanelBluetooth = 128,
    PanelTasklist = 129,
    PanelObreeySync = 130,
    PanelSetreadingmode = 131,
    PanelSetreadingmodeInvert = 132,
    PanelFrontLight = 133,

    Globalrequest = 149,
/* 
enum globalaction_on_event_e {
	GLOBALACTION_ON_KEYPRESS = 0, //masked by mpc->gka0
	GLOBALACTION_ON_KEYHOLD, //masked by mpc->gka1
	GLOBALACTION_ON_DOUBLECLICK, //masked by mpc->gka2
};*/
    Globalaction = 150, //send to taskmanager par1 = key, par2 = enum globalaction_on_event_e
    Foreground = 151,
    Background = 152,
    Subtaskclose = 153,
    Configchanged = 154,
    Savestate = 155,
    ObreeyConfigChanged = 156,

    Sdin = 161,
    Sdout = 162,
    UsbstoreIn	= 163,
    UsbstoreOut = 164,

    BtRxcomplete = 171,
    BtTxcomplete = 172,

    SynthEnded = 200,
    DicClosedARD = 202,
    ShowKeyboard201,

    Textclear = 209,
    ExtKb = 210,
    Letter = 211,

    Callback = 212,

    Scanprogress = 213,
    Stopscan = 214,
    Startscan = 215,
    Scanstopped = 216,
    PostponeTimedPoweroff = 217,
    FrameActivated = 218,
    FrameDeactivated = 219,
    ReadProgressChanged = 220,
    DumpBitmapsDebugInfo = 221,

    NetConnected = 256,
    NetDisconnected = 257,
    NetFoundNewFw = 260,
    SynthPosition = 261,
    AsyncTaskFinished = 262, // used for framework-2 async_code realization

    StopPlaying = 263,
    AvrcpCommand = 264,

    AudioChanged = 265, //audio output routing was changed

    PackageJobChanged = 266,
    Custom = 267,
}

#[derive(Debug)]
pub struct Event {
    pub event_type: EventType,
    pub key: u8,
}

extern "C" fn event_handler(arg0: c_int, arg1: c_int, _: c_int) -> c_int {
    if let Some(f) = unsafe { &mut EVENT_HANDLER_CONTEXT } {
        f(Event { event_type: EventType::try_from(arg0).unwrap(), key: arg1 as u8 }).into()
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
            PrepareForLoop(Some(event_handler)) 
        }
    }
}


extern "C" fn prepare_for_loop_ex_handler(ctx: *mut c_void, arg0: c_int, arg1: c_int, _: c_int) -> c_int {
    println!("AAAAAAAAAAAAA");
    //let closure: &mut &mut dyn FnMut(Event) -> bool = unsafe { mem::transmute(ctx) };
    //closure(Event { event_type: EventType::try_from(arg0).unwrap(), key: arg1 as u8 }) as c_int

    let closure: &mut Box<dyn FnMut(Event) -> bool> = unsafe { mem::transmute(ctx) };
    closure(Event { event_type: EventType::try_from(arg0).unwrap(), key: arg1 as u8 }) as c_int
}

pub fn prepare_for_loop_ex<F: FnMut(Event) -> bool /* true if accepted */>(f: F) {
    //let mut f_mut = f;
    //let mut cb: &mut dyn FnMut(Event) -> bool = &mut f_mut;
    //let cb = &mut cb;

    let cb: Box<Box<dyn FnMut(Event) -> bool>> = Box::new(Box::new(f));
 
    //unsafe { PrepareForLoopEx(cb as *mut _ as *mut c_void, Some(prepare_for_loop_ex_handler)) }

    unsafe { PrepareForLoopEx(Box::into_raw(cb) as *mut _, Some(prepare_for_loop_ex_handler)) }

    
}

pub fn process_event_loop() { unsafe { ProcessEventLoop() } }
pub fn clear_on_exit() { unsafe { EVENT_HANDLER_CONTEXT = None; ClearOnExit() } }

pub fn screen_width() -> Result<u32, <i32 as TryInto<u32>>::Error> { unsafe { ScreenWidth() }.try_into() }
pub fn screen_height() -> Result<u32, <i32 as TryInto<u32>>::Error> { unsafe { ScreenHeight() }.try_into() }

extern "C" fn set_hard_timer_handler(ctx: *mut c_void) {
    let closure: &mut &mut dyn FnMut() = unsafe { mem::transmute(ctx) };
    closure()
}

pub fn set_hard_timer<F: FnMut()>(name: &str, f: F, ms: u32) {
    println!("OOOOOOOOOOOOOOOOOOO");

    let mut f_mut = f;
    let mut cb: &mut dyn FnMut() = &mut f_mut;
    let cb = &mut cb;

    unsafe { SetHardTimerEx(name.as_ptr() as *const c_char, Some(set_hard_timer_handler), cb as *mut _ as *mut c_void, ms as i32) }
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

impl Default for Side {
    fn default() -> Self { Self::all() }
}

pub fn get_screen_dpi2() -> usize {
    unsafe { get_screen_dpi() as usize }
}

pub fn get_screen_scale_factor2() -> f64 {
    unsafe { get_screen_scale_factor() }
}

pub fn get_canvas() -> Canvas<'static> {
    let cvs = unsafe { &mut*GetCanvas() };
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

pub fn full_update(tp: FullSoftUpdateType) {
    match tp {
        FullSoftUpdateType::Normal(_) => unsafe { FullUpdate() },
        FullSoftUpdateType::HQ(_) => unsafe { FullUpdateHQ() },
    }
}

pub fn soft_update(tp: FullSoftUpdateType) {
    match tp {
        FullSoftUpdateType::Normal(_) => unsafe { SoftUpdate() },
        FullSoftUpdateType::HQ(_) => unsafe { SoftUpdateHQ() },
    }
}

pub enum PartialUpdateType {
    Normal(update_type::Normal),
    HQ(update_type::HQ),
    Black(update_type::Black),
    BW(update_type::BW),
    DU4(update_type::DU4)
}

pub fn partial_update(tp: PartialUpdateType, x: usize, y: usize, w: usize, h: usize) {
    match tp {
        PartialUpdateType::Normal(_) => unsafe { PartialUpdate(x as c_int, y as c_int, w as c_int, h as c_int) },
        PartialUpdateType::HQ(_) => unsafe { PartialUpdateHQ(x as c_int, y as c_int, w as c_int, h as c_int) },
        PartialUpdateType::Black(_) => unsafe { PartialUpdateBlack(x as c_int, y as c_int, w as c_int, h as c_int) },
        PartialUpdateType::BW(_) => unsafe { PartialUpdateBW(x as c_int, y as c_int, w as c_int, h as c_int) },
        PartialUpdateType::DU4(_) => unsafe { PartialUpdateDU4(x as c_int, y as c_int, w as c_int, h as c_int) },
    }
}

pub enum DynamicUpdateType {
    Normal(update_type::Normal),
    BW(update_type::BW),
}

pub fn dynamic_update(tp: DynamicUpdateType, x: usize, y: usize, w: usize, h: usize) {
    match tp {
        DynamicUpdateType::Normal(_) => unsafe { DynamicUpdate(x as c_int, y as c_int, w as c_int, h as c_int) },
        DynamicUpdateType::BW(_) => unsafe { DynamicUpdateBW(x as c_int, y as c_int, w as c_int, h as c_int) },
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

pub fn draw_circle(position: VecI32, radius: i32, color: Color32) {
    unsafe { DrawCircle(position.x, position.y, radius, color.0 as c_int) }
}

pub fn draw_circle_quarter(pos: VecI32, radius: u32, style: Style, thickness: u32, color: Color32, bg_color: Color32) {
    unsafe { 
        DrawCircleQuarter(
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

pub fn fill_area(rect: Rect, color: Color32) {
    unsafe { FillArea(rect.pos.x, rect.pos.y, rect.size.x as c_int, rect.size.y as c_int, color.0 as c_int) }
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
    unsafe { DrawRect(rect.pos.x, rect.pos.y, rect.size.x as c_int, rect.size.y as c_int, color.0 as c_int) }
}

pub fn draw_frame_certified_ex(
    rect: Rect, 
    thickness: c_int /*enum edef_thickness*/, 
    sides: Side, 
    style: Style,
    radius: usize, 
    color: Color32, 
    bg_color: Color32
) {
    unsafe {
        DrawFrameCertifiedEx(
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

    pub(crate) c_data: *mut c_ifont
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
    unsafe { CStr::from_ptr(iv_get_default_font(fonttype as c_int)).to_str().unwrap() }
}

pub fn open_font(name: &str, size: usize, aa: u8) -> Font<'static> {
    unsafe { 
        let font = &mut*OpenFont(name.as_ptr() as *const c_char, size as c_int, aa as c_int);
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
    unsafe { SetFont(font.c_data, color.0 as c_int) }
}

pub fn draw_string(pos: VecI32, s: &str) {
    unsafe { DrawString(pos.x, pos.y, s.as_ptr() as *const c_char) }
}

pub fn draw_string_r(x: c_int, y: c_int, s: *const c_char) {

}

pub fn text_rect_height(width: usize, s: &str, flags: i32) -> c_int {
    unsafe { TextRectHeight(width as c_int, s .as_ptr() as *const c_char, flags) }
}

pub fn text_rect_height_ex(size: VecUSize, s: &str, flags: i32) -> c_int {
    unsafe { TextRectHeightEx(size.x as c_int, size.y as c_int, s.as_ptr() as *const c_char, flags) }
}

pub fn minimal_text_rect_width(w: usize, s: &str) -> usize {
    unsafe { MinimalTextRectWidth(w as c_int, s.as_ptr() as *const c_char) as usize }
}

pub fn draw_text_rect(rect: Rect, s: &str, flags: i32) -> &CStr {
    unsafe {         
        CStr::from_ptr(DrawTextRect(rect.pos.x, rect.pos.y, rect.size.x as c_int, rect.size.y as c_int, s.as_ptr() as *const c_char, flags))
    }
}