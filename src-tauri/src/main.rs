// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::{error};
use std::panic;


fn main() {
     panic::set_hook(Box::new(|info| {
        let msg = match info.payload().downcast_ref::<&str>() {
            Some(s) => *s,
            None => "panic desconhecido",
        };

        let location = info.location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_else(|| "localização desconhecida".into());

        error!("🛑 PANIC: {}\n📍 Local: {}", msg, location);
    }));

    
    manage_projects_lib::run()
}
