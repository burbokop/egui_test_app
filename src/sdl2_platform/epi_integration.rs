use epaint::Pos2;
use epi::backend::{FrameData, RepaintSignal};


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
    pub fn new(storage: Option<Box<dyn epi::Storage>>, prefer_dark_mode: Option<bool>) -> Self {
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
                prefer_dark_mode: prefer_dark_mode,
                cpu_usage: None,
                native_pixels_per_point: Some(1.),
            },
            output: Default::default(),
            repaint_signal: std::sync::Arc::from(RS::default()),
        });

        if prefer_dark_mode == Some(true) {
            egui_ctx.set_visuals(egui::Visuals::dark());
        } else {
            egui_ctx.set_visuals(egui::Visuals::light());
        }
 

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

    pub fn convert_event_to_app<A: epi::App>(&mut self, app: &mut A, event: &sdl2::event::Event) -> Option<egui::Event> {
        match event {
            sdl2::event::Event::Quit { timestamp } => { if app.on_exit_event() { std::process::exit(0) } None },
            sdl2::event::Event::AppTerminating { timestamp } => todo!(),
            sdl2::event::Event::AppLowMemory { timestamp } => todo!(),
            sdl2::event::Event::AppWillEnterBackground { timestamp } => todo!(),
            sdl2::event::Event::AppDidEnterBackground { timestamp } => todo!(),
            sdl2::event::Event::AppWillEnterForeground { timestamp } => todo!(),
            sdl2::event::Event::AppDidEnterForeground { timestamp } => todo!(),
            sdl2::event::Event::Display { timestamp, display_index, display_event } => todo!(),
            sdl2::event::Event::Window { timestamp, window_id, win_event } => None,
            sdl2::event::Event::KeyDown { timestamp, window_id, keycode, scancode, keymod, repeat } => Some(egui::Event::KeyDown {}),
            sdl2::event::Event::KeyUp { timestamp, window_id, keycode, scancode, keymod, repeat } => todo!(),
            sdl2::event::Event::TextEditing { timestamp, window_id, text, start, length } => todo!(),
            sdl2::event::Event::TextInput { timestamp, window_id, text } => todo!(),
            sdl2::event::Event::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel } => {
                Some(egui::Event::PointerMoved(Pos2::new(*x as f32, *y as f32)))
            },
            sdl2::event::Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                Some(egui::Event::PointerButton {
                    pos: Pos2::new(*x as f32, *y as f32),
                    button: match mouse_btn {
                        sdl2::mouse::MouseButton::Left => egui::PointerButton::Primary,
                        sdl2::mouse::MouseButton::Middle => egui::PointerButton::Middle,
                        sdl2::mouse::MouseButton::Right => egui::PointerButton::Secondary,
                        _ => egui::PointerButton::Primary,
                    },
                    pressed: true,
                    modifiers: egui::Modifiers::default(),
                })
            },
            sdl2::event::Event::MouseButtonUp { timestamp, window_id, which, mouse_btn, clicks, x, y } => {
                Some(egui::Event::PointerButton {
                    pos: Pos2::new(*x as f32, *y as f32),
                    button: match mouse_btn {
                        sdl2::mouse::MouseButton::Left => egui::PointerButton::Primary,
                        sdl2::mouse::MouseButton::Middle => egui::PointerButton::Middle,
                        sdl2::mouse::MouseButton::Right => egui::PointerButton::Secondary,
                        _ => egui::PointerButton::Primary,
                    },
                    pressed: false,
                    modifiers: egui::Modifiers::default(),
                })
            },
            sdl2::event::Event::MouseWheel { timestamp, window_id, which, x, y, direction } => todo!(),
            sdl2::event::Event::JoyAxisMotion { timestamp, which, axis_idx, value } => todo!(),
            sdl2::event::Event::JoyBallMotion { timestamp, which, ball_idx, xrel, yrel } => todo!(),
            sdl2::event::Event::JoyHatMotion { timestamp, which, hat_idx, state } => todo!(),
            sdl2::event::Event::JoyButtonDown { timestamp, which, button_idx } => todo!(),
            sdl2::event::Event::JoyButtonUp { timestamp, which, button_idx } => todo!(),
            sdl2::event::Event::JoyDeviceAdded { timestamp, which } => todo!(),
            sdl2::event::Event::JoyDeviceRemoved { timestamp, which } => todo!(),
            sdl2::event::Event::ControllerAxisMotion { timestamp, which, axis, value } => todo!(),
            sdl2::event::Event::ControllerButtonDown { timestamp, which, button } => todo!(),
            sdl2::event::Event::ControllerButtonUp { timestamp, which, button } => todo!(),
            sdl2::event::Event::ControllerDeviceAdded { timestamp, which } => todo!(),
            sdl2::event::Event::ControllerDeviceRemoved { timestamp, which } => todo!(),
            sdl2::event::Event::ControllerDeviceRemapped { timestamp, which } => todo!(),
            sdl2::event::Event::FingerDown { timestamp, touch_id, finger_id, x, y, dx, dy, pressure } => todo!(),
            sdl2::event::Event::FingerUp { timestamp, touch_id, finger_id, x, y, dx, dy, pressure } => todo!(),
            sdl2::event::Event::FingerMotion { timestamp, touch_id, finger_id, x, y, dx, dy, pressure } => todo!(),
            sdl2::event::Event::DollarGesture { timestamp, touch_id, gesture_id, num_fingers, error, x, y } => todo!(),
            sdl2::event::Event::DollarRecord { timestamp, touch_id, gesture_id, num_fingers, error, x, y } => todo!(),
            sdl2::event::Event::MultiGesture { timestamp, touch_id, d_theta, d_dist, x, y, num_fingers } => todo!(),
            sdl2::event::Event::ClipboardUpdate { timestamp } => todo!(),
            sdl2::event::Event::DropFile { timestamp, window_id, filename } => todo!(),
            sdl2::event::Event::DropText { timestamp, window_id, filename } => todo!(),
            sdl2::event::Event::DropBegin { timestamp, window_id } => todo!(),
            sdl2::event::Event::DropComplete { timestamp, window_id } => todo!(),
            sdl2::event::Event::AudioDeviceAdded { timestamp, which, iscapture } => todo!(),
            sdl2::event::Event::AudioDeviceRemoved { timestamp, which, iscapture } => todo!(),
            sdl2::event::Event::RenderTargetsReset { timestamp } => todo!(),
            sdl2::event::Event::RenderDeviceReset { timestamp } => todo!(),
            sdl2::event::Event::User { timestamp, window_id, type_, code, data1, data2 } => todo!(),
            sdl2::event::Event::Unknown { timestamp, type_ } => todo!(),
        }
    }
    
    pub fn on_event<A: epi::App>(&mut self, app: &mut A, event: &sdl2::event::Event) -> Option<egui::Event> {
        self.convert_event_to_app(app, event)
    }

    pub fn update(
        &mut self,
        app: &mut dyn epi::App,
        w: u32,
        h: u32,
        events: Vec<egui::Event>
    ) -> egui::FullOutput {
        //let frame_start = instant::Instant::now();


        let raw_input = egui::RawInput { 
            screen_rect: Some(egui::Rect::from_min_size(
                Default::default(), 
                emath::Vec2::new(w as f32, h as f32)
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