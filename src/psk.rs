use num::complex::Complex32;
use yttria_math::linspace;
use yttria_math::prelude::*;
use rayon::prelude::*;

use crate::traits::*;

pub struct PskModulation {
    pub bits_per_symbol: usize,
    pub sample_rate: f32,
    pub bandwidth: f32,
    pub symbol_map: Vec<Complex32>,
    // spreading_code: Option<Vec<u8>>,
}

impl PskModulation {
    pub fn new(bits_per_symbol: usize, sample_rate: f32, bandwidth: f32) -> Self {
        let npoints = usize::pow(2, bits_per_symbol as u32);

        let offset = if bits_per_symbol % 2 == 0 {
            std::f32::consts::PI / npoints as f32
        } else {
            0.0
        };

        let symbol_angles = linspace(offset, 2.0 * std::f32::consts::PI + offset, npoints, false);

        let symbol_map = symbol_angles
            .iter()
            .map(|x| Complex32::new(0.0, *x).exp())
            .collect::<Vec<_>>();

        Self {
            bits_per_symbol,
            sample_rate,
            bandwidth,
            symbol_map,
        }
    }
}

impl Modulation for PskModulation {
    fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    fn bits_per_symbol(&self) -> usize {
        self.bits_per_symbol
    }

    fn symbol_period(&self) -> f32 {
        2.0 / self.bandwidth
    }

    fn bitrate(&self) -> f32 {
        self.bits_per_symbol() as f32 / self.symbol_period()
    }

    fn samples_per_symbol(&self) -> usize {
        (self.symbol_period() * self.sample_rate) as usize
    }

    fn symbol_map(&self) -> &[Complex32] {
        self.symbol_map.as_ref()
    }
}

impl Modulator for PskModulation {
    fn modulate_into(&self, data: &[u8], out: &mut [Complex32]) {
        assert!(out.len() >= (data.len() * 8).div_ceil(self.bits_per_symbol()) * self.samples_per_symbol());

        let bits = data.unpackbits();
        println!("{} {} {}", data.len(), self.samples_per_symbol(), out.len());

        out.par_chunks_exact_mut(self.samples_per_symbol())
            .zip(bits.par_chunks(self.bits_per_symbol()))
            .for_each(|(out, symbol)| {
                let sym = if symbol.len() == self.bits_per_symbol() {
                    &self.symbol_map[symbol.pack_into::<usize>()]
                }
                else {
                    let sym_offset = self.bits_per_symbol() - symbol.len();
                    &self.symbol_map[symbol.pack_into::<usize>() << sym_offset]
                };

                out.fill(*sym);
            });
    }

    fn modulate(&self, data: &[u8]) -> Vec<Complex32> {
        let len = (data.len() * 8).div_ceil(self.bits_per_symbol()) * self.samples_per_symbol();
        let mut out = Vec::with_capacity(len);
        unsafe { out.set_len(len) };
        self.modulate_into(data, out.as_mut_slice());
        out
    }
}

impl Demodulator for PskModulation {
    fn demodulate_into(&self, _samples: &[Complex32], _out: &mut [u8]) {
        todo!()
    }

    fn demodulate(&self, samples: &[Complex32]) -> Vec<u8> {
        let len = (samples.len() * self.bits_per_symbol()).div_ceil(self.samples_per_symbol() * 8);
        let mut out = Vec::with_capacity(len);
        unsafe { out.set_len(len) };
        self.demodulate_into(samples, out.as_mut_slice());
        out
    }
}

#[cfg(test)]
mod tests {
    use plotly::{Plot, Scatter};

    use super::*;
    use yttria_math::{arange, firwin2};

    #[test]
    fn test_bpsk_modulate() {
        let psk = PskModulation::new(3, 20e6, 5e6 / 3.0);

        let filter = firwin2(
            psk.samples_per_symbol(),
            &[0.0, 0.5 / 3.0, 0.5 / 3.0, 1.0],
            &[1.0, 1.0, 0.0, 0.0],
            false,
        ).as_type();

        let data = b"hello, world!";

        let iq = psk.modulate(data);
        let iq = iq.convolve(filter.as_slice());

        let mut plot = Plot::new();

        let trace = Scatter::new(arange(0, iq.len(), 1), iq.real());
        plot.add_trace(trace);
        let trace = Scatter::new(arange(0, iq.len(), 1), iq.imag());
        plot.add_trace(trace);

        plot.show();
    }
}
