// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//test
//use tauri::menu::{Menu, MenuItem, MenuBuilder, Submenu};
//const quit = MenuItem::;

fn main() {
    chessero_lib::run()
}
