use epi::backend::{FrameData, RepaintSignal};

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
        Self { can_drag_window: true, egui_ctx: egui_ctx, quit: false, frame: frame, pending_full_output: Default::default() }
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


    pub fn on_event<A: epi::App>(&mut self, app: &mut A, event: &inkview::Event) -> bool {

        println!("event: {:?}", event);
        match event.event_type {
            inkview::EventType::Init => false,
            inkview::EventType::Exit => if app.on_exit_event() { 
                inkview::clear_on_exit(); 
                std::process::exit(0)
            } else { false },
            inkview::EventType::Show => false,
            inkview::EventType::Hide => todo!(),
            inkview::EventType::KeyPress => todo!(),
            inkview::EventType::KeyRelease => todo!(),
            inkview::EventType::KeyRepeat => todo!(),
            inkview::EventType::KeyPressExt => todo!(),
            inkview::EventType::KeyReleaseExt => todo!(),
            inkview::EventType::KeyRepeatExt => todo!(),
            inkview::EventType::PointerUp => todo!(),
            inkview::EventType::PointerDown => todo!(),
            inkview::EventType::PointerMove => todo!(),
            inkview::EventType::Scroll => todo!(),
            inkview::EventType::PointerLong => todo!(),
            inkview::EventType::PointerHold => todo!(),
            inkview::EventType::PointerDrag => todo!(),
            inkview::EventType::PointerCancel => todo!(),
            inkview::EventType::PointerChanged => todo!(),
            inkview::EventType::Orientation => todo!(),
            inkview::EventType::Focus => todo!(),
            inkview::EventType::Unfocus => todo!(),
            inkview::EventType::Activate => todo!(),
            inkview::EventType::MtSync => todo!(),
            inkview::EventType::TouchUp => todo!(),
            inkview::EventType::TouchDown => todo!(),
            inkview::EventType::TouchMove => todo!(),
            inkview::EventType::Repaint => todo!(),
            inkview::EventType::QnMove => todo!(),
            inkview::EventType::QnReleaseEASE => todo!(),
            inkview::EventType::QnBorder => todo!(),
            inkview::EventType::Snapshot => todo!(),
            inkview::EventType::Fsincoming => todo!(),
            inkview::EventType::Fschanged => todo!(),
            inkview::EventType::MpStatechanged => todo!(),
            inkview::EventType::MpTrackchanged => todo!(),
            inkview::EventType::Prevpage => todo!(),
            inkview::EventType::Nextpage => todo!(),
            inkview::EventType::Opendic => todo!(),
            inkview::EventType::ControlPanelAboutToOpen => todo!(),
            inkview::EventType::Update => todo!(),
            inkview::EventType::PanelBluetoothA2dp => todo!(),
            inkview::EventType::Tab => todo!(),
            inkview::EventType::Panel => todo!(),
            inkview::EventType::PanelIcon => todo!(),
            inkview::EventType::PanelText => todo!(),
            inkview::EventType::PanelProgress => todo!(),
            inkview::EventType::PanelMplayer => todo!(),
            inkview::EventType::PanelUsbdrive => todo!(),
            inkview::EventType::PanelNetwork => todo!(),
            inkview::EventType::PanelClock => todo!(),
            inkview::EventType::PanelBluetooth => todo!(),
            inkview::EventType::PanelTasklist => todo!(),
            inkview::EventType::PanelObreeySync => todo!(),
            inkview::EventType::PanelSetreadingmode => todo!(),
            inkview::EventType::PanelSetreadingmodeInvert => todo!(),
            inkview::EventType::PanelFrontLight => todo!(),
            inkview::EventType::Globalrequest => todo!(),
            inkview::EventType::Globalaction => todo!(),
            inkview::EventType::Foreground => todo!(),
            inkview::EventType::Background => todo!(),
            inkview::EventType::Subtaskclose => todo!(),
            inkview::EventType::Configchanged => todo!(),
            inkview::EventType::Savestate => todo!(),
            inkview::EventType::ObreeyConfigChanged => todo!(),
            inkview::EventType::Sdin => todo!(),
            inkview::EventType::Sdout => todo!(),
            inkview::EventType::UsbstoreIn => todo!(),
            inkview::EventType::UsbstoreOut => todo!(),
            inkview::EventType::BtRxcomplete => todo!(),
            inkview::EventType::BtTxcomplete => todo!(),
            inkview::EventType::SynthEnded => todo!(),
            inkview::EventType::DicClosedARD => todo!(),
            inkview::EventType::ShowKeyboard201 => todo!(),
            inkview::EventType::Textclear => todo!(),
            inkview::EventType::ExtKb => todo!(),
            inkview::EventType::Letter => todo!(),
            inkview::EventType::Callback => todo!(),
            inkview::EventType::Scanprogress => todo!(),
            inkview::EventType::Stopscan => todo!(),
            inkview::EventType::Startscan => todo!(),
            inkview::EventType::Scanstopped => todo!(),
            inkview::EventType::PostponeTimedPoweroff => todo!(),
            inkview::EventType::FrameActivated => todo!(),
            inkview::EventType::FrameDeactivated => todo!(),
            inkview::EventType::ReadProgressChanged => todo!(),
            inkview::EventType::DumpBitmapsDebugInfo => todo!(),
            inkview::EventType::NetConnected => todo!(),
            inkview::EventType::NetDisconnected => todo!(),
            inkview::EventType::NetFoundNewFw => todo!(),
            inkview::EventType::SynthPosition => todo!(),
            inkview::EventType::AsyncTaskFinished => todo!(),
            inkview::EventType::StopPlaying => todo!(),
            inkview::EventType::AvrcpCommand => todo!(),
            inkview::EventType::AudioChanged => todo!(),
            inkview::EventType::PackageJobChanged => todo!(),
            inkview::EventType::Custom => todo!(),
        }

        //self.egui_winit.on_event(&self.egui_ctx, event);
    }

    pub fn update(
        &mut self,
        app: &mut dyn epi::App,
    ) -> egui::FullOutput {
        //let frame_start = instant::Instant::now();


        let raw_input = egui::RawInput { 
            screen_rect: Some(egui::Rect::from_min_size(
                Default::default(), 
                emath::Vec2::new(inkview::screen_width().unwrap() as f32, inkview::screen_width().unwrap() as f32)
            )),            
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