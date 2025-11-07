use eframe::egui;
use crate::utils::config::Config;
use std::sync::Arc;

pub struct CalculatorApp {
    input: String,
    result: String,
    config: Config,
}

impl CalculatorApp {
    pub fn new(config: Config, cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(font_path) = &config.font_path {
            if let Ok(font_data) = std::fs::read(font_path) {
                let mut fonts = egui::FontDefinitions::default();
                fonts.font_data.insert(
                    "custom_font".to_owned(),
                    Arc::new(egui::FontData::from_owned(font_data)),
                );
                
                fonts.families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, "custom_font".to_owned());
                
                fonts.families
                    .entry(egui::FontFamily::Monospace)
                    .or_default()
                    .insert(0, "custom_font".to_owned());
                
                cc.egui_ctx.set_fonts(fonts);
            }
        }
        
        Self {
            input: String::new(),
            result: String::from("0"),
            config,
        }
    }

    fn add_to_input(&mut self, text: &str) {
        self.input.push_str(text);
    }

    fn clear_input(&mut self) {
        self.input.clear();
        self.result = String::from("0");
    }

    fn backspace(&mut self) {
        self.input.pop();
    }

    fn calculate(&mut self) {
        let mut expression = self.input.clone();

        expression = expression.replace("√(", "sqrt(");
        expression = expression.replace("²", "^2");

        let mut result = String::new();
        let mut in_abs = false;
        for ch in expression.chars() {
            if ch == '|' {
                if in_abs {
                    result.push(')');
                    in_abs = false;
                } else {
                    result.push_str("abs(");
                    in_abs = true;
                }
            } else {
                result.push(ch);
            }
        }
        expression = result;

        match meval::eval_str(&expression) {
            Ok(value) => self.result = value.to_string(),
            Err(e) => self.result = format!("Error: {}", e),
        }
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visuals = egui::Visuals::dark();
        
        if let Ok(color) = parse_hex_color(&self.config.background_color) {
            visuals.extreme_bg_color = color;
            visuals.panel_fill = color;
        }
        
        ctx.set_visuals(visuals);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading("VibeCalc");
                ui.add_space(10.0);
            });

            ui.group(|ui| {
                ui.set_min_height(80.0);
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(&self.input)
                        .size(self.config.font_size as f32)
                        .color(egui::Color32::WHITE));
                    ui.separator();
                    ui.label(egui::RichText::new(&self.result)
                        .size((self.config.font_size + 4) as f32)
                        .strong()
                        .color(egui::Color32::from_rgb(97, 175, 239)));
                });
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                let button_size = egui::vec2(
                    (ui.available_width() - 30.0) / 4.0,
                    40.0
                );

                if create_button(ui, "C", button_size, &self.config).clicked() {
                    self.clear_input();
                }
                if create_button(ui, "DEL", button_size, &self.config).clicked() {
                    self.backspace();
                }
                if create_button(ui, "(", button_size, &self.config).clicked() {
                    self.add_to_input("(");
                }
                if create_button(ui, ")", button_size, &self.config).clicked() {
                    self.add_to_input(")");
                }
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                let button_size = egui::vec2(
                    (ui.available_width() - 30.0) / 4.0,
                    45.0
                );

                if create_button(ui, "√", button_size, &self.config).clicked() {
                    self.add_to_input("√(");
                }
                if create_button(ui, "^", button_size, &self.config).clicked() {
                    self.add_to_input("^");
                }
                if create_button(ui, "²", button_size, &self.config).clicked() {
                    self.add_to_input("²");
                }
                if create_button(ui, "|x|", button_size, &self.config).clicked() {
                    self.add_to_input("||");
                }
            });

            ui.add_space(5.0);

            let rows = [
                ["7", "8", "9", "÷"],
                ["4", "5", "6", "×"],
                ["1", "2", "3", "-"],
                ["0", ".", "=", "+"],
            ];

            for row in rows.iter() {
                ui.horizontal(|ui| {
                    let button_size = egui::vec2(
                        (ui.available_width() - 30.0) / 4.0,
                        50.0
                    );

                    for &text in row {
                        let btn = create_button(ui, text, button_size, &self.config);
                        
                        if btn.clicked() {
                            match text {
                                "=" => self.calculate(),
                                "÷" => self.add_to_input("/"),
                                "×" => self.add_to_input("*"),
                                _ => self.add_to_input(text),
                            }
                        }
                    }
                });
                ui.add_space(5.0);
            }
        });
    }
}

fn create_button(ui: &mut egui::Ui, text: &str, size: egui::Vec2, config: &Config) -> egui::Response {
    let button_color = parse_hex_color(&config.button_color).unwrap_or(egui::Color32::from_rgb(97, 175, 239));
    let hover_color = parse_hex_color(&config.button_hover_color).unwrap_or(egui::Color32::from_rgb(82, 139, 189));
    let pressed_color = parse_hex_color(&config.button_pressed_color).unwrap_or(egui::Color32::from_rgb(58, 110, 165));

    let button = egui::Button::new(
        egui::RichText::new(text)
            .size(config.font_size as f32)
            .strong()
    )
    .min_size(size);

    let response = ui.add(button);

    let color = if response.is_pointer_button_down_on() {
        pressed_color
    } else if response.hovered() {
        hover_color
    } else {
        button_color
    };
    
    ui.painter().rect_filled(
        response.rect,
        3.0,
        color,
    );

    ui.painter().text(
        response.rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(config.font_size as f32),
        egui::Color32::WHITE,
    );

    response
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
            .with_inner_size([config.window_width as f32, config.window_height as f32])
            .with_resizable(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "VibeCalc",
        options,
        Box::new(move |cc| Ok(Box::new(CalculatorApp::new(config, cc)))),
    )
}