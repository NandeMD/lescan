use crate::message::{Message, SettingsMenu, SettingsMenuContentChanged};
use iced::alignment::Vertical;
use iced::widget::{
    button, column, container, horizontal_rule, horizontal_space, pick_list, row, scrollable, text,
    text_input, Column,
};
use iced::{Element, Length};
use iced_aw::sidebar::{SidebarWithContent, TabLabel};
use rust_i18n::t;

fn general_settings(app: &crate::TestApp) -> impl Into<Element<Message>> {
    container(scrollable(
        Column::new()
            .push(input_with_header(
                t!("settings.settings_file_path.label"),
                text_input(
                    t!("settings.settings_file_path.placeholder").as_ref(),
                    &app.settings_menu_contents.general_settings_file_path,
                )
                .on_input(|text| {
                    Message::SettingsMenu(SettingsMenu::ContentChanged(
                        SettingsMenuContentChanged::GeneralSettingsFilePath(text),
                    ))
                }),
            ))
            .push(input_with_header(
                "App Theme",
                pick_list(
                    iced::Theme::ALL,
                    Some(app.settings_menu_contents.app_theme.clone()),
                    |t| {
                        Message::SettingsMenu(SettingsMenu::ContentChanged(
                            SettingsMenuContentChanged::GeneralSettingsAppTheme(t),
                        ))
                    },
                ),
            ))
            .spacing(5),
    ))
    .padding(5)
}

pub fn settings_modal(app: &crate::TestApp) -> Element<Message> {
    let settings_sidebar_with_content = SidebarWithContent::new(|t_id| {
        Message::SettingsMenu(SettingsMenu::SettingsTabSelected(t_id))
    })
    .push(
        SettingsTabs::General,
        TabLabel::Text(t!("settings.buttons.general").into_owned()),
        general_settings(app),
    )
    .push(
        SettingsTabs::Advanced,
        TabLabel::Text(t!("settings.buttons.advanced").into_owned()),
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
            row![
                horizontal_space().width(Length::Fill),
                button("Close").on_press(Message::HideModal),
                button("Apply").on_press(Message::SettingsMenu(SettingsMenu::ApplySettings)),
                button("Save").on_press(Message::SettingsMenu(SettingsMenu::SaveSettings)),
            ]
            .width(Length::Fill)
            .spacing(3)
            .align_y(Vertical::Center)
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
    pub app_theme: iced::Theme,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsTabs {
    General,
    Advanced,
}

fn input_with_header<'a>(
    header_text: impl AsRef<str> + 'a,
    element: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    Column::new()
        .push(text(header_text.as_ref().to_string()))
        .push(element)
        .padding(5)
        .into()
}
