use std::fs::File;
use std::io::BufReader;
use eframe::egui;
use rodio::{Decoder, source::Source};

// Perform a forward FFT of size 1234
use rustfft::{FftPlanner, num_complex::Complex};

pub fn show_image(show_data: Vec<Complex<f32>>) {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "FFT Visualizer",
        native_options,
        Box::new(move |cc| Box::new(MyEguiApp::new(cc, show_data))),
    )
        .expect("Failed to run eframe");
}

struct MyEguiApp {
    show_data: Vec<Complex<f32>>,
}

impl MyEguiApp {
    fn new(
        _cc: &eframe::CreationContext<'_>,
        show_data: Vec<Complex<f32>>,
    ) -> Self {
        Self {
            show_data,
        }
    }
}

static mut INDEX: usize = 0;
const WINDOW_SIZE: usize = 4096;

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let source = self.show_data.iter().cycle().skip(unsafe { INDEX }).take(WINDOW_SIZE).copied().collect::<Vec<Complex<f32>>>();
        unsafe {
            INDEX += 1;
        }
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(WINDOW_SIZE);

        let mut buffer = source;

        fft.process(&mut buffer);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    let chart = egui_plot::BarChart::new(
                        buffer.iter().take(WINDOW_SIZE / 2)
                            .map(|c| egui_plot::Bar::new(c.im as f64, c.re as f64).width(0.001))
                            .collect(),
                    )
                        .color(egui::Color32::GOLD)
                        .name("FFT");

                    egui_plot::Plot::new("FFT")
                        .legend(egui_plot::Legend::default())
                        .clamp_grid(true)
                        .y_axis_width(4)
                        .allow_zoom([true, true])
                        .allow_drag([true, true])
                        .allow_scroll(true)
                        .show_grid([false, false])
                        .show(ui, |plot_ui| plot_ui.bar_chart(chart))
                        .response
                },
            );
        });

        ctx.request_repaint();
    }
}

fn main() {
    let file = BufReader::new(File::open("examples/musics/Data_No_1.wav").unwrap());
    let source = Decoder::new(file).unwrap();

    let source = Box::new(source.convert_samples::<f32>()).map(|s| Complex::new(s, 0.0)).collect::<Vec<Complex<f32>>>();

    show_image(source);
}
