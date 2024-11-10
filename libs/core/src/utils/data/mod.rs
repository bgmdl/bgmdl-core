use std::{fs, path::Path};

pub fn exist(path: &str) -> bool {
    let path = Path::new(path);
    path.exists()
}

pub fn init(path: &str) -> std::io::Result<()> {
    let path = Path::new(path);
    if !path.exists() {
        fs::create_dir_all(path)?;
        fs::write(path.join("config.json"), "{}")
    } else {
        Ok(())
    }
}