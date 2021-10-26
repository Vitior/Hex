pub mod ffi {

    pub mod content_registry {

        #[cxx::bridge]
        pub mod views {

            #[namespace = "hex"]
            pub struct View {
                x: u8
            }

            #[namespace = "hex::ContentRegistry::Views"]
            extern "C++" {
                include!("hex/api/content_registry.hpp");

                pub unsafe fn add(view: * mut View);
            }

        }

    }
}

pub mod content_registry {
    pub mod views {
        pub fn add(view: * mut crate::rust_utils_ffi::ffi::rust_utils::View) {
            unsafe {
                crate::content_registry_ffi::ffi::content_registry::views::add(view as *mut _);
            }
        }
    }
}