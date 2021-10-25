pub mod ffi {
    #[cxx::bridge]
    pub mod RustUtils {

        #[namespace = "hex"]
        pub struct View { placeholder: u8 }

        #[namespace = "hex::rust"]
        extern "C++" {
            include!("rust_utils.hpp");

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

/*
,
               destructorFunc: fn(),
               drawContentFunc: fn(& mut bool),
               drawAlwaysVisibleFunc: fn(),
               drawMenuFunc: fn(),
               isAvailableFunc: fn() -> bool,
               shouldProcessFunc: fn() -> bool,
               hasViewMenuItemEntryFunc: fn() -> bool) -> * mut Self

        unsafe {
            cxx::let_cxx_string!(cpp_name = unlocalized_name);
            crate::rust_utils::ffi::RustUtils::createRustView(&cpp_name,
                                                                                 destructorFunc,
                                                                                 drawContentFunc,
                                                                                 drawAlwaysVisibleFunc,
                                                                                 drawMenuFunc,
                                                                                 isAvailableFunc,
                                                                                 shouldProcessFunc,
                                                                                 hasViewMenuItemEntryFunc)
        }
 */

pub struct ViewBuilder {
    unlocalized_name: String,

    destructorFunc: Option<fn()>,
    drawContentFunc: Option<fn(& mut bool)>,
    drawAlwaysVisibleFunc: Option<fn()>,
    drawMenuFunc: Option<fn()>,
    isAvailableFunc: Option<fn() -> bool>,
    shouldProcessFunc: Option<fn() -> bool>,
    hasViewMenuItemEntryFunc: Option<fn() -> bool>
}

impl ViewBuilder {

    pub fn new(unlocalized_name: &str) -> Self {
        ViewBuilder {
            unlocalized_name: String::from(unlocalized_name),

            destructorFunc: None,
            drawContentFunc: None,
            drawAlwaysVisibleFunc: None,
            drawMenuFunc: None,
            isAvailableFunc: None,
            shouldProcessFunc: None,
            hasViewMenuItemEntryFunc: None
        }
    }

    pub fn destructor(&mut self, func: fn()) -> &mut Self {
        self.destructorFunc = Some(func);

        self
    }

    pub fn drawContent(&mut self, func: fn(& mut bool)) -> &mut Self {
        self.drawContentFunc = Some(func);

        self
    }

    pub fn drawAlwaysVisible(&mut self, func: fn()) -> &mut Self {
        self.drawAlwaysVisibleFunc = Some(func);

        self
    }

    pub fn drawMenu(&mut self, func: fn()) -> &mut Self {
        self.drawMenuFunc = Some(func);

        self
    }

    pub fn isAvailable(&mut self, func: fn() -> bool) -> &mut Self {
        self.isAvailableFunc = Some(func);

        self
    }

    pub fn shouldProcess(&mut self, func: fn() -> bool) -> &mut Self {
        self.shouldProcessFunc = Some(func);

        self
    }

    pub fn hasViewMenuItemEntry(&mut self, func: fn() -> bool) -> &mut Self {
        self.hasViewMenuItemEntryFunc = Some(func);

        self
    }

    pub fn build(&mut self) -> *mut ffi::RustUtils::View {
        unsafe {
            cxx::let_cxx_string!(cpp_name = &self.unlocalized_name);
            crate::rust_utils::ffi::RustUtils::createRustView(&cpp_name,
                                                              self.destructorFunc.is_some(),
                                                              self.destructorFunc.unwrap_or(||{}),
                                                              self.drawContentFunc.is_some(),
                                                              self.drawContentFunc.unwrap_or(|_: &mut bool|{}),
                                                              self.drawAlwaysVisibleFunc.is_some(),
                                                              self.drawAlwaysVisibleFunc.unwrap_or(||{}),
                                                              self.drawMenuFunc.is_some(),
                                                              self.drawMenuFunc.unwrap_or(||{}),
                                                              self.isAvailableFunc.is_some(),
                                                              self.isAvailableFunc.unwrap_or(||{ false }),
                                                              self.shouldProcessFunc.is_some(),
                                                              self.shouldProcessFunc.unwrap_or(||{ false }),
                                                              self.hasViewMenuItemEntryFunc.is_some(),
                                                              self.hasViewMenuItemEntryFunc.unwrap_or(||{ false }))
        }
    }
}