#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, Manager};

#[tauri::command]
fn minimize_to_tray(window: tauri::Window) {
    window.hide().unwrap();
}

#[tauri::command]
fn show_from_tray(window: tauri::Window) {
    window.show().unwrap();
    window.set_always_on_top(true).unwrap();
}

fn main() {
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.set_always_on_top(true).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![minimize_to_tray, show_from_tray])
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "show" => {
                        let window = app.get_window("main").unwrap();
                        window.show().unwrap();
                        window.set_always_on_top(true).unwrap();
                    },
                    "quit" => {
                        std::process::exit(0);
                    },
                    _ => {},
                }
            },
            _ => {},
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}