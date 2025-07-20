use iced::{
    widget::{button, column, container, text},
    Application, Command, Element, Settings, Theme,
};

mod app;
mod server;
use app::AppState;

fn main() -> iced::Result {
    GmailCleanerApp::run(Settings {
        window: iced::window::Settings {
            size: (400, 200),
            resizable: true,
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
pub enum Message {
    StartServer,
    StopServer,
    ShowAbout,
}

struct GmailCleanerApp {
    app_state: AppState,
    show_about: bool,
}

impl Application for GmailCleanerApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            GmailCleanerApp {
                app_state: AppState::new(),
                show_about: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Gmail Cleaner")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::StartServer => {
                self.app_state.start();
                // Open browser after a delay
                std::thread::spawn(|| {
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    if let Err(e) = open::that("http://127.0.0.1:5000") {
                        eprintln!("Failed to open browser: {}", e);
                    }
                });
            }
            Message::StopServer => {
                self.app_state.stop();
            }
            Message::ShowAbout => {
                self.show_about = !self.show_about;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let status = self.app_state.get_status();
        
        let content = if self.show_about {
            column![
                text("Gmail Cleaner").size(24),
                text("A tool to help clean up your Gmail inbox").size(14),
                text("Version 0.1.0").size(12),
                button("Back").on_press(Message::ShowAbout),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center)
        } else {
            column![
                text("Gmail Cleaner").size(24),
                button("Start Server")
                    .on_press(Message::StartServer)
                    .style(iced::theme::Button::Primary),
                button("Stop Server")
                    .on_press(Message::StopServer)
                    .style(iced::theme::Button::Secondary),
                button("About")
                    .on_press(Message::ShowAbout)
                    .style(iced::theme::Button::Text),
                text(&status).size(12),
            ]
            .spacing(10)
            .align_items(iced::Alignment::Center)
        };

        container(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
