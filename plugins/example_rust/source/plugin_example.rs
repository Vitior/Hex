extern crate hex;



#[hex::plugin_setup("Example Rust", "WerWolv", "Example Rust plugin used as template for plugin devs")]
fn init() {
    let view = hex::View::new("Hello",
                                          ||{ println!("Destructor"); },
                                        ||{ println!("Draw Content"); },
                                    ||{ println!("Draw Always Visible"); },
                                          || { println!("Draw Menu"); },
        || { return true; },
        || { return true; },
        || { return true; }
    );

    hex::ContentRegistry::Views::add(view);
}

