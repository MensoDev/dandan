use iced::{Element, Subscription, Task};

use crate::Message;


#[derive(Debug)]
pub struct DandanLauncher {
    pub query: String,
    pub query_id: iced::widget::Id,
    pub selected_index: usize,
    pub scrollable_id: iced::widget::Id
}

impl DandanLauncher {

    fn new() -> Self {
        DandanLauncher {
            query: String::new(),
            query_id: iced::widget::Id::new("search_id"),
            selected_index: 0,
            scrollable_id: iced::widget::Id::new("scrollable_id")
        }
    }

    pub fn init() -> (Self, Task<Message>) {
        todo!();
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        todo!();
    }

    pub fn view(&self) -> Element<'_, Message> {
        todo!();
    }

    pub fn subscription(&self) -> Subscription<Message> {
        todo!();
    }

}
