use std::{collections::HashMap, fs::DirEntry};

#[derive(Default)]
struct FileManager {
    registry_to_folder: HashMap<String, String>,
    current_entries: Vec<DirEntry>,
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

    fn attach_folder_to_registry(&mut self, folder_name: &str, registry_name: &str) {
        if !self.is_folder_in_current_entries(folder_name) {
            panic!("Folder not found in current entries");
        }

        self.registry_to_folder
            .insert(registry_name.to_string(), folder_name.to_string());
    }

    fn is_folder_in_current_entries(&self, folder_name: &str) -> bool {
        self.current_entries
            .iter()
            .any(|entry| entry.path().to_str().unwrap() == folder_name)
    }

    fn get_all_folders(&self) -> Vec<String> {
        self.current_entries
            .iter()
            .filter(|entry| entry.path().is_dir())
            .map(|entry| entry.path().to_str().unwrap().to_string())
            .collect()
    }
    
}

fn main() {
    let mut file_manager = FileManager::default();
    file_manager.update_entries(".");

    println!("{:?}", file_manager.get_entries_as_string_vec());
    println!("{:?}", file_manager.get_all_folders());
    file_manager.attach_folder_to_registry("./src", "s");
    println!("{:?}", file_manager.registry_to_folder);

    
}
