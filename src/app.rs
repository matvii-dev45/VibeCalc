use eframe::egui;
use crate::utils::config::Config;

pub struct CalculatorApp {
    input: String,
    result: String,
    config: Config,
}

impl CalculatorApp {
    pub fn new(config: Config) -> Self {
        Self {
            input: String::new(),
            result: String::from("0"),
            config,
        }
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visuals = egui::Visuals::dark();
        
        if let Ok(color) = parse_hex_color(&self.config.background_color) {
            visuals.extreme_bg_color = color;
        }
        
        ctx.set_visuals(visuals);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("VibeCalc");

            ui.style_mut().text_styles.insert(
                egui::TextStyle::Body,
                egui::FontId::proportional(self.config.font_size as f32),
            );
            
            ui.text_edit_singleline(&mut self.input);

            let button = egui::Button::new("=");
            if ui.add(button).clicked() {
                match eval::eval(&self.input) {
                    Ok(value) => self.result = value.to_string(),
                    Err(e) => self.result = format!("Error: {}", e),
                }
            }

            ui.label(format!("Result: {}", self.result));
        });
    }
}

fn parse_hex_color(hex: &str) -> Result<egui::Color32, ()> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err(());
    }
    
    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ())?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ())?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ())?;
    
    Ok(egui::Color32::from_rgb(r, g, b))
}

pub fn run(config: Config) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config.window_width as f32, config.window_height as f32]),
        ..Default::default()
    };
    
    eframe::run_native(
        "VibeCalc",
        options,
        Box::new(|_cc| Ok(Box::new(CalculatorApp::new(config)))),
    )
}