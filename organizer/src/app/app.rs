use eframe::egui;
use crate::scanner::search_main_folders;

pub struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
         
        }
    }
}

impl eframe::App for MyApp {
    // This function is called each time the UI needs repainting, which may be many times per second.
    // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`,
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Welcome to My App");
            
            // Text Input
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            
            // Counter/Slider   
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            
            // Interactive Button
            if ui.button("Click Me").clicked() {
                self.age += 1;
            }

            ui.label(format!(""));
        });
    }
}