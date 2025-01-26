use crate::message::Message;
use iced::widget::text::Wrapping;
use iced::widget::{row, text, text_editor, Row};
use iced::Length::{Fill, FillPortion};
use rsff::balloon::Balloon;
use rsff::consts::TYPES;
use rsff::Document;

pub const IDX_PORTION: u16 = 1;
pub const TYPE_TEXT_PORTION: u16 = 4;
pub const TL_PORTION: u16 = 10;
pub const COMMENTS_PORTION: u16 = 6;

pub fn bln_creator(idx: usize, bl: &Balloon) -> Row<Message> {
    let tl = &bl.tl_content;
    let pr = &bl.pr_content;
    let cmmnts = &bl.comments;

    let tr_text = if pr.is_empty() {
        text(tl.join(" - "))
    } else {
        text(pr.join(" - "))
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

pub fn bln_content_creator(
    tl_doc: &Document,
    current_balloon: usize,
) -> (
    text_editor::Content,
    text_editor::Content,
    text_editor::Content,
) {
    let t1_content = {
        if tl_doc.balloons.is_empty() {
            text_editor::Content::default()
        } else {
            let tl = &tl_doc.balloons[current_balloon].tl_content;
            text_editor::Content::with_text(tl.join("\n//\n").as_str())
        }
    };

    let t2_content = {
        if tl_doc.balloons.is_empty() {
            text_editor::Content::default()
        } else {
            let pr = &tl_doc.balloons[current_balloon].pr_content;
            text_editor::Content::with_text(pr.join("\n//\n").as_str())
        }
    };

    let t3_content = {
        if tl_doc.balloons.is_empty() {
            text_editor::Content::default()
        } else {
            let cmmnts = &tl_doc.balloons[current_balloon].comments;
            text_editor::Content::with_text(cmmnts.join("\n//\n").as_str())
        }
    };

    (t1_content, t2_content, t3_content)
}
