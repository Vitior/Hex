extern crate hex;

#[hex::plugin_setup("Example Rust", "WerWolv", "Example Rust plugin used as template for plugin devs")]
fn init() {
    let mut view: Box<hex::View> = hex::View::new();

    hex::ContentRegistry::Views::add(& mut view);

    Box::leak(view);
}

