use crate::app::AppState;
use iced::{
    widget::{button, column, container, horizontal_rule, row, text},
    Element, Subscription,
};
use native_dialog::{MessageDialog, MessageType};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum Message {
    StartServer,
    StopServer,
    ShowAbout,
    Tick,
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
                .set_text(&format!("Gmail Cleaner v0.3.0-beta\n\n{}", about_text))
                .show_alert();
        }
        Message::Tick => {
            // No action needed, just trigger a UI refresh
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
        button("Start")
            .on_press_maybe(if state.app_state.is_running() {
                None
            } else {
                Some(Message::StartServer)
            })
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

pub fn subscription(_state: &GmailCleanerApp) -> Subscription<Message> {
    use iced::futures::{stream, StreamExt};
    use std::time::Instant;

    Subscription::run_with_id(
        "timer",
        stream::unfold(Instant::now(), |start| async move {
            let now = Instant::now();
            let elapsed = now.duration_since(start);

            // Sleep for the remaining time to reach 1 second
            let sleep_duration = Duration::from_secs(1).saturating_sub(elapsed);
            async_std::task::sleep(sleep_duration).await;

            Some((Message::Tick, Instant::now()))
        }),
    )
}
