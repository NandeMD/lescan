use crate::app::TestApp;
use crate::message::Message;
use iced::keyboard::key::{Key, Named};
use iced::widget::{
    self,
    text_editor::{self, Binding, KeyPress, Status},
};
use iced::Task;
use rsff::balloon::Balloon;

const SUPPORTED_IMG_EXTENSIONS: [&str; 12] = [
    "jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp", "avif", "dds", "ff", "hdr", "ico",
];

pub fn message_handler(msg: crate::message::Message, app: &mut TestApp) -> Task<Message> {
    match msg {
        Message::T1ContentChanged(action) => {
            app.t1_content.perform(action);
        }
        Message::T2ContentChanged(action) => {
            app.t2_content.perform(action);
        }
        Message::T3ContentChanged(action) => {
            app.t3_content.perform(action);
        }
        Message::SyncHeader(offset) => {
            app.current_scroll = offset;
        }
        Message::TableColumnResizing(index, offset) => {
            if let Some(col) = app.columns.get_mut(index) {
                col.resize_offset = Some(offset);
            }
        }
        Message::TableColumnResized => app.columns.iter_mut().for_each(|col| {
            if let Some(offset) = col.resize_offset.take() {
                col.width += offset;
            }
        }),
        Message::TabPressed => return iced::widget::focus_next(),
        Message::EnterPressed => handle_enter_key_press(app),
        Message::PaneGridResized(widget::pane_grid::ResizeEvent { split, ratio }) => {
            app.panes.resize(split, ratio);
        }
        Message::PaneGridDragged(widget::pane_grid::DragEvent::Dropped { pane, target }) => {
            app.panes.drop(pane, target);
        }
        Message::PaneGridDragged(_) => {}
        Message::ImageTabSelected(tab) => {
            app.current_img_tab = tab;
        }
        Message::ImageScrolled(vp) => {
            if app.translation_document.images.is_some() {
                app.img_scroller_current_scroll = vp.relative_offset();
                return widget::scrollable::snap_to(
                    app.img_scroller.clone(),
                    app.img_scroller_current_scroll,
                );
            }
        }
        Message::FileDropped(path) => {
            if path.is_file() {
                let ext = path.extension().unwrap().to_str().unwrap().to_lowercase();

                if ["sffx", "sffz", "txt"].contains(&ext.as_str()) {
                    app.translation_document = app
                        .translation_document
                        .open(path.as_os_str().to_str().unwrap())
                        .unwrap()
                        .unwrap();
                } else if SUPPORTED_IMG_EXTENSIONS.contains(&ext.as_str()) {
                    let current_bln = app.current_balloon;
                    let new_img_data = std::fs::read(path).unwrap();
                    app.translation_document.balloons[current_bln].add_image(ext, new_img_data);
                }
            } else if path.is_dir() {
                let mut images_in_path = std::fs::read_dir(path)
                    .unwrap()
                    .filter(|e| {
                        e.is_ok()
                            && e.as_ref().unwrap().path().is_file()
                            && SUPPORTED_IMG_EXTENSIONS.contains(
                                &e.as_ref()
                                    .unwrap()
                                    .path()
                                    .extension()
                                    .unwrap()
                                    .to_str()
                                    .unwrap(),
                            )
                    })
                    .map(|r| r.unwrap().path().to_str().unwrap().to_string())
                    .collect::<Vec<_>>();
                images_in_path.sort();

                app.translation_document.images = Some(images_in_path)
            }
        }
    }
    Task::none()
}

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
}

// Oh man, this was hard!!!
// Thanks to @kiilerix for the help!
// https://discourse.iced.rs/t/a-spesific-keybinding-applies-to-all-text-editors-while-it-should-not/784
pub fn editor_kp_bindings(kp: KeyPress) -> Option<Binding<Message>> {
    let bnd: Option<Binding<Message>> = if kp.key == Key::Named(Named::Enter)
        && kp.modifiers.shift()
        && matches!(kp.status, Status::Focused { .. })
    {
        let c1 = Binding::Insert::<Message>('\n');
        let c2 = Binding::Insert::<Message>('/');
        let c3 = Binding::Insert::<Message>('/');
        let c4 = Binding::Insert::<Message>('\n');

        Some(Binding::Sequence(vec![c1, c2, c3, c4]))
    } else if kp.key == Key::Named(Named::Enter) {
        None
    } else if kp.key == Key::Named(Named::Delete) {
        Some(Binding::Delete)
    } else {
        Binding::from_key_press(kp)
    };
    bnd
}
