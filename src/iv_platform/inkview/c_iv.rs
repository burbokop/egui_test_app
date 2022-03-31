use std::{os::raw::{c_int, c_char, c_uchar, c_ushort, c_double, c_uint}, ffi::c_void};




#[repr(C)]
pub(super) struct c_canvas {
	pub width: c_int,
	pub height: c_int,
    pub scanline: c_int,
    pub depth: c_int,
    pub clipx1: c_int,
    pub clipx2: c_int,
    pub clipy1: c_int,
    pub clipy2: c_int,
    pub addr: *mut u8,
}


#[repr(C)]
pub(crate) struct c_ifont {
    pub name: *mut c_char,
    pub family: *mut c_char,
    pub size: c_int,
    pub aa: c_uchar,
    pub isbold: c_uchar,
    pub isitalic: c_uchar,
    pub _r1: c_uchar,
    pub charset: c_ushort,
    pub _r2: c_ushort,
    pub color: c_int,
    pub height: c_int,
    pub linespacing: c_int,
    pub baseline: c_int,
    pub fdata: *mut c_void,
}

extern "C" {
    pub(super) fn GetCurrentTask() -> c_int;
    //taskinfo *GetTaskInfo(pid: std::os::raw::c_int);

    pub(super) fn PrepareForLoop(event_handler: Option<extern "C" fn(c_int, c_int, c_int) -> c_int>);
    pub(super) fn PrepareForLoopEx(ctx: *mut c_void, event_handler: Option<extern "C" fn(*mut c_void, c_int, c_int, c_int) -> c_int>);
    pub(super) fn ProcessEventLoop();
    pub(super) fn ClearOnExit();

    pub(super) fn GetDeviceModel() -> *mut c_char;
    pub(super) fn GetHardwareType() -> *mut c_char;
    pub(super) fn GetSoftwareVersion() -> *mut c_char;

    pub(super) fn ScreenWidth() -> c_int;
    pub(super) fn ScreenHeight() -> c_int;

    pub(super) fn SetHardTimerEx(name: *const c_char, tproc: Option<extern "C" fn(*mut c_void) -> ()>, context: *mut c_void, ms: c_int);

    pub(super) fn get_screen_dpi() -> c_int;
    pub(super) fn get_screen_scale_factor() -> c_double;

    pub(super) fn GetCanvas() -> *mut c_canvas;

    pub(super) fn FullUpdate();
    pub(super) fn FullUpdateHQ();
    pub(super) fn SoftUpdate();
    pub(super) fn SoftUpdateHQ();
    pub(super) fn PartialUpdate(x: c_int, y: c_int, w: c_int, h: c_int);
    pub(super) fn PartialUpdateBlack(x: c_int, y: c_int, w: c_int, h: c_int);
    pub(super) fn PartialUpdateBW(x: c_int, y: c_int, w: c_int, h: c_int);
    pub(super) fn PartialUpdateHQ(x: c_int, y: c_int, w: c_int, h: c_int);
    pub(super) fn PartialUpdateDU4(x: c_int, y: c_int, w: c_int, h: c_int);
    pub(super) fn DynamicUpdate(x: c_int, y: c_int, w: c_int, h: c_int);
    pub(super) fn DynamicUpdateBW(x: c_int, y: c_int, w: c_int, h: c_int);

    pub(super) fn DrawCircle(x0: c_int, y0: c_int, radius: c_int, color: c_int);

    pub(super) fn DrawCircleQuarter(x0: c_int, y0: c_int, radius: c_int, direction: c_int /* enum estyle */, thickness: c_int, color: c_int, bg_color: c_int);

    pub(super) fn FillArea(x: c_int, y: c_int, w: c_int, h: c_int, color: c_int);

    pub(super) fn DrawPixel(x: c_int, y: c_int, color: c_int);
    pub(super) fn DrawLine(x1: c_int, y1: c_int, x2: c_int, y2: c_int, color: c_int);
    pub(super) fn DrawLineEx(x1: c_int, y1: c_int, x2: c_int, y2: c_int, color: c_int, step: c_int);
    pub(super) fn DrawDashLine(x1: c_int, y1: c_int, x2: c_int, y2: c_int, color: c_int, fill: c_uint, space: c_uint);
    pub(super) fn DrawRect(x: c_int, y: c_int, w: c_int, h: c_int, color: c_int);

    pub(super) fn DrawFrameCertifiedEx(x: c_int, y: c_int, w: c_int, h: c_int, /*enum edef_thickness*/thickness: c_int, /*eside*/sides: c_int, /*enum estyle*/direction: c_int, radius: c_int, color: c_int, bg_color: c_int);

    pub(super) fn iv_get_default_font(fonttype: c_int) -> *mut c_char;

    pub(super) fn OpenFont(name: *const c_char, size: c_int, aa: c_int) -> *mut c_ifont;
    pub(super) fn SetFont(font: *const c_ifont, color: c_int);

    pub(super) fn DrawString(x: c_int, y: c_int, s: *const c_char);
    pub(super) fn DrawStringR(x: c_int, y: c_int, s: *const c_char);
    pub(super) fn TextRectHeight(width: c_int, s: *const c_char, flags: c_int) -> c_int;
    pub(super) fn TextRectHeightEx(width: c_int, height: c_int, s: *const c_char, flags: c_int) -> c_int;
    pub(super) fn MinimalTextRectWidth(w: c_int, s: *const c_char) -> c_int;
    pub(super) fn DrawTextRect(x: c_int, y: c_int, w: c_int, h: c_int, s: *const c_char, flags: c_int) -> *mut c_char;
}
