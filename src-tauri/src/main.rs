#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::menu::{MenuBuilder, MenuId, MenuItemBuilder, SubmenuBuilder};
use tauri::{Emitter, Manager};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let about_id = MenuId::new("about");
      let refresh_id = MenuId::new("refresh");
      let close_id = MenuId::new("close");

      let about_item = MenuItemBuilder::with_id(about_id.clone(), "关于").build(app)?;
      let refresh_item = MenuItemBuilder::with_id(refresh_id.clone(), "强制刷新")
        .accelerator("cmdOrControl+R")
        .build(app)?;
      let close_item = MenuItemBuilder::with_id(close_id.clone(), "退出")
        .accelerator("cmdOrControl+Q")
        .build(app)?;

      let file_submenu = SubmenuBuilder::new(app, "File")
        .item(&about_item)
        .item(&refresh_item)
        .item(&close_item)
        .build()?;

      let menu = MenuBuilder::new(app).item(&file_submenu).build()?;

      app.set_menu(menu)?;

      app.on_menu_event(move |app, event| {
        if event.id() == &about_id {
          if let Some(window) = app.get_webview_window("main") {
            window.emit("about-event", "aboutClick").unwrap();
          }
        } else if event.id() == &refresh_id {
          if let Some(window) = app.get_webview_window("main") {
            window.eval("location.reload(true);").unwrap();
          }
        } else if event.id() == &close_id {
          if let Some(window) = app.get_webview_window("main") {
            window.close().unwrap();
          }
        }
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
