#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod yfile;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      yfile::list_files_in_directory,
      yfile::read_file,
      yfile::save_to_file,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
