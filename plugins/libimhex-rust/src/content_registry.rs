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
            }

        }

    }
}

pub mod ContentRegistry {
    pub mod Views {
        pub fn add(view: * mut crate::rust_utils::ffi::RustUtils::View) {
            unsafe {
                crate::content_registry::ffi::ContentRegistry::Views::add(view as *mut _);
            }
        }
    }
}