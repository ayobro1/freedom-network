use iced::widget::{column, row, text, text_input, button, scrollable, container, vertical_space, vertical_rule};
use iced::{Application, Command, Element, Theme, Length, Alignment};

#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Home,
    ChatSite,
    ExampleSite,
}

impl Tab {
    fn label(&self) -> &str {
        match self {
            Tab::Home => "Home",
            Tab::ChatSite => "Chat",
            Tab::ExampleSite => "Example",
        }
    }

    fn all() -> Vec<Tab> {
        vec![Tab::Home, Tab::ChatSite, Tab::ExampleSite]
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SwitchTab(Tab),
    AddressChanged(String),
    NavigateTo,
    Refresh,
}

pub struct FreedomBrowser {
    current_tab: Tab,
    input_value: String,
}

impl Application for FreedomBrowser {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self {
            current_tab: Tab::Home,
            input_value: String::new(),
        }, Command::none())
    }

    fn title(&self) -> String { "Freedom Browser".into() }

    fn update(&mut self, msg: Self::Message) -> Command<Self::Message> {
        match msg {
            Message::SwitchTab(tab) => {
                self.current_tab = tab;
                self.input_value.clear();
                Command::none()
            }
            Message::AddressChanged(value) => {
                self.input_value = value;
                Command::none()
            }
            Message::NavigateTo => {
                if !self.input_value.is_empty() {
                    let url_lower = self.input_value.to_lowercase();
                    if url_lower.contains("chat") {
                        self.current_tab = Tab::ChatSite;
                    } else if url_lower.contains("example") {
                        self.current_tab = Tab::ExampleSite;
                    } else {
                        self.current_tab = Tab::Home;
                    }
                    self.input_value.clear();
                }
                Command::none()
            }
            Message::Refresh => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        // Left sidebar with tabs (Arc browser style)
        let sidebar_buttons: Vec<Element<'_, Message>> = Tab::all()
            .iter()
            .map(|tab| {
                let icon = self.get_icon(tab);

                let button_content = container(
                    row([
                        text(icon).size(18).into(),
                        text(tab.label()).size(12).into(),
                    ])
                    .align_items(Alignment::Center)
                    .spacing(8)
                )
                .padding(14.0);

                button(button_content)
                    .on_press(Message::SwitchTab(tab.clone()))
                    .width(Length::Fill)
                    .padding([6, 0])
                    .into()
            })
            .collect();

        let sidebar = container(
            scrollable(
                column(sidebar_buttons)
                    .width(Length::Fill)
                    .spacing(6)
                    .padding([16, 0])
            )
            .height(Length::Fill)
        )
        .width(Length::Shrink)
        .height(Length::Fill)
        .padding([20, 8]);

        // Header bar with controls
        let header = container(
            row([
                button("🔄").on_press(Message::Refresh).padding([6, 10]).into(),
                text_input("freedom://...", &self.input_value)
                    .on_input(Message::AddressChanged)
                    .on_submit(Message::NavigateTo)
                    .padding([8, 12])
                    .width(Length::Fill)
                    .into(),
            ])
            .spacing(10)
            .align_items(Alignment::Center)
            .padding([12, 20])
            .width(Length::Fill)
        )
        .width(Length::Fill);

        // Content area
        let content = self.render_page_content();

        // Main view: sidebar on left, content on right
        let main_view = row([
            sidebar.into(),
            vertical_rule(2).into(),
            column([
                header.into(),
                content.into(),
            ])
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(0)
            .into(),
        ])
        .spacing(0)
        .height(Length::Fill);

        container(main_view)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl FreedomBrowser {
    fn get_icon(&self, tab: &Tab) -> &'static str {
        match tab {
            Tab::Home => "🏠",
            Tab::ChatSite => "💬",
            Tab::ExampleSite => "📖",
        }
    }

    fn render_page_content(&self) -> Element<'_, Message> {
        let (title, content) = match self.current_tab {
            Tab::Home => (
                "🏠 Home",
                "Welcome to Freedom Browser 🌐\n\nYou are browsing the decentralized Freedom Network.\n\nQuick Start:\n• Visit Chat for messaging\n• Explore Example for more information\n• Type addresses in the bar to navigate\n\nConnected to: 127.0.0.1:5000\nStatus: Active (1 peers connected)"
            ),
            Tab::ChatSite => (
                "💬 Freedom Chat",
                "Welcome to Freedom Chat!\n\nA decentralized, peer-to-peer messaging platform.\n\nFeatures:\n• End-to-end encrypted messages\n• No central server required\n• Anonymous messaging\n• Message verification\n\nStart chatting with friends on the Freedom network!"
            ),
            Tab::ExampleSite => (
                "📖 Example Site",
                "Example Site\n\nThis is a demonstration site on the Freedom Network.\n\nThe Freedom Network is a decentralized internet built on QUIC protocol with:\n• Multi-hop routing\n• Cryptographic verification\n• Peer discovery\n• Content distribution\n\nVisit other sites by clicking tabs or typing in the address bar"
            ),
        };

        container(
            scrollable(
                column([
                    text(title).size(28).into(),
                    vertical_space().height(Length::Fixed(12.0)).into(),
                    text(content).size(14).into(),
                    vertical_space().height(Length::Fill).into(),
                ])
                .spacing(12)
                .padding(30)
            )
            .height(Length::Fill)
            .width(Length::Fill)
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }
}