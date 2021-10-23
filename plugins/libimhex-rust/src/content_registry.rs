use std::ptr;

pub mod ffi {

    pub mod ContentRegistry {

        #[cxx::bridge]
        pub mod Views {

            #[namespace = "hex"]
            pub struct View {
                pub vpointer: * mut * mut u8,
                pub vtable: [* mut u8; 10]
            }

            #[namespace = "hex::ContentRegistry::Views"]
            extern "C++" {
                include!("hex/api/content_registry.hpp");

                pub unsafe fn add(view: * mut View);
            }

        }

    }
}

fn drawContent() {
    println!("Rust Draw Content!");
}

fn placeholder() {
    println!("Rust Placeholder!");
}

fn hasViewMenuItemEntry() -> bool{
    println!("Rust hasViewMenuItemEntry");
    return true;
}

impl ffi::ContentRegistry::Views::View {
    pub fn new() -> Self {
        unsafe {
            let mut view = ffi::ContentRegistry::Views::View {
                vpointer: ptr::null_mut(),
                vtable: [ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut()]
            };

            view.vpointer = view.vtable.as_mut_ptr();
            view.vtable[0] = std::mem::transmute(placeholder as fn()); // ~View
            view.vtable[1] = std::mem::transmute(drawContent as fn()); // drawContent
            view.vtable[2] = std::mem::transmute(placeholder as fn()); // drawAlwaysVisible
            view.vtable[3] = std::mem::transmute(placeholder as fn()); // drawMenu
            view.vtable[4] = std::mem::transmute(placeholder as fn()); // handleShortcut
            view.vtable[5] = std::mem::transmute(placeholder as fn()); // isAvailable
            view.vtable[6] = std::mem::transmute(placeholder as fn()); // shouldProcess
            view.vtable[7] = std::mem::transmute(hasViewMenuItemEntry as fn() -> bool); // hasViewMenuItemEntry
            view.vtable[8] = std::mem::transmute(placeholder as fn()); // getMinSize
            view.vtable[9] = std::mem::transmute(placeholder as fn()); // getMaxSize

            view
        }

    }
}

pub mod ContentRegistry {
    pub mod Views {
        pub fn add(view: & mut crate::content_registry::ffi::ContentRegistry::Views::View) {
            unsafe {
                crate::content_registry::ffi::ContentRegistry::Views::add(view as *mut _);
            }
        }
    }
}