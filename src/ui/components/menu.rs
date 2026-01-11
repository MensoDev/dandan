use iced::{Alignment, Color, Element, Font, Length::Fill, widget::{ column, container, row, text }};

use crate::{DandanLauncher, Message, ui::components::Components};

impl Components {

    pub fn menu(_state: &DandanLauncher) -> Element<'_, Message> {

        let hello = text("Hi, type something to search apps")
            .size(18)
            .font(Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            });

        let col = column![
            hello, 
            hint("to search gitmojis", &':'), 
            hint("to calculate something", &'='), 
        ]
            .spacing(16)
            .width(Fill)
            .align_x(Alignment::Start);

        container(col)
            .padding(24)
            .width(Fill)
            .center_x(Fill)
            .into()
    }
}

fn hint<'a>(msg: &'a str, symbol: &'a char) -> Element<'a, Message> {
    row![
        hint_symbol(&symbol),
        text(msg)
            .size(14)
            .color(Color::from_rgb(0.5, 0.5, 0.5))
    ]
    .spacing(8)
    .align_y(Alignment::Start)
    .into()
}

fn hint_symbol(symbol: &char) -> Element<'_, Message> {
    container(text(symbol).width(10).align_x(Alignment::Center).size(14))
        .padding([2, 8])
        .style(container::rounded_box)
        .into()
}
