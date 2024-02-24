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

#[derive(Default)]
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

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    let chart = egui_plot::BarChart::new(
                        self.show_data.iter()
                            .map(|c| egui_plot::Bar::new(c.im as f64, (c.re as f64).abs()).width(0.1))
                            .collect(),
                    )
                        .color(egui::Color32::LIGHT_BLUE)
                        .name("FFT");

                    egui_plot::Plot::new("FFT")
                        .legend(egui_plot::Legend::default())
                        .clamp_grid(true)
                        .y_axis_width(4)
                        .allow_zoom([false, false])
                        .allow_drag([true, false])
                        .allow_scroll(true)
                        .show(ui, |plot_ui| plot_ui.bar_chart(chart))
                        .response
                },
            );
        });
    }
}

fn main() {
    let file = BufReader::new(File::open("examples/musics/Data_No_1.wav").unwrap());
    let source = Decoder::new(file).unwrap();

    let source = source.convert_samples::<f32>().map(|val| {
        Complex { re: val, im: 0.0 }
    }).take(4096).collect::<Vec<_>>();

    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(4096);

    let mut buffer = source;

    fft.process(&mut buffer);

    show_image(buffer.split_off(2048));
}
