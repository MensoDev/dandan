use iced::{Alignment, Color, Element, Font, Length::{self, Fill}, Theme, font::Weight, widget::{column, container, row, scrollable, text, text_input}};

use crate::{DandanLauncher, Message, ProviderResult, providers::Gitmoji, ui::UserInterface};

impl UserInterface {
    pub fn render(state: &DandanLauncher) -> Element<'_, Message> {
        let input = text_input("search", &state.query)
            .on_input(Message::InputChanged)
            .id(state.query_id.clone())
            .padding(10);

        let mut col = column![input].spacing(8);

        match &state.result {
            ProviderResult::Gitmoji(emojis) => {
                let items = gitmojis_view(emojis, state.selected_index, state.scrollable_id.clone());
                col = col.push(items);
            }
            _ => {
                let h = text("type something to search anything kkkk");
                col = col.push(h);
            }
        }

        col.into()
    }
}

fn gitmojis_view(emojis: &[Gitmoji], selected_index: usize, scrollable_id: iced::widget::Id) -> Element<'_, Message> {
    let mut col = column![];
    for (index, gitmoji) in emojis.iter().enumerate() {
        col = col.push(gitmoji_view(gitmoji, index == selected_index as usize));
    }
    scrollable(col).height(Fill).width(Fill).id(scrollable_id).into()
}

fn gitmoji_view(gitmoji: &Gitmoji, selected: bool) -> Element<'_, Message> {
    let icon = container(
        text(&gitmoji.emoji)
        .size(20)
    )
        .width(50)
        .height(24)
        .align_y(Alignment::Center)
        .align_x(Alignment::Center);

    let title = text(format!("{}", gitmoji.name))
        .size(16)
        .font(Font {
            weight: Weight::Bold,
            ..Font::DEFAULT
        });

    let desc = text(&gitmoji.description)
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
