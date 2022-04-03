#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds


/* 
fn main() {
    let app = egui_app::TemplateApp::default();
    let native_options = epi::NativeOptions::default();
    egui_app::platform::run(Box::new(app), &native_options);
}
*/

use egui_app::{TemplateApp, TemplateApp2};

#[cfg(feature = "use_eframe")]
fn run<A: epi::App + 'static>(app: Box<A>, native_options: epi::NativeOptions) -> ! {
    eframe::run_native(app, native_options);
}

#[cfg(feature = "use_sdl2")]
fn run<A: epi::App>(app: Box<A>, native_options: epi::NativeOptions) -> ! {
    egui_app::sdl2_platform::run_native(app, native_options);
}

#[cfg(feature = "default")]
fn run<A: epi::App>(app: Box<A>, native_options: epi::NativeOptions) {
    egui_app::iv_platform::run_native(app, native_options);
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    println!("args: {:?}", args);
    if args.len() > 1 {
        match args[1].as_str() {
            "app0" => run( Box::new(MyApp::default()), epi::NativeOptions::default()),
            "app1" => run( Box::new(TemplateApp::default()), epi::NativeOptions::default()),
            "app2" => run( Box::new(TemplateApp2::new()), epi::NativeOptions::default()),
            _ => run(Box::new(MyApp::default()), epi::NativeOptions::default()),
        }
    } else {
        run(Box::new(MyApp::default()), epi::NativeOptions::default())
    }
}

#[derive(Debug)]
struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {



        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }

    fn name(&self) -> &str {
        "my app"
    }
}