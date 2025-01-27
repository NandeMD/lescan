use iced::widget::text_editor;
use rsff::Document;

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
