use std::{env, thread, time::Duration};

use allen::{Buffer, BufferData, Channels, Source};
use hound::WavReader;

fn main() {
    let path = env::args().nth(1).expect("no file specified.");

    let device = allen::Device::open(None).unwrap();
    println!("Device name: {}", device.device_name());

    let context = device.create_context().unwrap();
    context.make_current();

    assert!(context.is_current());

    println!("loading...");
    let buffer = Buffer::new().unwrap();
    let source = Source::new().unwrap();

    {
        let mut reader = WavReader::open(path).unwrap();
        let samples = reader
            .samples::<i16>()
            .map(|s| s.unwrap())
            .collect::<Vec<_>>();

        buffer
            .data(
                BufferData::I16(samples),
                Channels::Stereo,
                reader.spec().sample_rate as i32,
            )
            .unwrap();

        source.set_buffer(&buffer);
        source.set_looping(true);
        source.play().unwrap();
        println!("playback started");
    }

    loop {
        println!(
            "playback position: {} ({} samples)",
            source.playback_position(),
            source.playback_position_in_samples()
        );
        thread::sleep(Duration::from_secs(5));
    }
}
