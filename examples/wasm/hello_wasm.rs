extern crate console_error_panic_hook;
use bevy::prelude::*;
use std::panic;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).expect("cannot initialize console_log");

    App::build().add_system(hello_wasm_system.system()).run();
}

fn hello_wasm_system() {
    log::info!("hello wasm");
}
