use std::time::Instant;

use egui::Event;
use epi::backend::{FrameData, RepaintSignal};
use queues::{Queue, IsQueue};

use super::convert::from_iv;

use super::inkview::{self as iv, NonZeroF32};

pub struct EpiIntegration {
    pub frame: epi::Frame,
    //last_auto_save: instant::Instant,
    pub egui_ctx: egui::Context,
    pending_full_output: egui::FullOutput,
    //egui_winit: crate::State,
    /// When set, it is time to quit
    quit: bool,
    can_drag_window: bool,
    pixels_per_point: iv::NonZeroF32,
    event_q: Queue<Event>
}


pub fn handle_app_output(app_output: epi::backend::AppOutput) {
    //let epi::backend::AppOutput {
    //    quit: _,
    //    window_size,
    //    window_title,
    //    decorated,
    //    drag_window,
    //} = app_output;

    //if let Some(decorated) = decorated {
    //    window.set_decorations(decorated);
    //}

    //if let Some(window_size) = window_size {
    //    window.set_inner_size(
    //        winit::dpi::PhysicalSize {
    //            width: (current_pixels_per_point * window_size.x).round(),
    //            height: (current_pixels_per_point * window_size.y).round(),
    //        }
    //        .to_logical::<f32>(crate::native_pixels_per_point(window) as f64),
    //    );
    //}

    //if let Some(window_title) = window_title {
    //    window.set_title(&window_title);
    //}

    //if drag_window {
    //    let _ = window.drag_window();
    //}

}


impl EpiIntegration {
    pub fn new(storage: Option<Box<dyn epi::Storage>>, pixels_per_point: iv::NonZeroF32, prefer_dark_mode: Option<bool>) -> Self {
        let egui_ctx = egui::Context::default();

        println!("pixels_per_point: {:?}", pixels_per_point);

        #[derive(Default)]
        struct RS {

        }

        impl RepaintSignal for RS {
            fn request_repaint(&self) {
                println!("request_repaint")
            }
        }

        let frame = epi::Frame::new(FrameData {
            info: epi::IntegrationInfo {
                name: "iv_integration",
                web_info: None,
                prefer_dark_mode: Some(false),
                cpu_usage: None,
                native_pixels_per_point: Some(pixels_per_point.to_f32()),
            },
            output: Default::default(),
            repaint_signal: std::sync::Arc::from(RS::default()),
        });
 
        if prefer_dark_mode == Some(true) {
            egui_ctx.set_visuals(egui::Visuals::dark());
        } else {
            egui_ctx.set_visuals(egui::Visuals::light());
        }
 
        Self { 
            can_drag_window: true, 
            egui_ctx: egui_ctx, 
            quit: false, 
            frame: frame, 
            pending_full_output: Default::default(), 
            pixels_per_point: pixels_per_point, 
            event_q: Default::default() 
        }
    }

    pub fn pixels_per_point(&self) -> NonZeroF32 { self.pixels_per_point }

    /*
    pub fn warm_up(&mut self, app: &mut dyn epi::App, window: &winit::window::Window) {
        let saved_memory: egui::Memory = self.egui_ctx.memory().clone();
        self.egui_ctx.memory().set_everything_is_visible(true);
        let full_output = self.update(app, window);
        self.pending_full_output.append(full_output); // Handle it next frame
        *self.egui_ctx.memory() = saved_memory; // We don't want to remember that windows were huge.
        self.egui_ctx.clear_animations();
    }
    */

    /// If `true`, it is time to shut down.
    pub fn should_quit(&self) -> bool {
        self.quit
    }
    /* 
    inkview::EventType::Exit => if app.on_exit_event() { 
        inkview::clear_on_exit(); 
        std::process::exit(0)
    } else { false },
*/

    pub fn convert_event_to_app<A: epi::App>(&mut self, app: &mut A, event: &iv::Event) -> Option<egui::Event> {

        //println!("event: {:?}, (ppp: {}) (sppp: {:?})", event, self.egui_ctx.pixels_per_point(), self.pixels_per_point);
        match event {
            iv::Event::Init => None,
            iv::Event::Exit => if app.on_exit_event() { 
                //inkview::clear_on_exit(); 
                std::process::exit(0)
            } else { None },
            iv::Event::Show => None,
            iv::Event::Hide => None,
            iv::Event::KeyPress => todo!(),
            iv::Event::KeyRelease => todo!(),
            iv::Event::KeyRepeat => todo!(),
            iv::Event::KeyPressExt => todo!(),
            iv::Event::KeyReleaseExt => todo!(),
            iv::Event::KeyRepeatExt => todo!(),
            iv::Event::PointerUp { pos } => Some(
                egui::Event::PointerButton {
                    pos: from_iv::emath_pos(*pos, self.pixels_per_point),
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::default(),
                }
            ),
            //Some(egui::Event::Touch {
            //    device_id: egui::TouchDeviceId(0),
            //    id: egui::TouchId(0),
            //    phase: egui::TouchPhase::End,
            //    pos: from_iv::emath_pos(*pos, self.pixels_per_point),
            //    force: 1.,
            //}),
            iv::Event::PointerDown { pos } => Some(
                egui::Event::PointerButton {
                    pos: from_iv::emath_pos(*pos, self.pixels_per_point),
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::default(),
                }
            ),
/*
            Some(egui::Event::Touch {
                device_id: egui::TouchDeviceId(0),
                id: egui::TouchId(0),
                phase: egui::TouchPhase::Start,
                pos: from_iv::emath_pos(*pos, self.pixels_per_point),
                force: 1.,
            }), */
            iv::Event::PointerMove { pos } => {
                None
            },
            iv::Event::Scroll => todo!(),
            iv::Event::PointerLong { pos } => { None },
            iv::Event::PointerHold { pos } => { None },
            iv::Event::PointerDrag { pos } => { 
                Some(egui::Event::PointerMoved(from_iv::emath_pos(*pos, self.pixels_per_point)))
            },
            iv::Event::PointerCancel { pos } => { None },
            iv::Event::PointerChanged { pos } => { None },
            iv::Event::Orientation => todo!(),
            iv::Event::Focus => todo!(),
            iv::Event::Unfocus => todo!(),
            iv::Event::Activate => todo!(),
            iv::Event::MtSync => None,
            iv::Event::TouchUp => todo!(),
            iv::Event::TouchDown => todo!(),
            iv::Event::TouchMove => todo!(),
            iv::Event::Repaint => todo!(),
            iv::Event::QnMove => todo!(),
            iv::Event::QnReleaseEASE => todo!(),
            iv::Event::QnBorder => todo!(),
            iv::Event::Snapshot => todo!(),
            iv::Event::Fsincoming => todo!(),
            iv::Event::Fschanged => todo!(),
            iv::Event::MpStatechanged => todo!(),
            iv::Event::MpTrackchanged => todo!(),
            iv::Event::Prevpage => todo!(),
            iv::Event::Nextpage => todo!(),
            iv::Event::Opendic => todo!(),
            iv::Event::ControlPanelAboutToOpen => todo!(),
            iv::Event::Update => todo!(),
            iv::Event::PanelBluetoothA2dp => todo!(),
            iv::Event::Tab => todo!(),
            iv::Event::Panel => todo!(),
            iv::Event::PanelIcon => todo!(),
            iv::Event::PanelText => todo!(),
            iv::Event::PanelProgress => todo!(),
            iv::Event::PanelMplayer => todo!(),
            iv::Event::PanelUsbdrive => todo!(),
            iv::Event::PanelNetwork => todo!(),
            iv::Event::PanelClock => todo!(),
            iv::Event::PanelBluetooth => todo!(),
            iv::Event::PanelTasklist => todo!(),
            iv::Event::PanelObreeySync => todo!(),
            iv::Event::PanelSetreadingmode => todo!(),
            iv::Event::PanelSetreadingmodeInvert => todo!(),
            iv::Event::PanelFrontLight => todo!(),
            iv::Event::GlobalRequest => todo!(),
            iv::Event::GlobalAction => todo!(),
            iv::Event::Foreground => None,
            iv::Event::Background => None,
            iv::Event::SubTaskClose => todo!(),
            iv::Event::ConfigChanged => todo!(),
            iv::Event::SaveState => todo!(),
            iv::Event::ObreeyConfigChanged => todo!(),
            iv::Event::Sdin => todo!(),
            iv::Event::Sdout => todo!(),
            iv::Event::UsbStoreIn => todo!(),
            iv::Event::UsbStoreOut => todo!(),
            iv::Event::BtRxComplete => todo!(),
            iv::Event::BtTxComplete => todo!(),
            iv::Event::SynthEnded => todo!(),
            iv::Event::DicClosedARD => todo!(),
            iv::Event::ShowKeyboard => todo!(),
            iv::Event::TextClear => todo!(),
            iv::Event::ExtKb => todo!(),
            iv::Event::Letter => todo!(),
            iv::Event::Callback => todo!(),
            iv::Event::ScanProgress => todo!(),
            iv::Event::StopScan => todo!(),
            iv::Event::StartScan => todo!(),
            iv::Event::ScanStopped => todo!(),
            iv::Event::PostponeTimedPowerOff => todo!(),
            iv::Event::FrameActivated => todo!(),
            iv::Event::FrameDeactivated => todo!(),
            iv::Event::ReadProgressChanged => todo!(),
            iv::Event::DumpBitmapsDebugInfo => todo!(),
            iv::Event::NetConnected => todo!(),
            iv::Event::NetDisconnected => todo!(),
            iv::Event::NetFoundNewFw => todo!(),
            iv::Event::SynthPosition => todo!(),
            iv::Event::AsyncTaskFinished => todo!(),
            iv::Event::StopPlaying => todo!(),
            iv::Event::AvrcpCommand => todo!(),
            iv::Event::AudioChanged => todo!(),
            iv::Event::PackageJobChanged => todo!(),
            iv::Event::Custom => todo!(),
        }
    }


    pub fn on_event<A: epi::App>(&mut self, app: &mut A, event: &iv::Event) -> bool {

        let e = self.convert_event_to_app(app, event);

        if let Some(ee) = e {
            self.event_q.add(ee).unwrap();
        }

        return true

        //self.egui_winit.on_event(&self.egui_ctx, event);
    }



    pub fn update(
        &mut self,
        app: &mut dyn epi::App,
    ) -> egui::FullOutput {
        let frame_start = Instant::now();


        let mut events: Vec<egui::Event> = Vec::with_capacity(self.event_q.size());
        while self.event_q.size() > 0 {
            events.push(self.event_q.peek().unwrap());
            self.event_q.remove().unwrap();
        }

        
        let raw_input = egui::RawInput { 
            screen_rect: Some(egui::Rect::from_min_size(
                Default::default(), 
                emath::Vec2::new(iv::screen_width().unwrap() as f32 / self.pixels_per_point, iv::screen_width().unwrap() as f32 / self.pixels_per_point)
            )),            
            pixels_per_point: Some(self.pixels_per_point.to_f32()),
            events: events,
            ..Default::default() 
        };


        //let raw_input = self.egui_winit.take_egui_input(window);
        let full_output = self.egui_ctx.run(raw_input, |egui_ctx| {
            app.update(egui_ctx, &self.frame);
        });
        self.pending_full_output.append(full_output);
        let full_output = std::mem::take(&mut self.pending_full_output);


        {
            let mut app_output = self.frame.take_app_output();
            app_output.drag_window &= self.can_drag_window; // Necessary on Windows; see https://github.com/emilk/egui/pull/1108
            self.can_drag_window = false;
            if app_output.quit {
                //self.quit = app.on_exit_event();
            }
            handle_app_output(app_output);
        }

        let frame_time = (Instant::now() - frame_start).as_secs_f64() as f32;
        self.frame.lock().info.cpu_usage = Some(frame_time);

        full_output
    }
/* 
    pub fn handle_platform_output(
        &mut self,
        window: &winit::window::Window,
        platform_output: egui::PlatformOutput,
    ) {
        self.egui_winit
            .handle_platform_output(window, &self.egui_ctx, platform_output);
    }
*/
    // ------------------------------------------------------------------------
    // Persistance stuff:
/* 
    pub fn maybe_autosave(&mut self, app: &mut dyn epi::App, window: &winit::window::Window) {
        let now = instant::Instant::now();
        if now - self.last_auto_save > app.auto_save_interval() {
            self.save(app, window);
            self.last_auto_save = now;
        }
    }
*/
/* 
    pub fn save(&mut self, _app: &mut dyn epi::App, _window: &winit::window::Window) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = self.frame.storage_mut() {
            if _app.persist_native_window() {
                epi::set_value(
                    storage,
                    STORAGE_WINDOW_KEY,
                    &crate::WindowSettings::from_display(_window),
                );
            }
            if _app.persist_egui_memory() {
                epi::set_value(storage, STORAGE_EGUI_MEMORY_KEY, &*self.egui_ctx.memory());
            }
            _app.save(storage);
            storage.flush();
        }
    }
    */
}