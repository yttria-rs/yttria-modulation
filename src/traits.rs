use num::complex::Complex32;

pub trait Modulation {
    fn sample_rate(&self) -> f32;
    fn bits_per_symbol(&self) -> usize;
    fn symbol_period(&self) -> f32;
    fn bitrate(&self) -> f32;
    fn samples_per_symbol(&self) -> usize;
    fn symbol_map(&self) -> &[Complex32];
}

pub trait Modulator {
    fn modulate_into(&self, data: &[u8], out: &mut [Complex32]);
    fn modulate(&self, data: &[u8]) -> Vec<Complex32>;
}

pub trait Demodulator {
    fn demodulate_into(&self, samples: &[Complex32], out: &mut [u8]);
    fn demodulate(&self, samples: &[Complex32]) -> Vec<u8>;
}
