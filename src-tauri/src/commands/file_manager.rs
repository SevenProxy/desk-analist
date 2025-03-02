use std::fs::{self};
use std::path::Path;

#[derive(serde::Serialize)]
pub struct DirEntry {
  name: String,
  is_dir: bool,
}

#[tauri::command]
pub fn list_files() -> Result<Vec<DirEntry>, String> {
  let path: &Path = Path::new("C:\\proxy\\Documents");
  if !path.exists() || !path.is_dir() {
    return Err("O caminho fornecido não é um diretório válido.".to_string());
  }

  let mut entries = Vec::new();

  for entry in fs::read_dir(path).map_err(|e| e.to_string()) ? {
    let entry = entry.map_err(|e| e.to_string())?;
    let path = entry.path();
    let name = path.file_name().unwrap().to_string_lossy().to_string();
    let is_dir = path.is_dir();

    entries.push(DirEntry { name, is_dir });
  }

  Ok(entries)
}
