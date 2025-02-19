use std::{
    env,
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct FileManager(PathBuf);
impl FileManager {
    pub fn new(app_name: &str) -> Self {
        let home_dir = env::var("HOME").expect("Failed to get HOME directory");
        let path = PathBuf::from(home_dir).join(".local/share").join(app_name);
        fs::create_dir_all(&path).expect("Failed to create app data directory");
        Self(path)
    }

    pub fn save_to_file<T: Serialize + for<'a> Deserialize<'a>>(
        &self,
        data: &T,
        filename: &str,
    ) -> std::io::Result<()> {
        let encoded = bincode::serialize(data).expect("Failed to serialize");
        let mut file = File::create(self.0.join(filename))?;
        file.write_all(&encoded)?;
        Ok(())
    }

    pub fn load_from_file<T: Serialize + for<'a> Deserialize<'a>>(
        &self,
        filename: &str,
    ) -> std::io::Result<T> {
        let mut file = File::open(self.0.join(filename))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let decoded: T = bincode::deserialize(&buffer).expect("Failed to deserialize");
        Ok(decoded)
    }

    pub fn get_saved_files(&self) -> std::io::Result<Vec<String>> {
        let mut files = Vec::new();

        for entry in fs::read_dir(self.0.to_owned())? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    files.push(file_name.to_string());
                }
            }
        }

        Ok(files)
    }
}
