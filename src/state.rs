use iced::{Element, Subscription, Task};

use crate::{Message, ProviderResult, engine::Engine};


#[derive(Debug)]
pub struct DandanLauncher {
    pub query: String,
    pub query_id: iced::widget::Id,
    pub engine: Option<Engine>,
    pub result: ProviderResult,
    pub selected_index: usize,
    pub scrollable_id: iced::widget::Id
}

impl DandanLauncher {

    fn new() -> Self {
        DandanLauncher {
            query: String::new(),
            query_id: iced::widget::Id::new("search_id"),
            engine: None,
            result: ProviderResult::None,
            selected_index: 0,
            scrollable_id: iced::widget::Id::new("scrollable_id")
        }
    }

    pub fn init() -> (Self, Task<Message>) {
        (DandanLauncher::new(), Task::perform(Engine::create_loaded(), Message::EngineReady))
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        crate::ui::UserInterface::update(self, message)
    }

    pub fn view(&self) -> Element<'_, Message> {
        crate::ui::UserInterface::render(self)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        crate::ui::UserInterface::subscribe(self)
    }


    pub fn try_select_next(&mut self) {
        match &self.result {
            ProviderResult::None => { self.selected_index = 0; },
            ProviderResult::Calculator(_) => { self.selected_index = 0; },
            ProviderResult::Gitmoji(gitmojis) => {
                if self.selected_index + 1 > gitmojis.len() - 1 {
                    self.selected_index = 0
                } else {
                    self.selected_index += 1;
                }
            },
            ProviderResult::Clipboard(entries) => {
                if self.selected_index + 1 > entries.len() - 1 {
                    self.selected_index = 0
                } else {
                    self.selected_index += 1;
                }
            },
            ProviderResult::Apps(apps) => {
                if self.selected_index + 1 > apps.len() - 1 {
                    self.selected_index = 0
                } else {
                    self.selected_index += 1;
                }
            }
        }
    }

    pub fn try_select_previous(&mut self) {
        match &self.result {
            ProviderResult::None => { self.selected_index = 0; },
            ProviderResult::Calculator(_) => { self.selected_index = 0; },
            ProviderResult::Gitmoji(gitmojis) => {
                if self.selected_index == 0 {
                    self.selected_index = gitmojis.len() - 1;
                } else {
                    self.selected_index -= 1;
                }
            },
            ProviderResult::Clipboard(entries) => {
                if self.selected_index == 0 {
                    self.selected_index = entries.len() - 1;
                } else {
                    self.selected_index -= 1;
                }
            },
            ProviderResult::Apps(apps) => {
                if self.selected_index == 0 {
                    self.selected_index = apps.len() - 1;
                } else {
                    self.selected_index -= 1;
                }
            }
        }
    }

}
