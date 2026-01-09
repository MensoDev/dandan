use std::path::{Path, PathBuf};


pub fn get_config_path(filename: &str) -> Option<PathBuf> {

    if let Some(mut config_path) = dirs::config_dir() {

        config_path.push("dandan");
        config_path.push(filename);

        if config_path.exists() {
            return Some(config_path);
        }
    }

    let local_path = Path::new(filename);
    if local_path.exists() {
        return Some(local_path.to_path_buf());
    }

    None
}
