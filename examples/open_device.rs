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
}
