use iced::widget::{self, column, row, scrollable, text, text_editor, Column};
use iced::Length::FillPortion;
use iced::{Center, Element, Fill, Task, Theme};
use rsff::Document;
use std::cell::LazyCell;

use crate::message::Message;
use crate::utils::bln::*;
use crate::utils::handlers::*;

use crate::tinput::{editor_kp_bindings};

const SCROLLER_ID: LazyCell<scrollable::Id> = LazyCell::new(scrollable::Id::unique);

pub struct TestApp {
    pub translation_document: Document,

    // Translation text
    pub t1_content: text_editor::Content,
    // Proofread text
    pub t2_content: text_editor::Content,
    // Comments
    pub t3_content: text_editor::Content,

    pub current_scroll: scrollable::RelativeOffset,

    pub current_balloon: usize,

    pub theme: Theme,
}

impl TestApp {
    pub fn new() -> (Self, Task<Message>) {
        let tl_doc = Document::default().open("test.sffx").unwrap().unwrap();
        let current_balloon: usize = 0;

        let (t1_content, t2_content, t3_content) = bln_content_creator(&tl_doc, current_balloon);

        (
            Self {
                translation_document: tl_doc,

                t1_content,
                t2_content,
                t3_content,

                current_scroll: scrollable::RelativeOffset::START,
                current_balloon,

                theme: Theme::TokyoNight,
            },
            widget::focus_next(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        println!("{:?}", message);
        match message {
            Message::T1ContentChanged(action) => {
                self.t1_content.perform(action);
            }
            Message::T2ContentChanged(action) => {
                self.t2_content.perform(action);
            }
            Message::T3ContentChanged(action) => {
                self.t3_content.perform(action);
            }
            Message::Scrolled(viewport) => {
                self.current_scroll = viewport.relative_offset();
            }
            Message::TabPressed => return widget::focus_next(),
            Message::EnterPressed => handle_enter_key_press(self),
        }
        scrollable::snap_to(SCROLLER_ID.clone(), self.current_scroll)
    }

    pub fn view(&self) -> Element<Message> {
        let editor_1 = text_editor(&self.t1_content)
            .placeholder("Default text...")
            .on_action(Message::T1ContentChanged)
            .height(100)
            .padding(10)
            .key_binding(editor_kp_bindings);

        let editor_2 = text_editor(&self.t2_content)
            .placeholder("Default text...")
            .on_action(Message::T2ContentChanged)
            .height(100)
            .padding(10)
            .key_binding(editor_kp_bindings);

        let editor_3 = text_editor(&self.t3_content)
            .placeholder("Default text...")
            .on_action(Message::T3ContentChanged)
            .height(100)
            .padding(10)
            .key_binding(editor_kp_bindings);

        // Maybe change this table type to
        // https://github.com/tarkah/iced_table/blob/master/example/src/main.rs
        // ??
        let scroller_col_names = row![
            text!("ID").width(FillPortion(IDX_PORTION)),
            text!("Type").width(FillPortion(TYPE_TEXT_PORTION)),
            text!("Translation").width(FillPortion(TL_PORTION)),
            text!("Comments").width(FillPortion(COMMENTS_PORTION))
        ]
        .width(Fill);

        let mut bs = Column::new().align_x(Center).padding([40, 0]).spacing(40);
        for b in self.translation_document.balloons.iter().enumerate() {
            bs = bs.push(bln_creator(b.0, b.1));
        }

        let scroller: Element<Message> = Element::from(
            scrollable(bs)
                .direction(scrollable::Direction::Vertical(
                    scrollable::Scrollbar::new().anchor(scrollable::Anchor::Start),
                ))
                .width(Fill)
                .height(Fill)
                .id(SCROLLER_ID.clone())
                .on_scroll(Message::Scrolled),
        );

        column![editor_1, editor_2, editor_3, scroller_col_names, scroller]
            .spacing(10)
            .padding(10)
            .into()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        let batch =
            iced::Subscription::batch([iced::keyboard::on_key_press(|k, m| match (k, m) {
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
                _ => None,
            })]);

        batch
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
