use allen::{AllenError, Device};
use std::ffi::CString;

fn main() -> Result<(), AllenError> {
    let device = Device::open(None).unwrap();

    let context = device.create_context()?;
    context.make_current();

    assert!(context.is_current());

    println!(
        "{}\n{}\n{}\n{}",
        context.vendor(),
        context.version(),
        context.renderer(),
        context.extensions()
    );

    let ext_name = CString::new("AL_EXT_vorbis").unwrap();

    println!(
        "{} {}",
        device.is_extension_present(&ext_name)?,
        allen::is_extension_present(&ext_name)?,
    );

    Ok(())
}
