use rfd::{AsyncMessageDialog, MessageButtons, MessageLevel};

pub async fn show_error_dialog(title: impl Into<String>, message: impl Into<String>) {
    AsyncMessageDialog::new()
        .set_level(MessageLevel::Error)
        .set_buttons(MessageButtons::Ok)
        .set_title(title)
        .set_description(message)
        .show()
        .await;
}
