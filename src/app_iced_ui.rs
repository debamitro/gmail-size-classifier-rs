use crate::app::AppState;
use iced::{
    widget::{button, column, container, horizontal_rule, row, text},
    Application, Element, Settings, Theme,
};
use native_dialog::{MessageDialog, MessageType};

#[derive(Debug, Clone)]
pub enum Message {
    StartServer,
    StopServer,
    ShowAbout,
}

pub struct GmailCleanerApp {
    app_state: AppState,
}

impl GmailCleanerApp {
    pub fn new() -> Self {
        Self {
            app_state: AppState::new(),
        }
    }
}

impl Default for GmailCleanerApp {
    fn default() -> Self {
        Self::new()
    }
}

pub fn update(state: &mut GmailCleanerApp, message: Message) {
    match message {
        Message::StartServer => {
            state.app_state.start();
            // Open browser after a delay
            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_secs(3));
                if let Err(e) = open::that("http://127.0.0.1:5000") {
                    eprintln!("Failed to open browser: {}", e);
                }
            });
        }
        Message::StopServer => {
            state.app_state.stop();
        }
        Message::ShowAbout => {
            // Show about dialog in a separate system window
            let about_text = "Gmail Cleaner is a desktop app which helps you find out which emails are taking up storage space in your Gmail account. This app runs on your desktop and does not send your email to any server. Therefore it is the most secure way of cleaning up your Gmail account. You don't need to give any permissions to this app, neither do you need to provide credentials. When you start it, a browser window opens up with Gmail's login page. Once Gmail authorizes you the app visually shows you what you can delete. The app does not read your email or cannot modify your Gmail account in any way.";

            let _ = MessageDialog::new()
                .set_type(MessageType::Info)
                .set_title("About Gmail Cleaner")
                .set_text(&format!("Gmail Cleaner v0.1.0-beta\n\n{}", about_text))
                .show_alert();
        }
    }
}

pub fn view(state: &GmailCleanerApp) -> Element<Message> {
    let status = state.app_state.get_status();
    let content = column![
        row![
            text("Gmail Cleaner").size(28),
            container(
                button("About")
                    .on_press(Message::ShowAbout)
                    .style(iced::widget::button::text)
            )
            .width(iced::Length::Fill)
            .align_x(iced::alignment::Horizontal::Right)
        ],
        button("start")
            .on_press(Message::StartServer)
            .style(iced::widget::button::primary),
        horizontal_rule(2),
        text(status).size(12),
    ]
    .spacing(10)
    .padding(10);

    container(content)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .center_x(iced::Length::Fill)
        .center_y(iced::Length::Fill)
        .into()
}
