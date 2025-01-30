use crate::message::Message;
use iced::widget::{checkbox, container, text};
use iced::{Element, Length, Renderer, Theme};
use iced_table::table;
use rsff::balloon::Balloon;
use rsff::consts::TYPES;

pub enum ColumnKind {
    Index,
    BlType,
    BlImage,
    TlContent,
    PrContent,
    Comments,
}

// Based on:
// https://github.com/tarkah/iced_table/blob/master/example/src/main.rs

pub struct BCol {
    pub kind: ColumnKind,
    pub width: f32,
    pub resize_offset: Option<f32>,
}

impl BCol {
    pub fn new(kind: ColumnKind) -> Self {
        let width = match kind {
            ColumnKind::Index => 20.0,
            ColumnKind::BlType => 60.0,
            ColumnKind::BlImage => 60.0,
            ColumnKind::TlContent => 100.0,
            ColumnKind::PrContent => 100.0,
            ColumnKind::Comments => 100.0,
        };

        BCol {
            kind,
            width,
            resize_offset: None,
        }
    }
}

impl<'a> table::Column<'a, Message, Theme, Renderer> for BCol {
    type Row = Balloon;

    fn header(&'a self, _col_index: usize) -> Element<'a, Message> {
        let content = match self.kind {
            ColumnKind::Index => "#",
            ColumnKind::BlType => "Type",
            ColumnKind::BlImage => "Img",
            ColumnKind::TlContent => "TL Content",
            ColumnKind::PrContent => "PR Content",
            ColumnKind::Comments => "Comments",
        };

        container(text(content)).center_y(24).into()
    }

    fn cell(
        &'a self,
        _col_index: usize,
        row_index: usize,
        row: &'a Self::Row,
    ) -> Element<'a, Message> {
        let content: Element<_> = match self.kind {
            ColumnKind::Index => text(row_index).into(),
            ColumnKind::BlImage => {
                if row.balloon_img.is_some() {
                    checkbox("", true)
                } else {
                    checkbox("", false)
                }
            }
            .into(),
            ColumnKind::TlContent => text(row.tl_content.join(" - ")).into(),
            ColumnKind::PrContent => text(row.pr_content.join(" - ")).into(),
            ColumnKind::Comments => text(row.comments.join(" - ")).into(),
            ColumnKind::BlType => match row.btype {
                TYPES::DIALOGUE => text("Dialogue"),
                TYPES::SQUARE => text("Square"),
                TYPES::THINKING => text("Thinking"),
                TYPES::OT => text("OT"),
                TYPES::ST => text("ST"),
            }
            .into(),
        };

        container(content).width(Length::Fill).center_y(32).into()
    }

    fn footer(&'a self, _col_index: usize, _rows: &'a [Self::Row]) -> Option<Element<'a, Message>> {
        // This footer is one per column, so it's not useful for us
        // I want to have some bottom, single footer for the whole table
        // let content = text(format!("Balloons: {}", rows.len()));
        // Some(container(content).center_y(24).into())

        None
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}
