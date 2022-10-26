use std::{path::Path, cmp::Ordering, fs::{File, self}, io::{Read, Write}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct YFile {
    name: String,
    path: String,
    is_directory: bool,
    parent_path: String,
}

impl YFile {
    fn cmp(&self, other: &YFile) -> Ordering {
        if self.is_directory && other.is_directory {
            return self.name.cmp(&other.name);
        } else if self.is_directory {
            return Ordering::Less;
        } else if other.is_directory {
            return Ordering::Greater;
        } else {
            return self.name.cmp(&other.name);
        }
    }
}

fn create_by_dir_entry(entry: fs::DirEntry) -> YFile {
    return YFile {
        name: entry.file_name().into_string().unwrap(),
        path: entry.path().display().to_string(),
        is_directory: entry.path().is_dir(),
        parent_path: entry.path().parent().get_or_insert(Path::new("")).display().to_string(),
    };
}

#[tauri::command]
pub fn list_files_in_directory(dir: &Path) -> Result<Vec<YFile>, String> {
    if dir.is_dir() {
        let paths = fs::read_dir(dir).map_err(|err| err.to_string())?;
        let mut files = Vec::new();
        for entry in paths {
            files.push(create_by_dir_entry(entry.unwrap()));
        }
        files.sort_by(|x, y| x.cmp(&y));
        return Ok(files);
    }
    return Ok(Vec::new());
}

#[tauri::command]
pub fn read_file(filepath: &Path) -> Result<String, String> {
    if filepath.is_file() {
        let mut file = File::open(filepath).map_err(|err| err.to_string())?;
        let mut content  = String::new();
        file.read_to_string(&mut content).map_err(|err| err.to_string())?;
        return Ok(content);
    }
    return Err("read file failed!".to_string());
}

#[tauri::command]
pub fn save_to_file(filepath: &Path, content: String) -> Result<String, String> {
    println!("filepath: {}", filepath.display());
    if filepath.is_file() {
        let mut file = File::create(filepath).map_err(|err| err.to_string())?;
        file.write_all(content.as_bytes()).map_err(|err| err.to_string())?;
        file.flush().map_err(|err| err.to_string())?;
        return Ok("write to file success".to_string());
    }
    return Err("write to file failed!".to_string());
}
