use iced::{widget, Alignment, Color, Length};

pub fn icon_text_button<'a, Message>(
    svg_handle: impl Into<widget::svg::Handle>,
    width: impl Into<Length>, height: impl Into<Length>,
    color: Color, text: impl widget::text::IntoFragment<'a>,
) -> widget::Button<'a, Message> where Message: Send + 'static{
    widget::button(
        widget::row![
            widget::svg(svg_handle)
            .width(width).height(height)
            .style(move |_, _| { 
                widget::svg::Style {
                    color: Some(color)
                }
            }),
            widget::text(text)
        ]
        .spacing(4)
        .align_y(Alignment::Center)
    )
}

pub fn icon_button<'a, Message>(
    svg_handle: impl Into<widget::svg::Handle>,
    width: impl Into<Length>, height: impl Into<Length>,
    color: Color,
) -> widget::Button<'a, Message> where Message: Send + 'static{
    widget::button(
        widget::svg(svg_handle)
        .width(width).height(height)
        .style(move |_, _| { 
            widget::svg::Style {
                color: Some(color)
            }
        }),
    )
}
