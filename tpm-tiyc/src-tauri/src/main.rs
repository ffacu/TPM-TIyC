// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ptr::null;

fn main() {
    tpm_tiyc_lib::run()
}
