use gpui::{
    div, rgb, Context, Hsla, InteractiveElement, IntoElement, MouseButton, ParentElement, Render,
    Styled, Window,
};

use crate::app::AppState;
use std::thread;
use std::time;

#[cfg(feature = "gpui_ui")]
pub struct GmailCleanerApp {
    app_state: AppState,
}

#[cfg(feature = "gpui_ui")]
impl GmailCleanerApp {
    pub fn new() -> Self {
        Self {
            app_state: AppState::new(),
        }
    }
}

#[cfg(feature = "gpui_ui")]
impl Render for GmailCleanerApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(rgb(0xffffff))
            .flex()
            .flex_col()
            .gap_3()
            .w_full()
            .h_full()
            .px_2()
            .children([
                div().text_center().text_3xl().child("Gmail Cleaner"),
                div().justify_center().children([div()
                    .bg(rgb(0x4CAF50))
                    .px_4()
                    .py_2()
                    .w_48()
                    .rounded_md()
                    .cursor_pointer()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _event, _win, cx| {
                            this.app_state.start();
                            thread::sleep(time::Duration::from_secs(5));
                            *this.app_state.status.lock().unwrap() =
                                "Open http://127.0.0.1:5000 in your web browser".to_string();
                            cx.notify();
                            cx.open_url("http://127.0.0.1:5000/");
                        }),
                    )
                    .text_center()
                    .child("start")]),
                div()
                    .border_t_1()
                    .border_color(Hsla::black())
                    .bg(rgb(0xe0e0e0))
                    .px_2()
                    .text_color(rgb(0x000000))
                    .text_sm()
                    .child(format!(
                        "{}",
                        self.app_state.status.lock().unwrap().as_str()
                    )),
            ])
    }
}
