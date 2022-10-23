use std::{path::Path, fs, cmp::Ordering};
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
    println!("Listing files in directory. dir: {}", dir.display().to_string());
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