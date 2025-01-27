use iced::widget::{self, column, scrollable, text_editor};
use iced::{Element, Task, Theme};
use rsff::Document;
use std::sync::LazyLock;

use crate::message::Message;
use crate::utils::bln::bln_content_creator;
use crate::utils::handlers::*;

use crate::tinput::editor_kp_bindings;

use crate::balloons_table::*;

pub static SCROLLER_ID: LazyLock<scrollable::Id> = LazyLock::new(scrollable::Id::unique);
pub static SCROLLER_ID2: LazyLock<scrollable::Id> = LazyLock::new(scrollable::Id::unique);

pub struct TestApp {
    pub translation_document: Document,

    // Translation text
    pub t1_content: text_editor::Content,
    // Proofread text
    pub t2_content: text_editor::Content,
    // Comments
    pub t3_content: text_editor::Content,

    pub current_scroll: scrollable::AbsoluteOffset,

    pub current_balloon: usize,

    pub theme: Theme,

    pub columns: Vec<BalloonColumn>,
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

                current_scroll: scrollable::AbsoluteOffset::default(),
                current_balloon,

                theme: Theme::TokyoNight,

                columns: vec![
                    BalloonColumn::new(ColumnKind::Index),
                    BalloonColumn::new(ColumnKind::BlType),
                    BalloonColumn::new(ColumnKind::BlImage),
                    BalloonColumn::new(ColumnKind::TlContent),
                    BalloonColumn::new(ColumnKind::PrContent),
                    BalloonColumn::new(ColumnKind::Comments),
                ],
            },
            widget::focus_next(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        println!("{:?}", &message);
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
            Message::SyncHeader(offset) => {
                self.current_scroll = offset;
            }
            Message::TableColumnResizing(index, offset) => {
                if let Some(col) = self.columns.get_mut(index) {
                    col.resize_offset = Some(offset);
                }
            }
            Message::TableColumnResized => self.columns.iter_mut().for_each(|col| {
                if let Some(offset) = col.resize_offset.take() {
                    col.width += offset;
                }
            }),
            Message::TabPressed => return widget::focus_next(),
            Message::EnterPressed => handle_enter_key_press(self),
        }
        Task::batch(vec![
            scrollable::scroll_to(SCROLLER_ID.clone(), self.current_scroll),
            scrollable::scroll_to(SCROLLER_ID2.clone(), self.current_scroll),
        ])
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
        /* let scroller_col_names = row![
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
            .into() */

        let table = create_balloons_table(self);

        column![editor_1, editor_2, editor_3, table]
            .spacing(10)
            .padding(10)
            .into()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
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
        })])
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
