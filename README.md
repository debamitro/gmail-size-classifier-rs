# Gmail Cleaner

![downloads](https://img.shields.io/github/downloads/debamitro/gmail-size-classifier-rs/total)

A desktop app which tells you which emails are taking up space in your gmail mailbox. It runs on your computer and does not send your email data anywhere - hence you can use it for sensitive email accounts as well.

## Download

[![Download for MacOS Apple](https://img.shields.io/badge/Download-macOS%20Apple%20DMG-0078d7?style=for-the-badge)](https://github.com/debamitro/gmail-size-classifier-rs/releases/latest/download/gmail_cleaner_0.3_beta.dmg)

[![Download for MacOS Intel](https://img.shields.io/badge/Download-macOS%20Intel%20DMG-0078d7?style=for-the-badge)](https://github.com/debamitro/gmail-size-classifier-rs/releases/latest/download/gmail_cleaner_0.3_beta-x86_64-apple-darwin.dmg)

[![Download for Windows](https://img.shields.io/badge/Download-Windows%20ZIP-0078d7?style=for-the-badge)](https://github.com/debamitro/gmail-size-classifier-rs/releases/latest/download/gmail_cleaner_0.3_beta_win.zip)

## How to build

As of now there are two UIs for this tool. You can build either of them. The Iced UI is the default one used for official releases

### GPUI UI

```sh
cargo build --features gpui_ui
```

### Iced UI

```sh
 cargo build --features iced_ui
```


