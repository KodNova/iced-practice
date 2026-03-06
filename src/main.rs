use iced::widget::{column, button, text};
use iced::Element;
#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update (&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {


column![
    button("+").on_press(Message::Increment),
    text(self.value),
    button("-").on_press(Message::Decrement),
].into()

    }
}

fn  main() -> iced::Result {
iced::run(Counter::update, Counter::view)
}
