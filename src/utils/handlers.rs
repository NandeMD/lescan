use crate::app::TestApp;
use iced::widget::text_editor;
use rsff::balloon::Balloon;

pub fn handle_enter_key_press(app: &mut TestApp) {
    // Save the content of the text editors to the current balloon
    let tl = app
        .t1_content
        .text()
        .trim_end_matches("\n//\n")
        .trim_end_matches("\n")
        .split("\n//\n")
        .map(|s| s.to_string())
        .collect();
    println!("{:?}", tl);
    let pr = app
        .t2_content
        .text()
        .trim_end_matches("\n//\n")
        .trim_end_matches("\n")
        .split("\n//\n")
        .map(|s| s.to_string())
        .collect();
    let cmmnts = app
        .t3_content
        .text()
        .trim_end_matches("\n//\n")
        .trim_end_matches("\n")
        .split("\n//\n")
        .map(|s| s.to_string())
        .collect();

    app.translation_document.balloons[app.current_balloon].tl_content = tl;
    app.translation_document.balloons[app.current_balloon].pr_content = pr;
    app.translation_document.balloons[app.current_balloon].comments = cmmnts;

    // Proceed to the next balloon

    let max_len = app.translation_document.balloons.len();
    let new = app.current_balloon + 1;

    if new == max_len {
        let new_balloon = Balloon::default();
        app.translation_document.balloons.push(new_balloon);
        app.current_balloon = new;

        // Clear the text inputs
        app.t1_content = text_editor::Content::default();
        app.t2_content = text_editor::Content::default();
        app.t3_content = text_editor::Content::default();
    } else {
        app.current_balloon = new;

        let tl = &app.translation_document.balloons[new].tl_content;
        app.t1_content = text_editor::Content::with_text(tl.join("\n//\n").as_str());

        let pr = &app.translation_document.balloons[new].pr_content;
        app.t2_content = text_editor::Content::with_text(pr.join("\n//\n").as_str());

        let cmmnts = &app.translation_document.balloons[new].comments;
        app.t3_content = text_editor::Content::with_text(cmmnts.join("\n//\n").as_str());
    }

    println!("{:?}", app.translation_document.balloons);
}
