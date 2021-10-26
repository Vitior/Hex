pub mod ffi {

    pub mod imhex_api {

        #[cxx::bridge]
        pub mod common {

            #[namespace = "hex::ImHexApi::Common"]
            extern "C++" {
                include!("hex/api/imhex_api.hpp");

                pub unsafe fn closeImHex(no_questions: bool);
                pub unsafe fn restartImHex();
            }

        }

        #[cxx::bridge]
        pub mod bookmarks {

            #[namespace = "hex::ImHexApi::Bookmarks"]
            extern "C++" {
                include!("hex/api/imhex_api.hpp");

                pub unsafe fn add(addr : u64, size : usize, name : &CxxString, comment : &CxxString, color : u32);
            }

        }

    }
}

pub struct Region {
    pub address : u64,
    pub size : usize
}

pub struct Color {
    pub a : u8,
    pub g : u8,
    pub b : u8,
    pub r : u8
}

impl Region {
    pub fn new(address : u64, size : usize) -> Self {
        Region { address, size }
    }
}

impl Color {
    pub fn new(r : u8, g : u8, b : u8, a : u8) -> Self {
        Color { a, g, b, r }
    }

    pub fn rgba(self) -> u32 {
        (self.a as u32) << 24 | (self.b as u32) << 16 | (self.g as u32) << 8 | (self.r as u32) << 0
    }
}

pub mod imhex_api {

    pub mod common {

        pub fn close_imhex() {

            unsafe {
                crate::imhex_api_ffi::ffi::imhex_api::common::closeImHex(false);
            }

        }

        pub fn restart_imhex() {

            unsafe {
                crate::imhex_api_ffi::ffi::imhex_api::common::restartImHex();
            }

        }

    }

    pub mod bookmarks {

        pub fn add(region : crate::Region, name : &str, comment : &str, color : Option<crate::Color>) {
            cxx::let_cxx_string!(cpp_name = name);
            cxx::let_cxx_string!(cpp_comment = comment);

            unsafe {
                crate::imhex_api_ffi::ffi::imhex_api::bookmarks::add(region.address, region.size, &cpp_name, &cpp_comment, color.unwrap_or(crate::Color::new(0, 0, 0, 0)).rgba());
            }
        }

    }

}