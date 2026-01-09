use iced::{Task, clipboard, widget::operation::AbsoluteOffset};

use crate::{DandanLauncher, Message, ProviderResult, ui::UserInterface};


impl UserInterface {
    pub fn update(state: &mut DandanLauncher, message: Message) -> Task<Message> {
        match message {
            Message::EngineReady(engine) => { 
                state.engine = Some(engine);
                iced::widget::operation::focus(state.query_id.clone())
            },
            Message::InputChanged(query) => {
                if let Some(engine) = &state.engine {
                    state.result = engine.search(&query).unwrap_or(ProviderResult::None);
                }
                state.query = query;
                state.selected_index = 0;
                iced::widget::operation::scroll_to(state.scrollable_id.clone(), AbsoluteOffset { x: 0.0, y: 0.0 })
            }
            Message::ExecuteAction => {
                match &state.result {
                    ProviderResult::Gitmoji(gitmojis) => {
                        if let Some(gitmoji) = &gitmojis.get(state.selected_index as usize) {
                            return clipboard::write(format!("{} :: ", gitmoji.emoji))
                                .chain(Task::future(async {
                                    tokio::time::sleep(std::time::Duration::from_millis(100)).await
                                }))
                            .map(|_| Message::Exit);
                        }
                        Task::none()
                    },
                    ProviderResult::None => {
                        Task::none()
                    }
                }
            },
            Message::NavigationUp => {
                state.try_select_previous();
                let item_hight: f32 = 50.0;
                let offset = state.selected_index as f32 * item_hight;
                iced::widget::operation::scroll_to(state.scrollable_id.clone(), AbsoluteOffset { x: 0.0, y: offset })
            },
            Message::NavigationDown => {
                state.try_select_next();
                let item_height: f32 = 50.0;
                let offset = state.selected_index as f32 * item_height;
                iced::widget::operation::scroll_to(state.scrollable_id.clone(), AbsoluteOffset { x: 0.0, y: offset })
            },
            Message::Exit => {
                iced::exit()
            }
        }
    }
}
