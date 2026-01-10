use std::{path::{Path, PathBuf}, process::{Command, Stdio}};


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

pub fn copy(text: &str) {
    let _ = Command::new("wl-copy")
        .arg(&text)
        .spawn();
}

pub fn launcher(command: &str, use_terminal: bool) {

    let cleaned_command = command
        .split_whitespace()
        .filter(|part| !part.starts_with('%'))
        .collect::<Vec<&str>>()
        .join(" ");

    let final_command = if use_terminal {
        let terminal = get_terminal_emulator();
        format!("{} -e {}", terminal, cleaned_command)
    } else {
        cleaned_command
    };

    // 3. Execução via sh -c para garantir compatibilidade
    let process = Command::new("sh")
        .arg("-c")
        .arg(&final_command)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    match process {
        Ok(_) => println!("🚀 Lançado: {}", final_command),
        Err(e) => eprintln!("❌ Erro ao lançar: {}", e),
    }
}

fn get_terminal_emulator() -> String {
    let terminals = ["kitty", "foot", "alacritty", "gnome-terminal", "konsole", "xfce4-terminal"];
    
    for t in terminals {
        if Command::new("which").arg(t).stdout(Stdio::null()).status().is_ok() {
            return t.to_string();
        }
    }
    "xterm".to_string()
}

