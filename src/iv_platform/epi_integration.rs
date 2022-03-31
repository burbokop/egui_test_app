use egui::Event;
use epaint::{Vec2, Pos2};
use epi::backend::{FrameData, RepaintSignal};
use queues::{Queue, IsQueue};

use super::inkview;

pub struct EpiIntegration {
    pub frame: epi::Frame,
    //last_auto_save: instant::Instant,
    pub egui_ctx: egui::Context,
    pending_full_output: egui::FullOutput,
    //egui_winit: crate::State,
    /// When set, it is time to quit
    quit: bool,
    can_drag_window: bool,

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
    pub fn new(storage: Option<Box<dyn epi::Storage>>) -> Self {
        let egui_ctx = egui::Context::default();

        //*egui_ctx.memory() = load_egui_memory(storage.as_deref()).unwrap_or_default();

        //let prefer_dark_mode = prefer_dark_mode();

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
                native_pixels_per_point: Some(1.),
            },
            output: Default::default(),
            repaint_signal: std::sync::Arc::from(RS::default()),
        });
 

/*
        if prefer_dark_mode == Some(true) {
            egui_ctx.set_visuals(egui::Visuals::dark());
        } else {
            egui_ctx.set_visuals(egui::Visuals::light());
        }

        Self {
            frame,
            last_auto_save: instant::Instant::now(),
            egui_ctx,
            egui_winit: crate::State::new(max_texture_side, window),
            pending_full_output: Default::default(),
            quit: false,
            can_drag_window: false,
        }
         */
        Self { can_drag_window: true, egui_ctx: egui_ctx, quit: false, frame: frame, pending_full_output: Default::default(), event_q: Default::default() }
    }

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

    pub fn convert_event_to_app<A: epi::App>(&mut self, app: &mut A, event: &inkview::Event) -> Option<egui::Event> {

        println!("event: {:?}", event);
        match event {
            inkview::Event::Init => None,
            inkview::Event::Exit => if app.on_exit_event() { 
                inkview::clear_on_exit(); 
                std::process::exit(0)
            } else { None },
            inkview::Event::Show => None,
            inkview::Event::Hide => None,
            inkview::Event::KeyPress => todo!(),
            inkview::Event::KeyRelease => todo!(),
            inkview::Event::KeyRepeat => todo!(),
            inkview::Event::KeyPressExt => todo!(),
            inkview::Event::KeyReleaseExt => todo!(),
            inkview::Event::KeyRepeatExt => todo!(),
            inkview::Event::PointerUp { pos } => Some(egui::Event::Touch {
                device_id: egui::TouchDeviceId(0),
                id: egui::TouchId(0),
                phase: egui::TouchPhase::End,
                pos: Pos2::new(pos.x as f32, pos.y as f32),
                force: 1.,
            }),
            inkview::Event::PointerDown { pos } => Some(egui::Event::Touch {
                device_id: egui::TouchDeviceId(0),
                id: egui::TouchId(0),
                phase: egui::TouchPhase::Start,
                pos: Pos2::new(pos.x as f32, pos.y as f32),
                force: 1.,
            }),
            inkview::Event::PointerMove { pos } => Some(egui::Event::PointerMoved(Pos2::new(pos.x as f32, pos.y as f32))),
            inkview::Event::Scroll => todo!(),
            inkview::Event::PointerLong { pos } => { println!("\tlong: {:?}", pos); None },
            inkview::Event::PointerHold { pos } => { println!("\thold: {:?}", pos); None },
            inkview::Event::PointerDrag { pos } => { println!("\tdrag: {:?}", pos); None },
            inkview::Event::PointerCancel { pos } => { println!("\tcancel: {:?}", pos); None },
            inkview::Event::PointerChanged { pos } => { println!("\tchanged: {:?}", pos); None },
            inkview::Event::Orientation => todo!(),
            inkview::Event::Focus => todo!(),
            inkview::Event::Unfocus => todo!(),
            inkview::Event::Activate => todo!(),
            inkview::Event::MtSync => None,
            inkview::Event::TouchUp => todo!(),
            inkview::Event::TouchDown => todo!(),
            inkview::Event::TouchMove => todo!(),
            inkview::Event::Repaint => todo!(),
            inkview::Event::QnMove => todo!(),
            inkview::Event::QnReleaseEASE => todo!(),
            inkview::Event::QnBorder => todo!(),
            inkview::Event::Snapshot => todo!(),
            inkview::Event::Fsincoming => todo!(),
            inkview::Event::Fschanged => todo!(),
            inkview::Event::MpStatechanged => todo!(),
            inkview::Event::MpTrackchanged => todo!(),
            inkview::Event::Prevpage => todo!(),
            inkview::Event::Nextpage => todo!(),
            inkview::Event::Opendic => todo!(),
            inkview::Event::ControlPanelAboutToOpen => todo!(),
            inkview::Event::Update => todo!(),
            inkview::Event::PanelBluetoothA2dp => todo!(),
            inkview::Event::Tab => todo!(),
            inkview::Event::Panel => todo!(),
            inkview::Event::PanelIcon => todo!(),
            inkview::Event::PanelText => todo!(),
            inkview::Event::PanelProgress => todo!(),
            inkview::Event::PanelMplayer => todo!(),
            inkview::Event::PanelUsbdrive => todo!(),
            inkview::Event::PanelNetwork => todo!(),
            inkview::Event::PanelClock => todo!(),
            inkview::Event::PanelBluetooth => todo!(),
            inkview::Event::PanelTasklist => todo!(),
            inkview::Event::PanelObreeySync => todo!(),
            inkview::Event::PanelSetreadingmode => todo!(),
            inkview::Event::PanelSetreadingmodeInvert => todo!(),
            inkview::Event::PanelFrontLight => todo!(),
            inkview::Event::GlobalRequest => todo!(),
            inkview::Event::GlobalAction => todo!(),
            inkview::Event::Foreground => todo!(),
            inkview::Event::Background => todo!(),
            inkview::Event::SubTaskClose => todo!(),
            inkview::Event::ConfigChanged => todo!(),
            inkview::Event::SaveState => todo!(),
            inkview::Event::ObreeyConfigChanged => todo!(),
            inkview::Event::Sdin => todo!(),
            inkview::Event::Sdout => todo!(),
            inkview::Event::UsbStoreIn => todo!(),
            inkview::Event::UsbStoreOut => todo!(),
            inkview::Event::BtRxComplete => todo!(),
            inkview::Event::BtTxComplete => todo!(),
            inkview::Event::SynthEnded => todo!(),
            inkview::Event::DicClosedARD => todo!(),
            inkview::Event::ShowKeyboard => todo!(),
            inkview::Event::TextClear => todo!(),
            inkview::Event::ExtKb => todo!(),
            inkview::Event::Letter => todo!(),
            inkview::Event::Callback => todo!(),
            inkview::Event::ScanProgress => todo!(),
            inkview::Event::StopScan => todo!(),
            inkview::Event::StartScan => todo!(),
            inkview::Event::ScanStopped => todo!(),
            inkview::Event::PostponeTimedPowerOff => todo!(),
            inkview::Event::FrameActivated => todo!(),
            inkview::Event::FrameDeactivated => todo!(),
            inkview::Event::ReadProgressChanged => todo!(),
            inkview::Event::DumpBitmapsDebugInfo => todo!(),
            inkview::Event::NetConnected => todo!(),
            inkview::Event::NetDisconnected => todo!(),
            inkview::Event::NetFoundNewFw => todo!(),
            inkview::Event::SynthPosition => todo!(),
            inkview::Event::AsyncTaskFinished => todo!(),
            inkview::Event::StopPlaying => todo!(),
            inkview::Event::AvrcpCommand => todo!(),
            inkview::Event::AudioChanged => todo!(),
            inkview::Event::PackageJobChanged => todo!(),
            inkview::Event::Custom => todo!(),
        }
    }


    pub fn on_event<A: epi::App>(&mut self, app: &mut A, event: &inkview::Event) -> bool {

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
        //let frame_start = instant::Instant::now();


        let mut events: Vec<egui::Event> = Vec::with_capacity(self.event_q.size());
        while self.event_q.size() > 0 {
            events.push(self.event_q.peek().unwrap());
            self.event_q.remove().unwrap();
        }

        
        let raw_input = egui::RawInput { 
            screen_rect: Some(egui::Rect::from_min_size(
                Default::default(), 
                emath::Vec2::new(inkview::screen_width().unwrap() as f32, inkview::screen_width().unwrap() as f32)
            )),            
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

        //let frame_time = (instant::Instant::now() - frame_start).as_secs_f64() as f32;
        //self.frame.info.cpu_usage = Some(frame_time);

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