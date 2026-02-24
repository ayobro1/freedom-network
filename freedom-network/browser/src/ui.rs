use iced::{Application, Column, Command, Element, Text, TextInput, Row, Button, Scrollable};

pub struct FreedomBrowser {
    input_value: String,
    input_state: iced::text_input::State,
    top_sites: Vec<String>,
    scroll: iced::scrollable::State,
}

impl Application for FreedomBrowser {
    type Message = ();
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self {
            input_value: String::new(),
            input_state: iced::text_input::State::new(),
            top_sites: vec![
                "freedom://chat-site".into(),
                "freedom://example-site".into()
            ],
            scroll: iced::scrollable::State::new(),
        }, Command::none())
    }

    fn title(&self) -> String { "Freedom Browser".into() }

    fn update(&mut self, _msg: Self::Message) -> Command<Self::Message> { Command::none() }

    fn view(&mut self) -> Element<Self::Message> {
        let input = TextInput::new(&mut self.input_state, "Search or enter address", &self.input_value, |_| ());

        let top_sites_row = Row::with_children(
            self.top_sites.iter().map(|site| {
                Button::new(&mut iced::button::State::new(), Text::new(site)).into()
            }).collect()
        );

        let content = Column::new()
            .push(input)
            .push(top_sites_row)
            .push(Text::new("Connected Peers: 127.0.0.1:5000"))
            .spacing(20);

        Scrollable::new(&mut self.scroll)
            .padding(20)
            .push(content)
            .into()
    }
}