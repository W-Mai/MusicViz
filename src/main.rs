use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, source::Source};

// Perform a forward FFT of size 1234
use rustfft::{FftPlanner, num_complex::Complex};


fn main() {
    let file = BufReader::new(File::open("examples/musics/Data_No_1.wav").unwrap());
    let source = Decoder::new(file).unwrap();

    let source = source.convert_samples::<f32>().map(|val| {
        Complex { re: val, im: 0.0 }
    }).take(1024).collect::<Vec<_>>();

    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(1024);

    let mut buffer = source;

    fft.process(&mut buffer);

    for i in buffer {
        println!("{:?}", i);
    }
}
