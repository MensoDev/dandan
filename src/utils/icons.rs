use std::path::{Path, PathBuf};

const THEMES: [&str; 5] = [
    "hicolor",
    "locolor",
    "Adwaita",
    "Papirus",
    "breeze"
];

const SIZES: [&str; 6] = [
    "scalable",
    "symbolic",
    "48x48",
    "32x32",
    "24x24",
    "22x22"
];

const SIZES_LEGACY: [&str; 4] = [
    "48",
    "38",
    "24",
    "22",
];

const CONTEXTS: [&str; 6] = [
    "apps",
    "devices",
    "emblems",
    "categories",
    "actions",
    "places",
];

const EXTENSIONS: [&str; 2] = ["svg", "png"];

#[derive(Debug)]
pub struct IconsUtils {
}

impl IconsUtils {

    pub fn resolve(icon: &str) -> Option<PathBuf> {
        let icon_path = Path::new(icon);

        if icon_path.is_absolute() && icon_path.exists() {
            return Some(icon_path.to_path_buf());
        }

        let r = IconsUtils::resolve_current(icon);
        if r.is_some() { return r };

        IconsUtils::resolve_legacy(icon)
    }

    fn resolve_legacy(icon: &str) -> Option<PathBuf> {
        let mut icon_dirs = Vec::new();

        icon_dirs.push(Path::new("/usr/share/icons/").to_path_buf());

        let mut local_dir = dirs::home_dir().unwrap();
        local_dir.push(".local/share/icons");
        icon_dirs.push(local_dir);

        for base in &icon_dirs {
            for theme in &THEMES {
                for ctx in &CONTEXTS {
                    for size in &SIZES_LEGACY {
                        let dir = base.join(theme).join(size).join(ctx);
                        for ext in &EXTENSIONS {
                            let candidate = dir.join(format!("{icon}.{ext}"));
                            if candidate.exists() {
                                return Some(candidate);
                            }
                        }
                    }
                }
            }
        }

        None
    }

    fn resolve_current(icon: &str) -> Option<PathBuf> {

        let mut icon_dirs = Vec::new();

        icon_dirs.push(Path::new("/usr/share/icons/").to_path_buf());

        let mut local_dir = dirs::home_dir().unwrap();
        local_dir.push(".local/share/icons");
        icon_dirs.push(local_dir);

        for base in &icon_dirs {
            for theme in &THEMES {
                for size in &SIZES {
                    for ctx in &CONTEXTS {
                        let dir = base.join(theme).join(size).join(ctx);
                        for ext in &EXTENSIONS {
                            let candidate = dir.join(format!("{icon}.{ext}"));
                            if candidate.exists() {
                                return Some(candidate);
                            }
                        }
                    }
                }
            }
        }

        None
    }
}

