use iced::{Subscription, keyboard::{self, Key, key::Named}};

use crate::{DandanLauncher, Message, ui::UserInterface};


impl UserInterface {
    pub fn subscribe(_state: &DandanLauncher) -> Subscription<Message> {
        keyboard::listen().filter_map(|event| {
            match event {
                keyboard::Event::KeyPressed { key, modifiers, .. } => {
                    match key {
                        Key::Named(Named::Enter) => {
                            Some(Message::ExecuteAction)
                        },
                        Key::Named(Named::Escape) => {
                            Some(Message::Exit)
                        },
                        Key::Character(ch) if ch == "q" && modifiers.command() => {
                            Some(Message::Exit)
                        },
                        Key::Character(ch) if ch == "n" && modifiers.command() => {
                            Some(Message::NavigationDown)
                        },
                        Key::Named(Named::ArrowDown) => {
                            Some(Message::NavigationDown)
                        },
                        Key::Character(ch) if ch == "p" && modifiers.command() => {
                            Some(Message::NavigationUp)
                        },
                        Key::Named(Named::ArrowUp) => {
                            Some(Message::NavigationUp)
                        },
                        _ => None
                    }
                }
                _ => None,
            }
        })
    }
}
