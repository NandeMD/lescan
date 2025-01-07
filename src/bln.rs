use crate::Message;
use iced::widget::text::Wrapping;
use iced::widget::{row, text, Row};
use iced::Length::{Fill, FillPortion};
use rsff::balloon::Balloon;
use rsff::consts::TYPES;

pub const IDX_PORTION: u16 = 1;
pub const TYPE_TEXT_PORTION: u16 = 4;
pub const TL_PORTION: u16 = 10;
pub const COMMENTS_PORTION: u16 = 6;

pub fn bln_creator(idx: usize, bl: &Balloon) -> Row<Message> {
    let tl = &bl.tl_content;
    let cmmnts = &bl.comments;

    let tr_text = if tl.is_empty() {
        text("")
    } else {
        let pr = &bl.pr_content;
        if pr.is_empty() {
            let translation = tl.join(" ");
            text(translation)
        } else {
            text(pr.join(" "))
        }
    }
    .wrapping(Wrapping::Glyph)
    .width(FillPortion(TL_PORTION));

    let comments = if cmmnts.is_empty() {
        text("")
    } else {
        text(cmmnts.join(" "))
    }
    .wrapping(Wrapping::Glyph)
    .width(FillPortion(COMMENTS_PORTION));

    let type_text = match bl.btype {
        TYPES::OT => text("OT"),
        TYPES::ST => text("ST"),
        TYPES::DIALOGUE => text("DIALOGUE"),
        TYPES::SQUARE => text("SQUARE"),
        TYPES::THINKING => text("THINKING"),
    }
    .wrapping(Wrapping::Glyph)
    .width(FillPortion(TYPE_TEXT_PORTION));

    let r = row![
        text(idx).width(FillPortion(IDX_PORTION)),
        type_text,
        tr_text,
        comments
    ]
    .width(Fill);

    r
}
