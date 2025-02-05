pub mod widgets;

use iced::widget::{self, column, pane_grid, scrollable, text_editor};
use iced::{Element, Length, Task, Theme};
use iced_aw::menu::*;
use rsff::Document;

use crate::message::Message;
use crate::utils::bln::bln_content_creator;
use crate::utils::handlers::*;

use widgets::balloons_table::*;
use widgets::footer::footer;
use widgets::main_content::{main_content_pane_grid, Pane};
pub struct TestApp {
    pub translation_document: Document,

    pub panes: pane_grid::State<widgets::main_content::Pane>,

    pub selected_bln_type: Option<widgets::main_content::BlnTypes>,
    // Translation text
    pub t1_content: text_editor::Content,
    // Proofread text
    pub t2_content: text_editor::Content,
    // Comments
    pub t3_content: text_editor::Content,

    pub current_balloon: usize,

    pub theme: Theme,

    pub columns: Vec<BCol>,
    pub current_scroll: scrollable::AbsoluteOffset,
    pub table_header_scroller: scrollable::Id,
    pub table_body_scroller: scrollable::Id,
    pub table_footer_scroller: scrollable::Id,

    pub current_img_tab: usize,
    pub img_scroller: scrollable::Id,
    pub img_scroller_current_scroll: scrollable::RelativeOffset,
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
                a: Box::new(pane_grid::Configuration::Pane(Pane { id: 0 })),
                b: Box::new(pane_grid::Configuration::Pane(Pane { id: 1 })),
            }),
            b: Box::new(pane_grid::Configuration::Pane(Pane { id: 2 })),
        };

        let panes = pane_grid::State::with_configuration(pane_config);

        (
            Self {
                translation_document: tl_doc,
                panes,

                selected_bln_type: Some(widgets::main_content::BlnTypes::Dialogue),
                t1_content,
                t2_content,
                t3_content,

                current_scroll: scrollable::AbsoluteOffset::default(),
                current_balloon,

                theme: Theme::TokyoNight,

                columns: vec![
                    BCol::new(ColumnKind::Index),
                    BCol::new(ColumnKind::BlType),
                    BCol::new(ColumnKind::BlImage),
                    BCol::new(ColumnKind::TlContent),
                    BCol::new(ColumnKind::PrContent),
                    BCol::new(ColumnKind::Comments),
                ],

                table_header_scroller: scrollable::Id::unique(),
                table_body_scroller: scrollable::Id::unique(),
                table_footer_scroller: scrollable::Id::unique(),

                current_img_tab: 0,
                img_scroller: scrollable::Id::unique(),
                img_scroller_current_scroll: scrollable::RelativeOffset::START,
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
        let menu_files = Item::with_menu(
            iced::widget::button("File"),
            Menu::new(
                [
                    Item::new(iced::widget::button("Open")),
                    Item::new(iced::widget::button("Save")),
                ]
                .into(),
            ),
        );

        let menu_bar = MenuBar::new(vec![menu_files]).width(iced::Length::Fill);

        let pg = main_content_pane_grid(self);

        let footer_text = format!(
            "Balloons: {} | Total Lines: {} | TL Characters: {} | PR Characters: {} | Comment Characters: {}",
            self.translation_document.balloons.len(),
            self.translation_document.line_count(),
            self.translation_document.tl_chars(),
            self.translation_document.pr_chars(),
            self.translation_document.comment_chars()
        );
        let ftr = footer(footer_text)
            .width(Length::Fill)
            .height(Length::Fixed(30.0));

        column![menu_bar, pg, ftr].spacing(10).padding(10).into()
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
                _ => None,
            }),
        ])
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
