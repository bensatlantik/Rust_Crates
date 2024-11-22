use std::f32::consts::PI;
use hound::{WavSpec, SampleFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // WAV file settings
    let spec = WavSpec {
        channels: 1,                      // Mono audio
        sample_rate: 44100,               // Standard audio sample rate
        bits_per_sample: 16,              // 16-bit samples
        sample_format: SampleFormat::Int, // Integer sample format
    };

    let amplitude = i16::MAX as f32; // Maximum amplitude for 16-bit audio
    let frequency = 440.0;           // Frequency in Hz
    let duration = 2.0;              // Duration in seconds

    // Open a WAV writer
    let mut writer = hound::WavWriter::create("sine_wave_440hz.wav", spec)?;

    // Generate samples
    let num_samples = (spec.sample_rate as f32 * duration) as usize;
    let sample_rate = spec.sample_rate as f32;

    for t in 0..num_samples {
        let sample = (amplitude * (2.0 * PI * frequency * t as f32 / sample_rate).sin()) as i16;
        writer.write_sample(sample)?;
    }

    writer.finalize()?;
    println!("Generated sine_wave_440hz.wav");

    Ok(())
}
