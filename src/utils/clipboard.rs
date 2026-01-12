use std::process::Command;

use crate::providers::ClipboardEntry;

#[derive(Debug)]
pub struct ClipboardUtils { }

impl ClipboardUtils {
    pub fn list() -> Option<Vec<ClipboardEntry>> {
        let output = Command::new("cliphist")
            .arg("list")
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }


        let stdout = String::from_utf8_lossy(&output.stdout);

        let mut entries: Vec<ClipboardEntry> = vec![]; 

        for line in stdout.lines() {
            let mut parts = line.splitn(2, '\t');
            let id = parts.next()?.parse::<usize>().ok()?;
            let content = parts.next()?.to_string();

            entries.push(ClipboardEntry { id, content });
        }

        println!("{:?}", entries);

        Some(entries)
    }

    pub fn copy(id: &usize) {
        let _ = Command::new("sh")
            .arg("-c")
            .arg(format!( "cliphist decode {} | wl-copy", id))
            .spawn();
    }
}
