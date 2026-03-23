mod app;
mod fractal_clock;

use app::FractalClockApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((400.0, 400.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        "Fractal Clock",
        native_options,
        Box::new(|_| Ok(Box::<FractalClockApp>::default())),
    )
}
