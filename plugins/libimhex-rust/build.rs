fn main() {
    println!("cargo:rustc-link-lib=dylib=imhex");
    println!("cargo:rustc-link-search=all={}", env!("LIBIMHEX_OUTPUT_DIRECTORY"));

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/imhex_api.rs");
    println!("cargo:rerun-if-changed=src/content_registry.rs");

    cxx_build::bridge("src/imhex_api.rs")
        .include(format!("{}/include", env!("LIBIMHEX_SOURCE_DIRECTORY")))
        .flag_if_supported("-std=gnu++20")
        .flag_if_supported("-std=gnu++2a")
        .flag_if_supported("-fconcepts")
        .compiler(env!("CXX_COMPILER"))
        .compile("libimhex-bridge");

    cxx_build::bridge("src/content_registry.rs")
        .include(format!("{}/include", env!("LIBIMHEX_SOURCE_DIRECTORY")))
        .include(format!("{}/nlohmann_json/include", env!("EXTERNAL_DIRECTORY")))
        .include(format!("{}/cxx.rs/include", env!("EXTERNAL_DIRECTORY")))
        .flag_if_supported("-std=gnu++20")
        .flag_if_supported("-std=gnu++2a")
        .flag_if_supported("-fconcepts")
        .compiler(env!("CXX_COMPILER"))
        .compile("libimhex-bridge");
}