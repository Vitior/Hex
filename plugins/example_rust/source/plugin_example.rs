extern crate hex;

use hex::imgui::{Condition, Window};


#[hex::plugin_setup("Example Rust", "WerWolv", "Example Rust plugin used as template for plugin devs")]
fn init() {
    let view = hex::View::new("Hello",
                                          ||{ println!("Destructor"); },
                                        |& mut mut open|{
                                            let mut ctx = hex::imgui::Context::current().unwrap();
                                            let ui = ctx.current_ui();
                                            Window::new("Test").size([300.0, 300.0], Condition::FirstUseEver)
                                                .opened(& mut open)
                                                .build(&ui, || {
                                                    ui.text("Hello World from Rust!");
                                                    ui.separator();
                                                    ui.button("Hello");
                                                });
                                        },
                                    ||{  },
                                          || {  },
        || { return true; },
        || { return true; },
        || { return true; }
    );

    hex::ContentRegistry::Views::add(view);
}

