use cxx::CxxString;
use imgui::sys::ImVec2;

pub mod ffi {

    pub mod ContentRegistry {

        #[cxx::bridge]
        pub mod Views {

            #[namespace = "hex"]
            pub struct View {
                x: u8
            }

            #[namespace = "hex::ContentRegistry::Views"]
            extern "C++" {
                include!("hex/api/content_registry.hpp");

                pub unsafe fn add(view: * mut View);
                pub unsafe fn createRustView(unlocalizedName: &CxxString,
                                             destructorFunc: fn(),
                                             drawContentFunc: fn(& mut bool),
                                             drawAlwaysVisibleFunc: fn(),
                                             drawMenuFunc: fn(),
                                             isAvailableFunc: fn() -> bool,
                                             shouldProcessFunc: fn() -> bool,
                                             hasViewMenuItemEntryFunc: fn() -> bool) -> * mut View;
            }

        }

    }
}

impl ffi::ContentRegistry::Views::View {
    pub fn new(unlocalized_name: &str,
               destructorFunc: fn(),
               drawContentFunc: fn(& mut bool),
               drawAlwaysVisibleFunc: fn(),
               drawMenuFunc: fn(),
               isAvailableFunc: fn() -> bool,
               shouldProcessFunc: fn() -> bool,
               hasViewMenuItemEntryFunc: fn() -> bool) -> * mut Self {
        unsafe {
            cxx::let_cxx_string!(cpp_name = unlocalized_name);
            crate::content_registry::ffi::ContentRegistry::Views::createRustView(&cpp_name,
                                                                                 destructorFunc,
                                                                                 drawContentFunc,
                                                                                 drawAlwaysVisibleFunc,
                                                                                 drawMenuFunc,
                                                                                 isAvailableFunc,
                                                                                 shouldProcessFunc,
                                                                                 hasViewMenuItemEntryFunc)
        }
    }
}

pub mod ContentRegistry {
    pub mod Views {
        pub fn add(view: * mut crate::content_registry::ffi::ContentRegistry::Views::View) {
            unsafe {
                crate::content_registry::ffi::ContentRegistry::Views::add(view as *mut _);
            }
        }
    }
}