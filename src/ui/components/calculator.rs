use iced::{Alignment, Color, Element, Font, Length, Theme, font::Weight, widget::{column, container, row, text}};

use crate::{Message, providers::CalculatorResult, ui::components::Components};


impl Components {

    pub fn calculator_item(calc: &CalculatorResult) -> Element<'_, Message> {

        let icon = container(
            text("🖩")
            .size(20)
        )
            .width(50)
            .height(24)
            .align_y(Alignment::Center)
            .align_x(Alignment::Center);

        let txt = calc.result.clone().unwrap_or("-".to_string());
        let title = text(txt)
            .size(16)
            .font(Font {
                weight: Weight::Bold,
                ..Font::DEFAULT
            });

        let desc = text(&calc.expression)
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
                container::Style {
                    background: Some(palette.primary.into()),
                    text_color: Some(Color::WHITE),
                    ..container::Style::default()
                }
            })
        .into()
    }
}
