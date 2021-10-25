#![allow(non_snake_case)]

mod imhex_api;
mod content_registry;
mod rust_utils;

pub use macros::plugin_setup;
pub use imhex_api::ImHexApi;
pub use imhex_api::Region;
pub use imhex_api::Color;
pub use imgui;

pub use content_registry::ContentRegistry;

pub use rust_utils::ViewBuilder;