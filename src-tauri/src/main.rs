#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{SystemTray, SystemTrayMenu, SystemTrayEvent, Manager};
use tauri_plugin_positioner::{WindowExt, Position};
#[allow(unused_imports)]
use window_vibrancy::{apply_vibrancy, apply_blur, NSVisualEffectMaterial, NSVisualEffectState};

mod quakes;

#[tauri::command]
fn get_last_five_events() -> String {
  let result = quakes::afad::get_last_five_events().unwrap();
  return result;
}

#[tauri::command]
fn exit_request () {
  std::process::exit(0);
}

#[tauri::command]
async fn fetch_quakes() -> String {
  let result = quakes::kandilli::get_lastest_events().unwrap(); 
  return serde_json::to_string(&result).unwrap();
}

fn main() {
  let tray_menu = SystemTrayMenu::new();
  let system_tray = SystemTray::new().with_menu(tray_menu);

  tauri::Builder::default()
    .plugin(tauri_plugin_positioner::init())
    .setup(|app| {
      let window = app.get_window("main").unwrap();
      app.set_activation_policy(tauri::ActivationPolicy::Accessory);
      #[cfg(target_os = "macos")]
      apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
    
      #[cfg(target_os = "windows")]
      apply_blur(&window, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

      Ok(())
    })
    .system_tray(system_tray)
    .on_system_tray_event(|app, event| {
      tauri_plugin_positioner::on_tray_event(app, &event);
    })
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick { .. } => {
        let window = app.get_window("main").unwrap();
        let is_visible = window.is_visible().unwrap();
        match is_visible {
            true => {
              window.hide().unwrap();
            },
            false => {
              window.move_window(Position::TrayLeft).unwrap();
              window.show().unwrap();
              window.set_focus().unwrap();
            }
        }
      }
      _ => {}
    })
    .on_window_event(|event| match event.event() {
        // Hide outside click focus
        tauri::WindowEvent::Focused(is_focused) => {
            if !is_focused {
                event.window().hide().unwrap();
            }
        }
        _ => {}
    })
    .invoke_handler(tauri::generate_handler![get_last_five_events, fetch_quakes, exit_request])
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(|app_handle, event| match event {
      tauri::RunEvent::ExitRequested { api, .. } => {
        api.prevent_exit();
      },
      tauri::RunEvent::WindowEvent {
        label,
        event: win_event,
        ..
      } => match win_event {
          tauri::WindowEvent::CloseRequested { api, .. } => {
              let win = app_handle.get_window(label.as_str()).unwrap();
              win.hide().unwrap();
              api.prevent_close();
          }
          _ => {}
      },
      _ => {}
    });
    
}

// fn _toggle_action(app: &AppHandle, id: String) {
//   match id.as_str() {
//     "quit" => {
//       std::process::exit(0);
//     }
//     "hide" => {
//       let window = app.get_window("main").unwrap();
//       let item_handle = app.tray_handle().get_item(&id);
//       let is_visible = window.is_visible().unwrap();
//       match is_visible {
//           true => {
//             window.hide().unwrap();
//             item_handle.set_title("Göster").unwrap()
//           },
//           false => {
//             window.move_window(Position::TrayLeft).unwrap();
//             window.show().unwrap();
//             item_handle.set_title("Gizle").unwrap()
//           }
//       }
//     }
//     _ => {}
//   }
// }