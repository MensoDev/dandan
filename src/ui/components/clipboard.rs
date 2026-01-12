use iced::{Alignment, Color, Element, Font, Length::{self, Fill}, Theme, font::{Style, Weight}, widget::{ column, container, row, scrollable, text }};

use crate::{Message, providers::ClipboardEntry, ui::components::Components};


impl Components {

    pub fn clipboard_items(entries: &[ClipboardEntry], selected_index: usize, scrollable_id: iced::widget::Id) -> Element<'_, Message> {
        let mut col = column![];
        for (index, app) in entries.iter().enumerate() {
            col = col.push(Components::clipboard_item(app, index == selected_index as usize));
        }
        scrollable(col).height(Fill).width(Fill).id(scrollable_id).into()
    }

    fn clipboard_item(entry: &ClipboardEntry, selected: bool) -> Element<'_, Message> {

        let icon = container(
            text("📋")
            .size(20)
        )
            .width(50)
            .height(24)
            .align_y(Alignment::Center)
            .align_x(Alignment::Center);

        let title = text(format!("{}", entry.content))
            .size(16)
            .wrapping(text::Wrapping::None)
            .font(Font {
                weight: Weight::Bold,
                ..Font::DEFAULT
            });

        let desc = text(&entry.id)
            .size(13)
            .style(|theme: &Theme| {
                let palette = theme.palette();
                text::Style {
                    color: Some(palette.text.clone().into()),
                }
            });

        let info = column![title, desc].width(Length::Fill);

        let content = row![icon, info].align_y(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(50)
            .max_height(50)
            .align_y(Alignment::Center)
            .style(move |theme: &Theme| {
                let palette = theme.palette();
                if selected {
                    container::Style {
                        background: Some(palette.primary.into()),
                        text_color: Some(Color::WHITE),
                        ..container::Style::default()
                    }
                } else {
                    container::Style {
                        background: Some(palette.background.into()),
                        ..container::Style::default()
                    }
                }
            })
        .into()
    }
}

