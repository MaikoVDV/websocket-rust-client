use crate::*;

pub fn get_platform_graphics_api() -> Backends {
    // Setup native Graphics API for each platform.
    let platform_api = if cfg!(target_os = "windows") {
        Backends::DX12
    } else if cfg!(target_os = "macos") {
        Backends::METAL
    } else if cfg!(target_os = "linux") {
        Backends::GL
    } else {
        panic!("Unsupported platform!");
    };
    println!("Using {:?} as graphics api.", platform_api);
    return platform_api;
}