use rodio::{OutputStream, Sink, source::Source};
use std::f32::consts::PI;
use std::iter::Iterator;
// use plotters::prelude::*;

#[derive(Debug, Clone)]
pub enum Stream {
    SineWave ( SineWave ),
    ModulatedSineWave ( ModulatedSineWave ),
    Mix ( Mix ),
    Silence ( Silence ),
    Empty ( Empty )
}

impl Iterator for Stream {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::SineWave(s) => s.next(),
            Self::ModulatedSineWave(s) => s.next(),
            Self::Mix(s) => s.next(),
            Self::Silence(s) => s.next(),
            Self::Empty(s) => s.next(),
        }
    }
}

// A struct that generates a sine wave at a given frequency and sample rate.
#[derive(Debug, Clone)]
pub struct SineWave {
    frequency: f32,
    sample_rate: u32,
    current_sample: u32,
}

impl SineWave {
    pub fn new() -> Self {
        Self {
            frequency: 0f32,
            sample_rate: 44100,
            current_sample: 0,
        }
    }
    
    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
    }
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        // Compute the next sample in the sine wave
        let sample = (self.current_sample as f32 * 2.0 * PI * self.frequency / self.sample_rate as f32).sin();
        self.current_sample += 1;
        Some(sample)
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1 // Mono sound
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

// A struct that generates a sine wave at a given frequency and sample rate modulated by.
#[derive(Debug, Clone)]
pub struct ModulatedSineWave {
    frequency: f32,
    sample_rate: u32,
    modulator: Box<Stream>,
    current_sample: f32,
}

impl ModulatedSineWave {
    pub fn new() -> Self {
        Self {
            frequency: 0f32,
            sample_rate: 44100,
            modulator: Box::new(Stream::Empty(Empty::new())),
            current_sample: 0f32,
        }
    }

    pub fn set_modulator(&mut self, modulator: Stream) {
        self.modulator = Box::new(modulator);
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
    }
}

impl Iterator for ModulatedSineWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        // Compute the next sample in the sine wave
        self.modulator.next().map(|a| { 
            let sample = 1f32 * (self.current_sample as f32 * 2.0 * PI * self.frequency / self.sample_rate as f32).sin();
            self.current_sample = (self.current_sample + 1.0 + a) % self.sample_rate as f32;
            sample
        })
    }
}

impl Source for ModulatedSineWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1 // Mono sound
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

// mixes two audio streams
#[derive(Debug, Clone)]
pub struct Mix {
    sample_rate: u32,
    stream_a: Box<Stream>,
    stream_b: Box<Stream>,
    p: f32,
}

impl Iterator for Mix {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        // Compute the next sample in the sine wave
        self.stream_a.next().and_then(|a| { 
        self.stream_b.next().map(|b| {
            self.p * a + (1f32 - self.p) * b
        }) })
    }
}

impl Mix {
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
            stream_a: Box::new(Stream::Empty(Empty::new())),
            stream_b: Box::new(Stream::Empty(Empty::new())),
            p: 0.5
        }
    }

    pub fn set_stream_a(&mut self, modulator: Stream) {
        self.stream_a = Box::new(modulator);
    }

    pub fn set_stream_b(&mut self, modulator: Stream) {
        self.stream_b = Box::new(modulator);
    }

    pub fn set_p(&mut self, p: f32) {
        self.p = p;
    }
}

impl Source for Mix {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1 // Mono sound
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct Silence { sample_rate: u32 }

impl Silence {
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
        }
    }
}

impl Iterator for Silence {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        Some(0f32)
    }
}

impl Source for Silence {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1 // Mono sound
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct Empty { sample_rate: u32 }

impl Empty {
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
        }
    }
}

impl Iterator for Empty {
    type Item = f32;

    fn next(&mut self) -> Option<f32> { None }
}

impl Source for Empty {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1 // Mono sound
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}