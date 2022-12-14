use allen::{AllenError, BufferData, Channels, Device};
use lewton::inside_ogg::OggStreamReader;
use std::{env, fs::File, thread, time::Duration};

fn main() -> Result<(), AllenError> {
    let path = env::args().nth(1).expect("no file specified.");

    let device = Device::open(None).unwrap();

    let context = device.create_context()?;
    context.make_current();

    assert!(context.is_current());

    let source = context.new_source()?;

    let mut buffers = vec![];
    {
        let mut ogg = OggStreamReader::new(File::open(path).unwrap()).unwrap();

        let mut _n = 0;
        let mut _length = 0.0;
        let sample_channels =
            ogg.ident_hdr.audio_channels as f32 * ogg.ident_hdr.audio_sample_rate as f32;

        while let Some(samples) = ogg.read_dec_packet_itl().unwrap() {
            _length += samples.len() as f32 / sample_channels;

            let buffer = context.new_buffer()?;
            buffer.data(
                BufferData::I16(&samples),
                match ogg.ident_hdr.audio_channels {
                    1 => Channels::Mono,
                    2 => Channels::Stereo,
                    n => panic!("unsupported amount of channels: {}", n),
                },
                ogg.ident_hdr.audio_sample_rate as i32,
            )?;
            source.queue_buffer(&buffer)?;
            buffers.push(buffer);
        }
    }

    source.play()?;

    loop {
        thread::sleep(Duration::from_secs_f32(1.0));
    }
}
