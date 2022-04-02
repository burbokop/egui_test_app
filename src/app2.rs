use std::{iter::FromIterator};

use egui::{FontData, WidgetText, TextStyle, FontDefinitions, Label, Layout, Hyperlink, Separator, TopBottomPanel, Button};
use epaint::{Color32, FontFamily};

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

pub struct Headlines {
    articles: Vec<NewsCardData>,
}

struct NewsCardData {
    title: String,
    desc: String,
    url: String,
}

impl Headlines {
    pub fn new() -> Headlines {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title{}", a),
            desc: format!("desc{}", a),
            url: format!("https://example.com/{}", a),
        });
        Headlines {
            articles: Vec::from_iter(iter),
        }
    }

    pub fn configure_fonts(&self, ctx: &egui::Context) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "MesloLGS".to_string(),
            FontData::from_static(include_bytes!("MesloLGS_NF_Regular.ttf")),
        );
        //font_def.families.insert(
        //    eframe::egui::TextStyle::Heading,
        //    (FontFamily::Proportional, 35.),
        //);
        //font_def.family_and_size.insert(
        //    eframe::egui::TextStyle::Body,
        //    (FontFamily::Proportional, 20.),
        //);
        font_def
            .families
            .insert(FontFamily::Name("MesloLGS".into()), vec!["MesloLGS".to_string()]);

        ctx.set_fonts(font_def);
    }

    pub fn render_news_cards(&self, ui: &mut egui::Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);
            // render title
            let title = format!("▶ {}", a.title);
            ui.colored_label(WHITE, title);
            // render desc
            ui.add_space(PADDING);
            let desc = Label::new(&a.desc); //text_style(eframe::egui::TextStyle::Button);
            ui.add(desc);

            // render hyperlinks
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(), |ui| {
                ui.add(Hyperlink::from_label_and_url("read more ⤴", &a.url));
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }

    pub(crate) fn render_top_panel(&self, ctx: &egui::Context) {
        // define a TopBottomPanel widget
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                // logo
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(Label::new(egui::WidgetText::from("📓").text_style(egui::TextStyle::Heading)));
                });
                // controls
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let close_btn = ui.add(Button::new(egui::WidgetText::from("❌").text_style(egui::TextStyle::Body)));
                    let refresh_btn = ui.add(Button::new(egui::WidgetText::from("🔄").text_style(egui::TextStyle::Body)));
                    let theme_btn = ui.add(Button::new(egui::WidgetText::from("🌙").text_style(egui::TextStyle::Body)));
                });
            });
            ui.add_space(10.);
        });
    }
}

impl epi::App for Headlines {
    fn setup(
        &mut self, 
        ctx: &egui::Context, 
        frame: &epi::Frame, 
        storage: Option<&dyn epi::Storage>
    ) {
        self.configure_fonts(ctx);
    }
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        self.render_top_panel(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            egui::ScrollArea::vertical()
                .stick_to_right()
                .stick_to_bottom()
                .show(ui, |ui| {
                self.render_news_cards(ui);
            });
            render_footer(ctx);
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

fn render_footer(ctx: &egui::Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            ui.add(Label::new("API source: newsapi.org"));
            ui.add(
                Hyperlink::from_label_and_url(WidgetText::from("Made with egui").text_style(TextStyle::Monospace), "https://github.com/emilk/egui")
            );
            ui.add(
                egui::Hyperlink::from_label_and_url(WidgetText::from("creativcoder/headlines").text_style(TextStyle::Monospace), "https://github.com/creativcoder/headlines")
            );
            ui.add_space(10.);
        })
    });
}

fn render_header(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("headlines");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}
