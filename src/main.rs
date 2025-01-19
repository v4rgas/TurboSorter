use std::{collections::HashMap, fs::DirEntry};

#[derive(Default)]
struct FileManager {
    registry_to_folder: HashMap<String, DirEntry>,
    pub current_entries: Vec<DirEntry>,
}

impl FileManager {
    fn update_entries(&mut self, path: &str) {
        self.current_entries = std::fs::read_dir(path)
            .unwrap()
            .map(|entry| entry.unwrap())
            .collect();
    }

    fn get_entries_as_string_vec(&self) -> Vec<String> {
        self.current_entries
            .iter()
            .map(|entry| entry.path().to_str().unwrap().to_string())
            .collect()
    }

    fn attach_folder_to_registry(&mut self, folder_name: &str, folder: DirEntry) {
        self.registry_to_folder
            .insert(folder_name.to_string(), folder);
    }

    fn get_folders(&self) -> Vec<DirEntry> {
        self.current_entries
            .iter()
            .filter(|entry| entry.path().is_dir())
            .map(|entry| entry.to_owned())
            .collect()
    }
}

fn main() {
    let mut file_manager = FileManager::default();
    file_manager.update_entries(".");

    println!("{:?}", file_manager.get_entries_as_string_vec());
}
