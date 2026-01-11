use std::{fs::{self, File}, io::{self, BufRead, BufReader}, path::{PathBuf}};

use crate::{ProviderResult, engine::Provider};

const ENTRY_TOKEN: &str = "[Desktop Entry]";
const ACTION_TOKEN: &str = "[Desktop Action";

const NAME_TOKEN: &str = "Name=";
const COMMENT_TOKEN: &str = "Comment=";
const EXEC_TOKEN: &str = "Exec=";
const ICON_TOKEN: &str = "Icon=";
const NO_DISPLAY_TOKEN: &str = "NoDisplay=";
const TERMINAL: &str = "Terminal=";

#[derive(Debug)]
pub struct ApplicationProvider {
    entries: Option<Vec<DesktopEntry>>,
}

impl ApplicationProvider {
    pub fn new() -> Self {
        ApplicationProvider {
            entries: None
        }
    }
}

impl Provider for ApplicationProvider {

    fn search(&self, query: &str) -> Option<ProviderResult> {

        if let Some(entries) = &self.entries {
            let r = entries
                .iter()
                .filter(|e| e.action.name.to_lowercase().contains(&query.to_lowercase()))
                .cloned()
                .collect();

            return Some(ProviderResult::Apps(r));
        }

        None
    }

    fn init(&mut self) {
        self.entries = Some(load());
    }

}

#[derive(Debug, Clone)]
pub struct DesktopEntry {
    pub path: PathBuf,
    pub action: DesktopAction,
    pub actions: Vec<DesktopAction>,
}

#[derive(Default, Debug, Clone)]
pub struct DesktopAction {
    pub name: String,
    pub displayable: bool,
    pub terminal: bool,
    pub exec: String,
    pub icon: String,
    pub icon_path: Option<PathBuf>,
    pub comment: String,
}

impl DesktopEntry {

    pub fn create(path: &PathBuf) -> io::Result<Self> {
        let (action, actions) = create_desktop_action(path)?;
        Ok(Self { path: path.to_path_buf(), action, actions })
    }
}

fn load() -> Vec<DesktopEntry> {

    let mut entries: Vec<DesktopEntry> = vec![];

    parse("/home/menso/.local/share/applications", &mut entries);
    parse("/usr/local/share/applications", &mut entries);
    parse("/usr/share/applications", &mut entries);

    entries
}

fn parse(path: &str, entries: &mut Vec<DesktopEntry>) {

    if let Ok(paths) = fs::read_dir(path) {
        for path in paths {

            let path = path.unwrap();
            let path = path.path();
            let extension = path.extension().unwrap().to_str().unwrap();

            if extension != "desktop" { continue; }

            let entry = DesktopEntry::create(&path).unwrap();
            if !entry.action.displayable { continue; }

            entries.push(entry);
        }
    }
}


fn create_desktop_action(path: &PathBuf) -> io::Result<(DesktopAction, Vec<DesktopAction>)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut entry = DesktopAction { name: String::new(), displayable: true, terminal: false, exec: String::new(), icon: String::new(), comment: String::new(), icon_path: None };
    let mut actions: Vec<DesktopAction> = vec![];

    let mut section = Section::None;
    let mut temp = DesktopAction { name: String::new(), displayable: true, terminal: false, exec: String::new(), icon: String::new(), comment: String::new(), icon_path: None };

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') { continue; }

        if line.starts_with(ENTRY_TOKEN) {
            flush(&mut temp, &mut entry, &mut actions, &section);
            section = Section::Entry;
            continue;
        }

        if line.starts_with(ACTION_TOKEN) {
            flush(&mut temp, &mut entry, &mut actions, &section);
            section = Section::Action;
            continue;
        }

        if let Some(name) = line.strip_prefix(NAME_TOKEN) {
            temp.name = name.to_string();
        } else if let Some(comment) = line.strip_prefix(COMMENT_TOKEN) {
            temp.comment = comment.to_string();
        } else if let Some(icon) = line.strip_prefix(ICON_TOKEN) {
            temp.icon = icon.to_string();
            temp.icon_path = crate::IconsUtils::resolve(icon);
        } else if let Some(exec) = line.strip_prefix(EXEC_TOKEN) {
            temp.exec = exec.to_string();
        } else if let Some(no_display) = line.strip_prefix(NO_DISPLAY_TOKEN) {
            temp.displayable = !no_display.parse::<bool>().unwrap_or(false);
        } else if let Some(terminal) = line.strip_prefix(TERMINAL) {
            temp.terminal = terminal.parse::<bool>().unwrap_or(false);
        }
    }

    flush(&mut temp, &mut entry, &mut actions, &section);

    Ok((entry, actions))
}

fn flush(
    temp: &mut DesktopAction,
    entry: &mut DesktopAction,
    actions: &mut Vec<DesktopAction>,
    section: &Section,
) {
    match section {
        Section::Entry => { *entry = std::mem::take(temp); },
        Section::Action => { actions.push(std::mem::take(temp)); },
        Section::None => {}
    }
}

enum Section {
    None,
    Entry,
    Action,
}
