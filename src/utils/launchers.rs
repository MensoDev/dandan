use std::{process::{Command, Stdio}};

#[derive(Debug)]
pub struct LauncherUtils { }

impl LauncherUtils {

    pub fn copy(text: &str) {
        let _ = Command::new("wl-copy")
            .arg(&text)
            .spawn();
    }

    pub fn run(command: &str, use_terminal: bool) {

        let cleaned_command = command
            .split_whitespace()
            .filter(|part| !part.starts_with('%'))
            .collect::<Vec<&str>>()
            .join(" ");

        let final_command = if use_terminal {
            let terminal = get_terminal_emulator();
            // usa a flag '-e' ou '--' para executar um comando.
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
            Ok(_) => println!("🚀 Success: {}", final_command),
            Err(e) => eprintln!("❌ Error: {}", e),
        }
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

