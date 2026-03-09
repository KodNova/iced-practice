#![windows_subsystem = "windows"]
use iced::Element;
use iced::widget::{button, column, container, row, text, tooltip};
#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
    Increment3,
    Decrement3,
    Incrementx3,
    Decrementx3,
}

fn cal_new_value(current: i64, message: Message) -> i64 {
    match message {
        Message::Increment => current + 1,
        Message::Decrement => current - 1,
        Message::Increment3 => current + 3,
        Message::Decrement3 => current - 3,
        Message::Incrementx3 => match current {
            num if num > 0 => num * 3,
            num if num < 0 => num / 3,
            num => num,
        },
        Message::Decrementx3 => match current {
            num if num > 0 => num / 3,
            num if num < 0 => num * 3,
            num => num,
        },
    }
}

impl Counter {
    fn update(&mut self, message: Message) {
        self.value = cal_new_value(self.value, message)
    }

    fn view(&self) -> Element<'_, Message> {
        row![
            column![
                text("one"),
                button("+").on_press(Message::Increment),
                text(self.value),
                button("-").on_press(Message::Decrement),
            ],
            column![
                text("three"),
                tooltip(
                    button("+").on_press(Message::Increment3),
                    container("increase by 3")
                        .padding(10)
                        .style(container::rounded_box),
                    tooltip::Position::Bottom,
                ),
                text(self.value),
                button("-").on_press(Message::Decrement3),
            ],
            column![
                text("x3"),
                button("+").on_press(Message::Incrementx3),
                text(self.value),
                button("-").on_press(Message::Decrementx3),
            ]
        ]
        .into()
    }
}

fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}
