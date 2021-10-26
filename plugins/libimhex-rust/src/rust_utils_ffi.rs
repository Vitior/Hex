pub mod ffi {
    #[cxx::bridge]
    pub mod rust_utils {

        #[namespace = "hex"]
        pub struct View { placeholder: u8 }

        #[namespace = "hex::rust"]
        extern "C++" {
            include!("rust_utils.hpp");

            #[rust_name = "create_rust_view"]
            pub unsafe fn createRustView(unlocalizedName: &CxxString,
                                         destructorValid: bool,
                                         destructorFunc: fn(),
                                         drawContentValid: bool,
                                         drawContentFunc: fn(&mut bool),
                                         drawAlwaysVisibleValid: bool,
                                         drawAlwaysVisibleFunc: fn(),
                                         drawMenuValid: bool,
                                         drawMenuFunc: fn(),
                                         isAvailableValid: bool,
                                         isAvailableFunc: fn() -> bool,
                                         shouldProcessValid: bool,
                                         shouldProcessFunc: fn() -> bool,
                                         hasViewMenuItemEntryValid: bool,
                                         hasViewMenuItemEntryFunc: fn() -> bool) -> *mut View;
        }
    }
}

pub struct ViewBuilder {
    unlocalized_name: String,

    destructor_func: Option<fn()>,
    draw_content_func: Option<fn(& mut bool)>,
    draw_always_visible_func: Option<fn()>,
    draw_menu_func: Option<fn()>,
    is_available_func: Option<fn() -> bool>,
    should_process_func: Option<fn() -> bool>,
    has_view_menu_item_entry_func: Option<fn() -> bool>
}

impl ViewBuilder {

    pub fn new(unlocalized_name: &str) -> Self {
        ViewBuilder {
            unlocalized_name: String::from(unlocalized_name),

            destructor_func: None,
            draw_content_func: None,
            draw_always_visible_func: None,
            draw_menu_func: None,
            is_available_func: None,
            should_process_func: None,
            has_view_menu_item_entry_func: None
        }
    }

    pub fn destructor(&mut self, func: fn()) -> &mut Self {
        self.destructor_func = Some(func);

        self
    }

    pub fn draw_content(&mut self, func: fn(& mut bool)) -> &mut Self {
        self.draw_content_func = Some(func);

        self
    }

    pub fn draw_always_visible(&mut self, func: fn()) -> &mut Self {
        self.draw_always_visible_func = Some(func);

        self
    }

    pub fn draw_menu(&mut self, func: fn()) -> &mut Self {
        self.draw_menu_func = Some(func);

        self
    }

    pub fn is_available(&mut self, func: fn() -> bool) -> &mut Self {
        self.is_available_func = Some(func);

        self
    }

    pub fn should_process(&mut self, func: fn() -> bool) -> &mut Self {
        self.should_process_func = Some(func);

        self
    }

    pub fn has_view_menu_item_entry(&mut self, func: fn() -> bool) -> &mut Self {
        self.has_view_menu_item_entry_func = Some(func);

        self
    }

    pub fn build(&mut self) -> *mut ffi::rust_utils::View {
        unsafe {
            cxx::let_cxx_string!(cpp_name = &self.unlocalized_name);
            crate::rust_utils_ffi::ffi::rust_utils::create_rust_view(&cpp_name,
                                                                     self.destructor_func.is_some(),
                                                                     self.destructor_func.unwrap_or(||{}),
                                                                     self.draw_content_func.is_some(),
                                                                     self.draw_content_func.unwrap_or(|_: &mut bool|{}),
                                                                     self.draw_always_visible_func.is_some(),
                                                                     self.draw_always_visible_func.unwrap_or(||{}),
                                                                     self.draw_menu_func.is_some(),
                                                                     self.draw_menu_func.unwrap_or(||{}),
                                                                     self.is_available_func.is_some(),
                                                                     self.is_available_func.unwrap_or(||{ false }),
                                                                     self.should_process_func.is_some(),
                                                                     self.should_process_func.unwrap_or(||{ false }),
                                                                     self.has_view_menu_item_entry_func.is_some(),
                                                                     self.has_view_menu_item_entry_func.unwrap_or(||{ false }))
        }
    }
}