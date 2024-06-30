#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    CustomMenuItem, GlobalShortcutManager, LogicalSize, Manager, PhysicalPosition, SystemTray,
    SystemTrayEvent, SystemTrayMenu,
};

#[tauri::command]
fn minimize_to_tray(window: tauri::Window) {
    window.hide().unwrap();
}

#[tauri::command]
fn show_from_tray(window: tauri::Window) {
    if let Some(monitor) = window.primary_monitor().unwrap() {
        let screen_size = monitor.size();
        let window_size: LogicalSize<f64> = window
            .outer_size()
            .unwrap()
            .to_logical::<f64>(monitor.scale_factor());
        let x = (screen_size.width as f64 - window_size.width) / 2.0;
        let y = (screen_size.height as f64 - window_size.height) / 2.0;

        window
            .set_position(PhysicalPosition {
                x: x as i32,
                y: y as i32,
            })
            .unwrap();
        window.show().unwrap();
        window.set_always_on_top(true).unwrap();
    }
}

fn main() {
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(show).add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.set_always_on_top(true).unwrap();

            // Register the global shortcut
            let app_handle = app.handle();
            app.global_shortcut_manager()
                .register("Ctrl+Space", move || {
                    let window = app_handle.get_window("main").unwrap();

                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        if let Some(monitor) = window.primary_monitor().unwrap() {
                            let screen_size = monitor.size();
                            let window_size: LogicalSize<f64> = window
                                .outer_size()
                                .unwrap()
                                .to_logical::<f64>(monitor.scale_factor());
                            let x = (screen_size.width as f64 - window_size.width) / 2.0;
                            let y = (screen_size.height as f64 - window_size.height) / 2.0;

                            window
                                .set_position(PhysicalPosition {
                                    x: x as i32,
                                    y: y as i32,
                                })
                                .unwrap();
                            window.show().unwrap();
                            window.set_always_on_top(true).unwrap();
                        }
                    }
                })
                .expect("Failed to register global shortcut");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![minimize_to_tray, show_from_tray])
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "show" => {
                    let window = app.get_window("main").unwrap();

                    if let Some(monitor) = window.primary_monitor().unwrap() {
                        let screen_size = monitor.size();
                        let window_size: LogicalSize<f64> = window
                            .outer_size()
                            .unwrap()
                            .to_logical::<f64>(monitor.scale_factor());
                        let x = (screen_size.width as f64 - window_size.width) / 2.0;
                        let y = (screen_size.height as f64 - window_size.height) / 2.0;

                        window
                            .set_position(PhysicalPosition {
                                x: x as i32,
                                y: y as i32,
                            })
                            .unwrap();
                        window.show().unwrap();
                        window.set_always_on_top(true).unwrap();
                    }
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
