// to use organizer\src\app.rs
mod app;
mod scanner;
use app::MyApp;


fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My First Desktop App",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}
