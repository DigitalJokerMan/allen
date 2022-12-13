use allen::{AllenError, BufferData, Channels};
use hound::WavReader;
use std::env;

fn main() -> Result<(), AllenError> {
    let path = env::args().nth(1).expect("no file specified.");

    let device = allen::Device::open(None).unwrap();
    println!("Device name: {}", device.device_name());

    let context = device.create_context()?;

    assert!(context.is_current());

    println!("loading...");
    let buffer = context.new_buffer()?;
    let source = context.new_source()?;

    {
        let mut reader = WavReader::open(path).unwrap();
        let samples = reader
            .samples::<i16>()
            .map(|s| s.unwrap())
            .collect::<Vec<_>>();

        buffer.data(
            BufferData::I16(&samples),
            Channels::Stereo,
            reader.spec().sample_rate as i32,
        )?;

        source.set_buffer(Some(&buffer));
        source.set_looping(true)?;
        source.play()?;
        println!("playback started");
    }

    println!("you can seek by inputting the desired time position (in seconds).\ninvalid numbers will simply report the current position.");

    loop {
        use std::io::stdin;

        let mut buffer = String::new();

        if let Some(new_position) = stdin()
            .read_line(&mut buffer)
            .ok()
            .and_then(|_| buffer.trim().parse::<f32>().ok())
        {
            println!("changing...");
            source.set_time_in_secs(new_position)?;
        }

        println!(
            "playback position: {}s / {} samples / {} bytes",
            source.time_in_secs()?,
            source.time_in_samples()?,
            source.time_in_bytes()?
        );
    }
}
