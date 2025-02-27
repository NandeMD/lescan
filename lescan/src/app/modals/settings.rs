use iced::widget::{column, container, horizontal_rule, scrollable, text, text_input};
use iced::{Element, Length};
use iced_aw::sidebar::{SidebarWithContent, TabLabel};

pub fn settings_modal<'a, Message, F>(app: &'a crate::TestApp, m: F) -> Element<'a, Message>
where
    Message: Clone + 'a,
    F: 'static + Fn(SettingsTabs) -> Message + 'a,
{
    let general_settings = container(scrollable(column![
        text_input("Settings file path...", &app.settings.settings_file_path),
        text("dummy setting"),
    ]))
    .padding(5);

    let settings_sidebar_with_content = SidebarWithContent::new(m)
        .push(
            SettingsTabs::General,
            TabLabel::Text("General".into()),
            general_settings,
        )
        .push(
            SettingsTabs::Advanced,
            TabLabel::Text("Advanced".into()),
            text("Advanced settings"),
        )
        .tab_label_padding(5)
        .tab_label_spacing(5)
        .set_active_tab(&app.current_settings_tab)
        .height(Length::Fill);

    container(
        column![
            settings_sidebar_with_content,
            horizontal_rule(10),
            text("dummy setting")
        ]
        .height(Length::Fill),
    )
    .width(600)
    .height(450)
    .padding(10)
    .style(container::rounded_box)
    .into()
}

pub struct SettingsMenuContents {
    pub general_settings_file_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsTabs {
    General,
    Advanced,
}
