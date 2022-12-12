use std::ffi::{CStr, CString};

use allen::Device;

fn main() {
    let device = Device::open(None).unwrap();

    let context = device.create_context().unwrap();
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
        device.is_extension_present(&ext_name).unwrap(),
        allen::is_extension_present(&ext_name).unwrap(),
    );
}
