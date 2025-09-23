#[cfg(target_arch = "wasm32")]
mod wasm_web {
    use rust_graphics_play_core::run_web;
    
    // Re-export the web entry point
    pub use rust_graphics_play_core::run_web;
}

#[cfg(target_arch = "wasm32")]
pub use wasm_web::*;

#[cfg(not(target_arch = "wasm32"))]
pub fn run_web() {
    panic!("This crate should only be used for WASM targets");
}


