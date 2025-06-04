use eframe::{egui, App, CreationContext, Frame};
use colorful::{Color, Colorful};
use egui_code_editor::{Syntax, Token, TokenType};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "tabular",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

fn color(token: TokenType) -> Color {
    match token {
        TokenType::Comment(_) => Color::Grey37,
        TokenType::Function => Color::Yellow3b,
        TokenType::Keyword => Color::IndianRed1c,
        TokenType::Literal => Color::NavajoWhite1,
        TokenType::Numeric(_) => Color::MediumPurple,
        TokenType::Punctuation(_) => Color::Orange3,
        TokenType::Special => Color::Cyan,
        TokenType::Str(_) => Color::Green,
        TokenType::Type => Color::GreenYellow,
        TokenType::Whitespace(_) => Color::White,
        TokenType::Unknown => Color::Pink1,
        TokenType::Hyperlink => todo!(),
    }
}


#[derive(Default)]
struct MyApp {
    editor_text: String,
    selected_menu: String,
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     ui.label("SQL Editor");
        // });

        egui::SidePanel::left("sidebar")
            .resizable(true)
            .default_width(150.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Items").clicked() {
                        self.selected_menu = "Items".to_string();
                    }
                    if ui.button("Queries").clicked() {
                        self.selected_menu = "Queries".to_string();
                    }
                    if ui.button("History").clicked() {
                        self.selected_menu = "History".to_string();
                    }
                });

                ui.separator();

                // show tree view or list of items
                ui.label("Items List");
                ui.label("Query List");
                ui.label("History List");
                

            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.editor_text)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_rows(20),
            );
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.label(format!("Selected Menu: {}", self.selected_menu));
        });
    }
}
