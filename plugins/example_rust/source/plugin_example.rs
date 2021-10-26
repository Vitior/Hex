extern crate hex;

use hex::imgui::Window;

#[hex::plugin_setup("Example Rust", "WerWolv", "Example Rust plugin used as template for plugin devs")]
fn init() {
    let view = hex::ViewBuilder::new("Rust View")
        .draw_content(|open: &mut bool| {
            let mut ctx = hex::imgui::Context::current().unwrap();
            let ui = ctx.current_ui();

            Window::new("Rust View")
                .opened(open)
                .build(&ui, || {
                    ui.text("Label");

                    ui.separator();

                    if ui.button("Click") {
                        println!("Clicked!");
                    }
                });
        }).build();

    hex::content_registry::views::add(view);
}

