use iced::{widget, Length};

pub fn row_space_between<'a, Message>(
    leading: iced::Element<'a, Message>,
    trailing: iced::Element<'a, Message>,
) -> widget::Row<'a, Message> where Message: Send + 'static {
    widget::row![
        leading,
        widget::space().width(Length::Fill),
        trailing
    ]
}

#[allow(dead_code)]
pub fn column_space_between<'a, Message>(
    leading: iced::Element<'a, Message>,
    trailing: iced::Element<'a, Message>,
) -> widget::Column<'a, Message> where Message: Send + 'static {
    widget::column![
        leading,
        widget::space().width(Length::Fill),
        trailing
    ]
}
