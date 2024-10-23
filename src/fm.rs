use rodio::{OutputStream, Sink, source::Source};
use std::f32::consts::PI;
use std::iter::Iterator;
use noise::{self, NoiseFn};
use rand;
// use plotters::prelude::*;

#[derive(Debug, Clone)]
pub enum Stream {
    SineWave ( SineWave ),
    SquareWave ( SquareWave ),
    TriangleWave ( TriangleWave ),
    SawtoothWave ( SawtoothWave ),
    ModulatedSineWave ( ModulatedSineWave ),
    Mix ( Mix ),
    Const ( Const ),
    Empty ( Empty ),
    Envelope ( Envelope ),
    Perlin ( Perlin ),
    WhiteNoise ( WhiteNoise ),
    Add ( Add ),
    Multiply ( Multiply ),
}

impl Iterator for Stream {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::SineWave(s) => s.next(),
            Self::SquareWave(s) => s.next(),
            Self::TriangleWave(s) => s.next(),
            Self::SawtoothWave(s) => s.next(),
            Self::ModulatedSineWave(s) => s.next(),
            Self::Mix(s) => s.next(),
            Self::Const(s) => s.next(),
            Self::Envelope ( s ) => s.next(),
            Self::Perlin ( s ) => s.next(),
            Self::WhiteNoise ( s ) => s.next(),
            Self::Empty (s) => s.next(),
            Self::Add ( s ) => s.next(),
            Self::Multiply (s) => s.next(),
        }
    }
}

impl Default for Stream {
    fn default() -> Self {
        Self::Empty(Empty::new())
    }
}

impl Source for Stream {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1 // Mono sound
    }

    fn sample_rate(&self) -> u32 {
        match self {
            Self::SineWave(s) => s.sample_rate(),
            Self::SquareWave(s) => s.sample_rate(),
            Self::TriangleWave(s) => s.sample_rate(),
            Self::SawtoothWave(s) => s.sample_rate(),
            Self::ModulatedSineWave(s) => s.sample_rate(),
            Self::Mix(s) => s.sample_rate(),
            Self::Const(s) => s.sample_rate(),
            Self::Envelope ( s ) => s.sample_rate(),
            Self::Perlin ( s ) => s.sample_rate(),
            Self::WhiteNoise ( s ) => s.sample_rate(),
            Self::Empty(s) => s.sample_rate(),
            Self::Add ( s ) => s.sample_rate(),
            Self::Multiply (s) => s.sample_rate(),
        }
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

// A struct that generates a sine wave at a given frequency and sample rate.
#[derive(Debug, Clone)]
pub struct SineWave {
    frequency: f32,
    sample_rate: u32,
    current_sample: u32,
    phase_shift: f32,
}

impl SineWave {
    pub fn new() -> Self {
        Self {
            frequency: 0f32,
            sample_rate: 44100,
            current_sample: 0,
            phase_shift: 0f32,
        }
    }
    
    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
    }

    pub fn set_phase_shift(&mut self, shift: f32) {
        self.phase_shift = shift;
    }
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        // Compute the next sample in the sine wave
        let sample = ((self.current_sample as f32 + self.phase_shift) * 2.0 * PI * self.frequency / self.sample_rate as f32).sin();
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

// A struct that generates a sine wave at a given frequency and sample rate.
#[derive(Debug, Clone)]
pub struct SquareWave {
    frequency: f32,
    sample_rate: u32,
    current_sample: u32,
    phase_shift: f32,
}

impl SquareWave {
    pub fn new() -> Self {
        Self {
            frequency: 0f32,
            sample_rate: 44100,
            current_sample: 0,
            phase_shift: 0f32,
        }
    }
    
    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
    }

    pub fn set_phase_shift(&mut self, shift: f32) {
        self.phase_shift = shift;
    }
}

fn square_wave(x: f32) -> f32 {
    if x % 1f32 <= 0.5 { 1f32 }
    else { -1f32 }
}

impl Iterator for SquareWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        // Compute the next sample in the sine wave
        let sample = square_wave((self.current_sample as f32 + self.phase_shift) * self.frequency / self.sample_rate as f32);
        self.current_sample += 1;
        Some(sample)
    }
}

impl Source for SquareWave {
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

// A struct that generates a sine wave at a given frequency and sample rate.
#[derive(Debug, Clone)]
pub struct TriangleWave {
    frequency: f32,
    sample_rate: u32,
    current_sample: u32,
    phase_shift: f32,
}

impl TriangleWave {
    pub fn new() -> Self {
        Self {
            frequency: 0f32,
            sample_rate: 44100,
            current_sample: 0,
            phase_shift: 0f32,
        }
    }
    
    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
    }

    pub fn set_phase_shift(&mut self, shift: f32) {
        self.phase_shift = shift;
    }
}

fn triangle_wave(x: f32) -> f32 {
    4.0 * (x + 0.25 - (x + 0.75).floor()).abs() - 1.0
}

impl Iterator for TriangleWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        // Compute the next sample in the sine wave
        let sample = triangle_wave((self.current_sample as f32 + self.phase_shift) * self.frequency / self.sample_rate as f32);
        self.current_sample += 1;
        Some(sample)
    }
}

impl Source for TriangleWave {
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
pub struct SawtoothWave {
    frequency: f32,
    sample_rate: u32,
    current_sample: u32,
    phase_shift: f32,
}

impl SawtoothWave {
    pub fn new() -> Self {
        Self {
            frequency: 0f32,
            sample_rate: 44100,
            current_sample: 0,
            phase_shift: 0f32,
        }
    }
    
    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
    }

    pub fn set_phase_shift(&mut self, shift: f32) {
        self.phase_shift = shift;
    }
}

fn sawtooth_wave(x: f32) -> f32 {
    x % 1.0
}

impl Iterator for SawtoothWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        // Compute the next sample in the sine wave
        let sample = sawtooth_wave((self.current_sample as f32 + self.phase_shift) * self.frequency / self.sample_rate as f32);
        self.current_sample += 1;
        Some(sample)
    }
}

impl Source for SawtoothWave {
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
pub struct Const { 
    sample_rate: u32,
    val: f32,
}

impl Const {
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
            val: 0f32
        }
    }

    pub fn set_val(&mut self, val: f32) {
        self.val = val;
    }
}

impl Iterator for Const {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        Some(self.val)
    }
}

impl Source for Const {
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

#[derive(Debug, Clone)]
pub struct Envelope {
    // ADSR
    a:  f32,
    ad: f32,
    dd: f32,
    s:  f32,
    sd: f32,
    rd: f32,
    stream: Box<Stream>,
    sample_rate: u32,
    current_sample: u32,
}

impl Envelope {
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
            current_sample: 0,
            a: 1.0,
            ad: 0.3,
            dd: 0.3,
            s: 0.6,
            sd: 2.0,
            rd: 1.0,
            stream: Box::new(Stream::default()),
        }
    }
    
    pub fn set_stream(&mut self, stream: Stream) { self.stream = Box::new(stream); }
    pub fn set_a(&mut self, v: f32) -> () { self.a = v; } 
    pub fn set_ad(&mut self, v: f32) -> () { self.ad = v; } 
    pub fn set_dd(&mut self, v: f32) -> () { self.dd = v; } 
    pub fn set_s(&mut self, v: f32) -> () { self.s = v; } 
    pub fn set_sd(&mut self, v: f32) -> () { self.sd = v; } 
    pub fn set_rd(&mut self, v: f32) -> () { self.rd = v; } 
}

fn lerp(a: f32, b: f32, f: f32) -> f32 { a * (1.0-f) + b * f }

impl Iterator for Envelope {
    type Item = f32;

    fn next(&mut self) -> Option<f32> { 
        let t = self.current_sample as f32 / self.sample_rate as f32;
        self.current_sample += 1;
        self.stream.next().map(|sample: f32| {
            sample * {
            if t < self.ad { lerp(0.0, self.a, t/self.ad) }
            else if (t < self.ad + self.dd) { lerp(self.a,self.s, (t-self.ad)/self.dd) }
            else if (t < self.ad + self.dd + self.sd) { self.s }
            else if (t < self.ad + self.dd + self.sd + self.rd) { lerp(self.s,0.0, (t-self.ad-self.dd-self.sd)/self.rd) }
            else { 0.0 }}
        }) 
    }
}

impl Source for Envelope {
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
pub struct Perlin {
    scale: f32,
    perl: noise::Perlin,
    sample_rate: u32,
    current_sample: u32,
}

impl Perlin {
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
            current_sample: 0,
            perl: noise::Perlin::new(69),
            scale: 1.0
        }
    }

    pub fn set_scale(&mut self, v: f32) { self.scale = v; }
}

impl Iterator for Perlin {
    type Item = f32;

    fn next(&mut self) -> Option<f32> { 
        // Compute the next sample in the sine wave
        let sample = self.perl.get([(self.current_sample as f32 * self.scale / self.sample_rate as f32) as f64]);
        self.current_sample += 1;
        Some(sample as f32 * 2.0 - 1.0)
    }
}

impl Source for Perlin {
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
pub struct WhiteNoise { sample_rate: u32 }

impl WhiteNoise {
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
        }
    }
}

impl Iterator for WhiteNoise {
    type Item = f32;

    fn next(&mut self) -> Option<f32> { 
        Some(rand::random::<f32>() * 2.0 - 1.0)
    }
}

impl Source for WhiteNoise {
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
pub struct Add {
    sample_rate: u32,
    stream_a: Box<Stream>,
    stream_b: Box<Stream>,
}

impl Iterator for Add {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        // Compute the next sample in the sine wave
        self.stream_a.next().and_then(|a| { 
        self.stream_b.next().map(|b| {
            a + b
        }) })
    }
}

impl Add {
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
            stream_a: Box::new(Stream::Empty(Empty::new())),
            stream_b: Box::new(Stream::Empty(Empty::new())),
        }
    }

    pub fn set_stream_a(&mut self, modulator: Stream) {
        self.stream_a = Box::new(modulator);
    }

    pub fn set_stream_b(&mut self, modulator: Stream) {
        self.stream_b = Box::new(modulator);
    }
}

impl Source for Add {
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
pub struct Multiply {
    sample_rate: u32,
    stream_a: Box<Stream>,
    stream_b: Box<Stream>,
}

impl Iterator for Multiply {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        // Compute the next sample in the sine wave
        self.stream_a.next().and_then(|a| { 
        self.stream_b.next().map(|b| {
            a * b
        }) })
    }
}

impl Multiply {
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
            stream_a: Box::new(Stream::Empty(Empty::new())),
            stream_b: Box::new(Stream::Empty(Empty::new())),
        }
    }

    pub fn set_stream_a(&mut self, modulator: Stream) {
        self.stream_a = Box::new(modulator);
    }

    pub fn set_stream_b(&mut self, modulator: Stream) {
        self.stream_b = Box::new(modulator);
    }
}

impl Source for Multiply {
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