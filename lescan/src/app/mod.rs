pub mod modals;
pub mod widgets;

use iced::widget::{self, column, pane_grid, scrollable, text_editor, Column};
use iced::{Element, Length, Task, Theme};
use iced_aw::{
    menu::{Item, Menu},
    menu_bar, menu_items,
};
use rsff::Document;
use rust_i18n::t;
use widgets::top_menu::*;

use crate::app_cache::AppCache;
use crate::message::Message;
use crate::settings::AppSettings;
use crate::utils::bln::bln_content_creator;
use crate::utils::handlers::*;
use crate::utils::{panes::MainPanes, tabs::ImageTabs};

use widgets::footer::footer;
use widgets::main_content::{main_content_pane_grid, Pane};

pub struct TestApp {
    pub translation_document: Document,
    pub settings: AppSettings,
    pub settings_menu_contents: modals::settings::SettingsMenuContents,

    pub panes: pane_grid::State<widgets::main_content::Pane>,

    pub selected_bln_type: Option<widgets::main_content::BlnTypes>,
    // Translation text
    pub t1_content: text_editor::Content,
    // Proofread text
    pub t2_content: text_editor::Content,
    // Comments
    pub t3_content: text_editor::Content,

    pub current_balloon: usize,

    pub current_img_tab: ImageTabs,
    pub img_scroller: scrollable::Id,
    pub img_scroller_current_scroll: scrollable::RelativeOffset,

    pub current_settings_tab: modals::settings::SettingsTabs,

    pub document_file_location: Option<String>,

    pub show_modal: Option<modals::ModalType>,
    pub modal_markdowns: modals::ModalMarkdowns,
}

impl TestApp {
    pub fn new() -> (Self, Task<Message>) {
        let mut tl_doc = Document::default();
        tl_doc.add_balloon_empty();
        let current_balloon: usize = 0;

        let (t1_content, t2_content, t3_content) = bln_content_creator(&tl_doc, current_balloon);

        let pane_config = pane_grid::Configuration::Split {
            axis: pane_grid::Axis::Horizontal,
            ratio: 0.5,
            a: Box::new(pane_grid::Configuration::Split {
                axis: pane_grid::Axis::Vertical,
                ratio: 0.5,
                a: Box::new(pane_grid::Configuration::Pane(Pane {
                    id: MainPanes::Image,
                })),
                b: Box::new(pane_grid::Configuration::Pane(Pane {
                    id: MainPanes::Editor,
                })),
            }),
            b: Box::new(pane_grid::Configuration::Pane(Pane {
                id: MainPanes::Table,
            })),
        };

        let panes = pane_grid::State::with_configuration(pane_config);

        let cache = AppCache::default();
        let settings = AppSettings::new(cache.settings_file_path.clone());
        let settings_menu_contents = modals::settings::SettingsMenuContents {
            general_settings_file_path: settings.settings_file_path.clone(),
            app_theme: settings.app_theme.clone(),
            language: settings.language.clone(),
        };

        if let Some(lang) = &settings.language {
            rust_i18n::set_locale(lang);
        }

        (
            Self {
                translation_document: tl_doc,
                settings,
                settings_menu_contents,
                panes,

                selected_bln_type: Some(widgets::main_content::BlnTypes::Dialogue),
                t1_content,
                t2_content,
                t3_content,

                current_balloon,

                current_img_tab: ImageTabs::Document,
                img_scroller: scrollable::Id::unique(),
                img_scroller_current_scroll: scrollable::RelativeOffset::START,

                current_settings_tab: modals::settings::SettingsTabs::General,

                document_file_location: None,
                show_modal: None,
                modal_markdowns: modals::ModalMarkdowns::default(),
            },
            widget::focus_next(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        #[cfg(debug_assertions)]
        println!("{:?}", &message);
        message_handler(message, self)
    }

    pub fn view(&self) -> Element<Message> {
        let menu_tpl_1 = |items| Menu::new(items).max_width(180.0).offset(15.0).spacing(5.0);
        //let menu_tpl_2 = |items| Menu::new(items).max_width(180.0).offset(0.0).spacing(5.0);

        #[rustfmt::skip]
        let mb = menu_bar!(
            (menu_main_button(t!("file_menu.file")), menu_tpl_1(menu_items!(
                (menu_sub_button_file_new())
                (menu_sub_button_file_open())
                (menu_sub_button_file_save())
                (menu_sub_button_file_save_as())
            )))
            (menu_main_button(t!("app_menu.app")), menu_tpl_1(menu_items!(
                (menu_sub_button_app_settings())
                (menu_sub_button_about())
            )))
        );

        let pg = main_content_pane_grid(self);

        let footer_text = format!(
            "{}: {} | {}: {} | {}: {} | {}: {} | {}: {}",
            t!("footer.balloons"),
            self.translation_document.balloons.len(),
            t!("footer.total_lines"),
            self.translation_document.line_count(),
            t!("footer.tl_chars"),
            self.translation_document.tl_chars(),
            t!("footer.pr_chars"),
            self.translation_document.pr_chars(),
            t!("footer.comment_chars"),
            self.translation_document.comment_chars()
        );
        let ftr = footer(footer_text)
            .width(Length::Fill)
            .height(Length::Fixed(30.0));

        if let Some(modal) = &self.show_modal {
            let base: Column<Message> = column![mb, pg, ftr].spacing(10).padding(10);
            modals::modal_handler(
                base,
                modal.clone(),
                Message::HideModal,
                Message::LinkClicked,
                self,
            )
        } else {
            column![mb, pg, ftr].spacing(10).padding(10).into()
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch([
            iced::keyboard::on_key_press(move |k, m| match (k, m) {
                (iced::keyboard::Key::Named(iced::keyboard::key::Named::Tab), _) => {
                    Some(Message::TabPressed)
                }
                (
                    iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter),
                    iced::keyboard::Modifiers::SHIFT,
                ) => None,
                (iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter), _) => {
                    Some(Message::EnterPressed)
                }
                (
                    iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowUp),
                    iced::keyboard::Modifiers::SHIFT,
                ) => Some(Message::BalloonTypeCycleUp),
                (
                    iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowDown),
                    iced::keyboard::Modifiers::SHIFT,
                ) => Some(Message::BalloonTypeCycleDown),
                _ => None,
            }),
            iced::keyboard::on_key_press(|k, m| {
                if let iced::keyboard::Key::Character(c) = k {
                    if c == "v" && m.control() || m.command() {
                        Some(Message::CurrentBlnImgPaste)
                    } else if c == "s" && m.control() || m.command() {
                        Some(Message::FileOperation(
                            crate::message::FileOperation::SaveFileDialog,
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }),
            iced::event::listen_with(|ev, _status, _window| match ev {
                iced::Event::Window(iced::window::Event::FileDropped(pth)) => {
                    Some(Message::FileDropped(pth))
                }
                iced::Event::Window(iced::window::Event::CloseRequested) => Some(Message::ExitApp),
                _ => None,
            }),
        ])
    }

    pub fn theme(&self) -> Theme {
        self.settings.app_theme.clone()
    }
}
