#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;  
use crate::egui::*;
  
struct CalcApp {}  
  
impl CalcApp {  
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {  
        CalcApp {}  
    }  
}  
  
impl eframe::App for CalcApp {  

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {         
            // Отображение контейнера с пунктирной границей:
            egui::CentralPanel::default().show(ctx, |ui| {
                // Создайте контейнер с пунктирной границей:
                let container = egui::containers::Frame::none()
                .fill(egui::Color32::TRANSPARENT) // Прозрачная заливка
                .stroke(egui::Stroke::new(1.0, Color32::BLACK)) // Пунктирная граница
                .outer_margin(egui::vec2(10.0, 20.0))
                .inner_margin(egui::vec2(10.0,20.0));
                
    
                // Добавьте виджеты внутри контейнера:
                container.show(ui, |ui| {
                    ui.label("This is a label");
                    ui.hyperlink("https://github.com/emilk/egui");
                    ui.label("Это контейнер с пунктирной границей");
                    // Добавьте другие виджеты по вашему выбору...
                });
            });
    }
     
}

fn main() -> eframe::Result<()>  {  

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(vec2(820.0, 740.0)), 
        ..Default::default()
    };

    eframe::run_native(  
        "Pixelation",  
        eframe::NativeOptions::default(),  
        Box::new(|cc| Box::new(CalcApp::new(cc))),  
    )  
    
}  