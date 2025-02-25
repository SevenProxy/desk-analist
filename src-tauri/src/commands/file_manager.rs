use std::fs::{self, ReadDir};
use std::io::Error;
use std::path::Path;

#[tauri::command]
pub fn list_files() -> Result<Vec<String>, String> {
  let path: &Path = Path::new("C:\\proxy\\Documents");

  let entries: ReadDir = fs::read_dir(path)
    .map_err(|e: Error| format!("Erro ao ler diretório: {}", e))?;

  let files: Vec<String> = entries
    .filter_map(|entry| entry.ok())
    .filter_map(|entry| entry.file_name().into_string().ok())
    .collect();

  Ok(vec![
    String::from("aaaa")
  ])
}
