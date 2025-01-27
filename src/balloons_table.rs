use crate::message::Message;
use iced::widget::{checkbox, container, horizontal_space, responsive, text};
use iced::Length::Fill;
use iced::{Element, Renderer, Theme};
use iced_table::table;
use rsff::balloon::Balloon;
use rsff::consts::TYPES;

pub fn create_balloons_table(app: &crate::TestApp) -> Element<Message> {
    responsive(|_size| {
        table(
            crate::app::SCROLLER_ID.clone(),
            crate::app::SCROLLER_ID2.clone(),
            &app.columns,
            &app.translation_document.balloons,
            Message::SyncHeader,
        )
        .on_column_resize(Message::TableColumnResizing, Message::TableColumnResized)
        .into()
    })
    .into()
}

pub enum ColumnKind {
    Index,
    BlType,
    BlImage,
    TlContent,
    PrContent,
    Comments,
}

pub struct BalloonColumn {
    pub kind: ColumnKind,
    pub width: f32,
    pub resize_offset: Option<f32>,
}

impl BalloonColumn {
    pub fn new(kind: ColumnKind) -> Self {
        let width = match kind {
            ColumnKind::Index => 20.0,
            ColumnKind::BlType => 60.0,
            ColumnKind::BlImage => 60.0,
            ColumnKind::TlContent => 100.0,
            ColumnKind::PrContent => 100.0,
            ColumnKind::Comments => 100.0,
        };

        BalloonColumn {
            kind,
            width,
            resize_offset: None,
        }
    }
}

impl<'a> table::Column<'a, Message, Theme, Renderer> for BalloonColumn {
    type Row = Balloon;

    fn header(&'a self, _: usize) -> Element<'a, Message> {
        let content = match self.kind {
            ColumnKind::Index => "#",
            ColumnKind::BlType => "Type",
            ColumnKind::BlImage => "Img",
            ColumnKind::TlContent => "TL Content",
            ColumnKind::PrContent => "PR Content",
            ColumnKind::Comments => "Comments",
        };
        text(content).into()
    }

    fn cell(
        &'a self,
        _col_index: usize,
        row_index: usize,
        row: &'a Self::Row,
    ) -> Element<'a, Message> {
        match self.kind {
            ColumnKind::Index => container(text(row_index)),
            ColumnKind::BlImage => {
                if row.balloon_img.is_some() {
                    container(checkbox("", true))
                } else {
                    container(checkbox("", false))
                }
            }
            ColumnKind::TlContent => container(text(row.tl_content.join(" - "))),
            ColumnKind::PrContent => container(text(row.pr_content.join(" - "))),
            ColumnKind::Comments => container(text(row.comments.join(" - "))),
            ColumnKind::BlType => match row.btype {
                TYPES::DIALOGUE => container(text("Dialogue")),
                TYPES::SQUARE => container(text("Square")),
                TYPES::THINKING => container(text("Thinking")),
                TYPES::OT => container(text("OT")),
                TYPES::ST => container(text("ST")),
            },
        }
        .width(Fill)
        .into()
    }

    fn footer(&'a self, _col_index: usize, rows: &'a [Self::Row]) -> Option<Element<'a, Message>> {
        let content = text(format!("Balloons: {}", rows.len()));

        Some(container(content).center_y(24).into())
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}
