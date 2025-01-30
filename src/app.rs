use iced::widget::{self, column, responsive, scrollable, text_editor};
use iced::{Element, Task, Theme};
use iced_table::table;
use rsff::Document;

use crate::message::Message;
use crate::utils::bln::bln_content_creator;
use crate::utils::handlers::*;

use crate::tinput::editor_kp_bindings;

use crate::balloons_table::*;

pub struct TestApp {
    pub translation_document: Document,

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
            },
            widget::focus_next(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        #[cfg(debug_assertions)]
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
        Task::none()
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

        let table = responsive(|size| {
            let t = table(
                self.table_header_scroller.clone(),
                self.table_body_scroller.clone(),
                &self.columns,
                &self.translation_document.balloons,
                Message::SyncHeader,
            )
            .on_column_resize(Message::TableColumnResizing, Message::TableColumnResized)
            .footer(self.table_footer_scroller.clone())
            .min_width(size.width);

            t.into()
        });

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
