mod translationhash;
mod translationloader;
use crate::translationhash::TranslationHash;
use crate::translationloader::TranslationLoader;
use eframe::egui;
use egui_plot::{Plot, PlotPoints, Line, Legend};
use image;
use std::time::Instant;

pub struct TranslatorApp {
    //user input variable is here
    input_text: String,
    // this is the output of the translation
    output_text: String,
    //counts num for x axis
    translation_count: usize,
    //this is where the data for translation times will be stored for the graph comparisons
    tree_times: Vec<[f64; 2]>,
    hashmap_times: Vec<[f64; 2]>,
    //vars for flags above input/output
    english_flag: Option<egui::TextureHandle>,
    spanish_flag: Option<egui::TextureHandle>,
    translation_hash: TranslationHash,
}
impl Default for TranslatorApp {
    fn default() -> Self {
        let loader = TranslationLoader { 
            path: "data/esdatabase.csv".to_string(), 
            count: 0, 
        }; 
        let hash = loader.load();
        Self {
            input_text: String::new(),
            output_text: String::new(),
            translation_count: 0,
            tree_times: Vec::new(),
            hashmap_times: Vec::new(),
            english_flag: None,
            spanish_flag: None,
            translation_hash: hash,
        }
    }
}
//this is where the icon for desktop is loaded
fn load_icon() -> Option<egui::IconData> {
    let image = image::open("assets/icon.png").ok()?.to_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    Some(egui::IconData { rgba, width, height })
}
//flags are loaded here
fn load_flag(ctx: &egui::Context, path: &str) -> Option<egui::TextureHandle> {
    let image = image::open(path).ok()?.to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let pixels = image.into_raw();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
    Some(ctx.load_texture(path.to_string(), color_image, Default::default()))
}
impl eframe::App for TranslatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.english_flag.is_none() {
            self.english_flag = load_flag(ctx, "assets/englishflag.png");
        }
        if self.spanish_flag.is_none() {
            self.spanish_flag = load_flag(ctx, "assets/spanishflag.png");
        }
        let english_flag = self.english_flag.as_ref().unwrap();
        let spanish_flag = self.spanish_flag.as_ref().unwrap();
        egui::CentralPanel::default().show(ctx, |ui| {
            //allows for scrolling, on my computers it looks fine but i think we need it just in case anyone has a smaller screen
            egui::ScrollArea::vertical().show(ui, |ui| {
                //ascii image for the logo, can be changed if we need to i just made it this way for fun
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(
                            r#"
    _____
  .-'.  ':'-.
.''::: .:    '.
/   :::::'      \
;.    ':' `       ;
|       '..       |
; '      ::::.    ;
 \       '::::   /
  '.      :::  .'
    '-.___'_.-'
"#,
                        )
                        .monospace()
                        .color(egui::Color32::WHITE),
                    );
                });
                ui.add_space(10.0);
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                    ui.heading(
                        egui::RichText::new("English to Spanish Translator")
                            .strong()
                            .size(28.0)
                            .color(egui::Color32::WHITE),
                    );
                });
                ui.add_space(20.0);
                let content_width = 640.0;
                //centers and alligns everything
                let left_padding = (ui.available_width() - content_width).max(0.0) / 2.0;
                ui.horizontal(|ui| {
                    ui.add_space(left_padding);
                    ui.group(|ui| {
                        ui.set_width(content_width);
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.image((english_flag.id(), egui::vec2(24.0, 24.0)));
                                    ui.label("English");
                                });
                                ui.label("Enter Text Here:");
                                egui::Frame::default()
                                    .stroke(egui::Stroke::new(2.0, ui.visuals().widgets.noninteractive.bg_stroke.color))
                                    .show(ui, |ui| {
                                        let response = ui.add_sized(
                                            [300.0, 100.0],
                                            egui::TextEdit::multiline(&mut self.input_text),
                                        );
                                        if response.changed() {
                                            //this makes sure text is limited to 25 chars so our program wont crash bc of too many inputs
                                            //we can change this to whatever i just put 25 as a starter
                                            self.input_text = self.input_text.replace('\n', "");
                                            if self.input_text.chars().count() > 25 {
                                                self.input_text = self.input_text.chars().take(25).collect();
                                            }
                                        }
                                    });
                                ui.label(
                                    //displays char count
                                    egui::RichText::new(format!("{} / 25", self.input_text.chars().count()))
                                        .small()
                                        .color(egui::Color32::GRAY),
                                );
                                ui.add_space(6.0);
                                if ui.add_sized([300.0, 34.0], egui::Button::new("Translate")).clicked() {
                                    let start = Instant::now(); 
                                    let result = self.translation_hash.at(&self.input_text);
                                    let hash_time = start.elapsed().as_secs_f64() * 1000.0; 
                                    self.output_text = match result { 
                                        Some(t) => t, 
                                        None => "Translation not found.".to_string(), 
                                    }; 
                                    //random time still for tree time just to test
                                    let tree_time = hash_time * 1.5;
                                    let t = (self.translation_count + 1) as f64;
                                    self.tree_times.push([t, tree_time]); 
                                    self.hashmap_times.push([t, hash_time]); 
                                    self.translation_count += 1; 
                                }
                            });
                            ui.add_space(20.0);
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.image((spanish_flag.id(), egui::vec2(24.0, 24.0)));
                                    ui.label("Spanish");
                                });
                                ui.label("Output Text Here:");
                                egui::Frame::default()
                                    .stroke(egui::Stroke::new(2.0, ui.visuals().widgets.noninteractive.bg_stroke.color))
                                    .fill(ui.visuals().extreme_bg_color)
                                    .show(ui, |ui| {
                                        ui.add_sized(
                                            [300.0, 100.0],
                                            egui::TextEdit::multiline(&mut self.output_text)
                                                .interactive(false)
                                                .frame(false),
                                        );
                                    });
                                ui.add_space(19.0);
                                //resets the input and output text, easier than deleting all
                                if ui.add_sized([300.0, 34.0], egui::Button::new("Reset")).clicked() {
                                    self.input_text.clear();
                                    self.output_text.clear();
                                }
                            });
                        });
                    });
                });
                ui.add_space(20.0);
                //dif lines for each data struc with dif colors, easy to compare visually, i thought it would be better than just text
                let tree_line = Line::new("26-ary Tree", PlotPoints::from(self.tree_times.clone()));
                let hashmap_line = Line::new("HashMap", PlotPoints::from(self.hashmap_times.clone()));
                let graph_width = 620.0;
                let left_padding = (ui.available_width() - graph_width).max(0.0) / 2.0;
                ui.horizontal(|ui| {
                    ui.add_space(left_padding);
                    ui.allocate_ui(egui::vec2(graph_width, 300.0), |ui| {
                        egui::Frame::group(ui.style())
                            .fill(ui.visuals().widgets.noninteractive.bg_fill)
                            .stroke(egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color))
                            .inner_margin(10.0)
                            .show(ui, |ui| {
                                Plot::new("translation_benchmark")
                                //this is where graph is generated, might need to change the y axis range depending on our actual times
                                    .legend(Legend::default())
                                    .x_axis_label("Translation #")
                                    .y_axis_label("Time (ms)")
                                    .include_x(1.0)
                                    .include_x(10.0)
                                    .include_y(0.0)
                                    .include_y(0.5)
                                    .show(ui, |plot_ui| {
                                        plot_ui.line(tree_line);
                                        plot_ui.line(hashmap_line);
                                    });
                            });
                    });
                });
            });
        });
    }
}

//run this main to test or go to project directory in terminal and run "cargo run" both work
fn main() -> Result<(), eframe::Error> {
    let mut options = eframe::NativeOptions::default();
    options.viewport.min_inner_size = Some(egui::vec2(400.0, 500.0));
    if let Some(icon) = load_icon() {
        options.viewport.icon = Some(icon.into());
    }
    eframe::run_native(
        "English to Spanish Translator",
        options,
        Box::new(|_cc| Ok(Box::new(TranslatorApp::default()))),
    )
}
