// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    log::info!("Starting SymCalc2");
    symcalc2_lib::run()
}
