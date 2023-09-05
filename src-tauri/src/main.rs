// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod process_detail;

mod spotlight;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // tauri_plugin_deep_link::prepare("com.test.dev");

    // let mut app = tauri::Builder::default()

    //     .invoke_handler(tauri::generate_handler![
    //         spotlight::init_spotlight_window,
    //         spotlight::show_spotlight,
    //         spotlight::hide_spotlight
    //     ])
    //     .manage(spotlight::State::default())
    //     .invoke_handler(tauri::generate_handler![greet, start_server])
    //     .build(tauri::generate_context!())
    //     .expect("error while building tauri application");
    // // let ee = app.get_window("main").unwrap();

    // // ee.hide();

    // // app.set_activation_policy(activation_policy);
    // app.run(|_app_handle, event| match event {
    //     tauri::RunEvent::ExitRequested { api, .. } => {
    //         // api.prevent_exit();
    //         // _app_handle.get_focused_window()
    //     }
    //     _ => {}
    // });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            spotlight::init_spotlight_window,
            spotlight::show_spotlight,
            spotlight::hide_spotlight
        ])
        .manage(spotlight::State::default())
        .setup(move |app| {
            // Set activation poicy to Accessory to prevent the app icon from showing on the dock
            // app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
