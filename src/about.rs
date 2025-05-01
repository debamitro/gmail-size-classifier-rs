use gpui::{div, rgb, Context, IntoElement, ParentElement, Render, Styled, Window, Hsla};

pub struct AboutWindow {}

impl Render for AboutWindow {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let about_text =
        "Gmail Cleaner is a desktop app which helps you find out which emails are taking up storage space \
in your Gmail account. This app runs on your desktop and does not send your email to any server. \
Therefore it is the most secure way of cleaning up your Gmail account. You don't need to give any permissions \
to this app, neither do you need to provide credentials. When you start it, a browser window opens up \
with Gmail's login page. Once Gmail authorizes you the app visually shows you what you can delete. \
The app does not read your email or cannot modify your Gmail account in any way.";
        div()
            .bg(rgb(0xffffff))
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .px_2()
            .children([
                div().text_3xl().text_center().child("About Gmail Cleaner"),
                div().text_center().child("Version 0.1.0"),
                div().border_t_1().border_color(Hsla::black()).h_1_2().flex_initial().child(about_text),
            ])
    }
}
