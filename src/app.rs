// Copied from: https://github.com/emilk/egui/blob/0.33.0/crates/egui_demo_app/src/wrap_app.rs
#[derive(Default)]
pub struct FractalClockApp {
    fractal_clock: crate::fractal_clock::FractalClock,
    pub mock_time: Option<f64>,
}

impl eframe::App for FractalClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::dark_canvas(&ctx.style())
                    .stroke(egui::Stroke::NONE)
                    .corner_radius(0),
            )
            .show(ctx, |ui| {
                self.fractal_clock
                    .ui(ui, self.mock_time.or(Some(seconds_since_midnight())));
            });
    }
}

/// Time of day as seconds since midnight. Used for clock in demo app.
fn seconds_since_midnight() -> f64 {
// Copied from https://github.com/emilk/egui/blob/0.33.0/crates/egui_demo_app/src/lib.rs
    use chrono::Timelike as _;
    let time = chrono::Local::now().time();
    time.num_seconds_from_midnight() as f64 + 1e-9 * (time.nanosecond() as f64)
}
