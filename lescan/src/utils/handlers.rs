use crate::app::widgets::main_content::BlnTypes;
use crate::app::TestApp;
use crate::message::*;
use crate::utils::dialog_windows;
use iced::keyboard::key::{Key, Named};
use iced::widget::{
    self,
    text_editor::{self, Binding, KeyPress, Status},
};
use iced::Task;
use rsff::balloon::Balloon;
use rsff::TYPES;

use super::tabs::ImageTabs;

use rust_i18n::t;

const SUPPORTED_IMG_EXTENSIONS: [&str; 12] = [
    "jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp", "avif", "dds", "ff", "hdr", "ico",
];

pub fn message_handler(msg: crate::message::Message, app: &mut TestApp) -> Task<Message> {
    match msg {
        Message::BlnTypeSelected(bln_type) => {
            app.selected_bln_type = Some(bln_type);
            match app.selected_bln_type {
                Some(BlnTypes::Dialogue) => {
                    app.translation_document.balloons[app.current_balloon].btype =
                        rsff::TYPES::DIALOGUE
                }
                Some(BlnTypes::Thinking) => {
                    app.translation_document.balloons[app.current_balloon].btype =
                        rsff::TYPES::THINKING
                }
                Some(BlnTypes::Square) => {
                    app.translation_document.balloons[app.current_balloon].btype =
                        rsff::TYPES::SQUARE
                }
                Some(BlnTypes::OT) => {
                    app.translation_document.balloons[app.current_balloon].btype = rsff::TYPES::OT
                }
                Some(BlnTypes::ST) => {
                    app.translation_document.balloons[app.current_balloon].btype = rsff::TYPES::ST
                }
                _ => {}
            }
        }
        Message::T1ContentChanged(action) => {
            app.t1_content.perform(action);
            handle_text_input_balloon_type_selection(
                &mut app.t1_content,
                &mut app.translation_document.balloons[app.current_balloon],
                &mut app.selected_bln_type,
            );
        }
        Message::T2ContentChanged(action) => {
            app.t2_content.perform(action);
            handle_text_input_balloon_type_selection(
                &mut app.t2_content,
                &mut app.translation_document.balloons[app.current_balloon],
                &mut app.selected_bln_type,
            );
        }
        Message::T3ContentChanged(action) => {
            app.t3_content.perform(action);
        }
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
                let ext = {
                    if let Some(extension) = path.extension() {
                        if let Some(ext) = extension.to_str() {
                            ext.to_string()
                        } else {
                            return Task::future(async move {
                                dialog_windows::show_error_dialog(
                                    t!("errors.extension_error.title"),
                                    t!("errors.extension_error.description", name = path.display()),
                                )
                                .await
                            })
                            .then(|_| Task::none());
                        }
                    } else {
                        return Task::future(async move {
                            dialog_windows::show_error_dialog(
                                t!("errors.extension_error.title"),
                                t!("errors.extension_error.description", name = path.display()),
                            )
                            .await
                        })
                        .then(|_| Task::none());
                    }
                };

                if ["sffx", "sffz", "txt"].contains(&ext.as_str()) {
                    app.current_balloon = 0;
                    app.translation_document = {
                        match rsff::Document::open(&path) {
                            Ok(doc) => doc,
                            Err(e) => {
                                let e = e.to_string();
                                return Task::future(async move {
                                    dialog_windows::show_error_dialog(
                                        t!("errors.open_file_error.title"),
                                        t!(
                                            "errors.open_file_error.description",
                                            p = path.display(),
                                            e = e
                                        ),
                                    )
                                    .await
                                })
                                .then(|_| Task::none());
                            }
                        }
                    };
                    app.document_file_location = Some(path.display().to_string());
                } else if SUPPORTED_IMG_EXTENSIONS.contains(&ext.as_str()) {
                    let current_bln = app.current_balloon;
                    match std::fs::read(&path) {
                        Ok(new_img_data) => app.translation_document.balloons[current_bln]
                            .add_image(ext, new_img_data),
                        Err(e) => {
                            let e = e.to_string();
                            return Task::future(async move {
                                dialog_windows::show_error_dialog(
                                    t!("errors.read_img_file_error.title"),
                                    t!(
                                        "errors.read_img_file_error.description",
                                        p = path.display(),
                                        e = e
                                    ),
                                )
                                .await
                            })
                            .then(|_| Task::none());
                        }
                    }
                }
            } else if path.is_dir() {
                let mut images_in_path = std::fs::read_dir(path)
                    .unwrap()
                    .filter(|e| {
                        e.is_ok()
                            && e.as_ref().unwrap().path().is_file()
                            && e.as_ref().unwrap().path().extension().is_some()
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
        Message::BalloonTypeCycleDown => {
            if let Some(current_bln_type) = app.selected_bln_type {
                let cbi = (BlnTypes::ALL
                    .iter()
                    .position(|bt| *bt == current_bln_type)
                    .unwrap()
                    + 1)
                    % BlnTypes::ALL.len();
                app.selected_bln_type = Some(BlnTypes::ALL[cbi]);
                app.translation_document.balloons[app.current_balloon].btype = [
                    TYPES::DIALOGUE,
                    TYPES::SQUARE,
                    TYPES::THINKING,
                    TYPES::ST,
                    TYPES::OT,
                ][cbi]
                    .clone();
            } else {
                app.selected_bln_type = Some(BlnTypes::ALL[0]);
                app.translation_document.balloons[app.current_balloon].btype = [
                    TYPES::DIALOGUE,
                    TYPES::SQUARE,
                    TYPES::THINKING,
                    TYPES::ST,
                    TYPES::OT,
                ][0]
                .clone()
            }
        }
        Message::BalloonTypeCycleUp => {
            if let Some(current_bln_type) = app.selected_bln_type {
                let cbi = BlnTypes::ALL
                    .iter()
                    .position(|bt| *bt == current_bln_type)
                    .unwrap();
                let cbi = if cbi == 0 {
                    BlnTypes::ALL.len() - 1
                } else {
                    cbi - 1
                };
                println!("{cbi}");
                app.selected_bln_type = Some(BlnTypes::ALL[cbi]);
                app.translation_document.balloons[app.current_balloon].btype = [
                    TYPES::DIALOGUE,
                    TYPES::SQUARE,
                    TYPES::THINKING,
                    TYPES::ST,
                    TYPES::OT,
                ][cbi]
                    .clone();
            } else {
                app.selected_bln_type = Some(BlnTypes::ALL[0]);
                app.translation_document.balloons[app.current_balloon].btype = [
                    TYPES::DIALOGUE,
                    TYPES::SQUARE,
                    TYPES::THINKING,
                    TYPES::ST,
                    TYPES::OT,
                ][0]
                .clone();
            }
        }
        Message::CurrentBlnImgPaste => clipboard_img_paste(app),
        Message::FileOperation(file_op) => match file_op {
            FileOperation::NewFileDialog => {
                return Task::future(async {
                    let dywt = rfd::AsyncMessageDialog::new()
                        .set_title(t!("dialog_windows.new_document.title"))
                        .set_description(t!("dialog_windows.new_document.description"))
                        .set_level(rfd::MessageLevel::Warning)
                        .set_buttons(rfd::MessageButtons::YesNo)
                        .show()
                        .await;
                    if dywt == rfd::MessageDialogResult::Yes {
                        Some(())
                    } else {
                        None
                    }
                })
                .and_then(|_| Task::done(Message::FileOperation(FileOperation::New)))
            }
            FileOperation::New => {
                app.selected_bln_type = Some(BlnTypes::Dialogue);
                app.t1_content = text_editor::Content::default();
                app.t2_content = text_editor::Content::default();
                app.t3_content = text_editor::Content::default();
                app.current_balloon = 0;
                app.current_img_tab = ImageTabs::Document;
                app.document_file_location = None;
                app.translation_document = rsff::Document::default();
                app.translation_document.add_balloon_empty();
            }
            FileOperation::Open => {
                return Task::future(async {
                    rfd::AsyncFileDialog::new()
                        .add_filter("RSFF", &["txt", "sffx", "sffz"])
                        .set_title(t!("dialog_windows.open_document.title"))
                        .pick_file()
                        .await
                })
                .and_then(|f_p_h| Task::done(Message::FileDropped(f_p_h.into())))
            }
            FileOperation::Save(save_location) => {
                if let Some(save_location) = save_location {
                    let save_res = app.translation_document.save(&save_location);

                    #[cfg(debug_assertions)]
                    println!("Saved to {:?}", save_location);

                    if let Err(save_error) = save_res {
                        return Task::future(async move {
                            rfd::AsyncMessageDialog::new()
                                .set_description(format!(
                                    "{}\n{}",
                                    save_location.display(),
                                    save_error
                                ))
                                .set_title(t!("dialog_windows.errors.error_while_saving.title"))
                                .show()
                                .await;
                        })
                        .then(|_| Task::none());
                    } else {
                        app.document_file_location = Some(save_location.display().to_string())
                    }
                }
            }
            FileOperation::SaveFileDialog => {
                let dfl = app.document_file_location.clone();
                return Task::perform(
                    async move {
                        if let Some(ref location) = dfl {
                            Some(std::path::PathBuf::from(location))
                        } else {
                            rfd::AsyncFileDialog::new()
                                .add_filter("RSFF", &["sffz"])
                                .set_title(t!("dialog_windows.save_document.title"))
                                .set_can_create_directories(true)
                                .set_file_name("scan.sffz")
                                .save_file()
                                .await
                                .map(|t| t.into())
                        }
                    },
                    |pb| Message::FileOperation(FileOperation::Save(pb)),
                );
            }
            FileOperation::SaveAsFileDialog => {
                return Task::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .add_filter(
                                t!("dialog_windows.save_as_document.filter_sffz"),
                                &["sffz"],
                            )
                            .add_filter(
                                t!("dialog_windows.save_as_document.filter_sffx"),
                                &["sffx"],
                            )
                            .add_filter(t!("dialog_windows.save_as_document.filter_txt"), &["txt"])
                            .add_filter(
                                t!("dialog_windows.save_as_document.filter_docx"),
                                &["docx"],
                            )
                            .set_title(t!("dialog_windows.save_as_document.title"))
                            .set_can_create_directories(true)
                            .set_file_name("scan")
                            .save_file()
                            .await
                            .map(|t| t.into())
                    },
                    |pb| Message::FileOperation(FileOperation::Save(pb)),
                )
            }
        },
        Message::BalloonSelected(i) => {
            app.current_balloon = i;
            app.t1_content = text_editor::Content::with_text(
                app.translation_document.balloons[i]
                    .tl_content
                    .join("\n//\n")
                    .as_str(),
            );
            app.t2_content = text_editor::Content::with_text(
                app.translation_document.balloons[i]
                    .pr_content
                    .join("\n//\n")
                    .as_str(),
            );
            app.t3_content = text_editor::Content::with_text(
                app.translation_document.balloons[i]
                    .comments
                    .join("\n//\n")
                    .as_str(),
            );
            app.selected_bln_type = Some({
                match app.translation_document.balloons[i].btype {
                    TYPES::DIALOGUE => BlnTypes::Dialogue,
                    TYPES::OT => BlnTypes::OT,
                    TYPES::SQUARE => BlnTypes::Square,
                    TYPES::ST => BlnTypes::ST,
                    TYPES::THINKING => BlnTypes::Thinking,
                }
            });
        }
        Message::ShowModal(modal_type) => {
            app.show_modal = Some(modal_type);
        }
        Message::HideModal(_) => {
            app.show_modal = None;
        }
        Message::LinkClicked(url) => match open::that_detached(url.to_string()) {
            Ok(_) => {}
            Err(e) => {
                return Task::future(async move {
                    dialog_windows::show_error_dialog(
                        t!("errors.could_not_open_url.title"),
                        t!("errors.could_not_open_url.description", p = url, e = e),
                    )
                    .await
                })
                .then(|_| Task::none());
            }
        },
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

fn handle_text_input_balloon_type_selection(
    text_editor_content: &mut text_editor::Content,
    current_balloon: &mut Balloon,
    selected_bln_type: &mut Option<BlnTypes>,
) {
    let editor_text = text_editor_content.text();

    if editor_text.starts_with("ST:") {
        if let Some(stripped) = editor_text.strip_prefix("ST:") {
            *text_editor_content = text_editor::Content::with_text(stripped);
            current_balloon.btype = TYPES::ST;
            *selected_bln_type = Some(BlnTypes::ST);
        }
    } else if editor_text.starts_with("OT:") {
        if let Some(stripped) = editor_text.strip_prefix("OT:") {
            *text_editor_content = text_editor::Content::with_text(stripped);
            current_balloon.btype = TYPES::OT;
            *selected_bln_type = Some(BlnTypes::OT);
        }
    } else if editor_text.starts_with("[]:") {
        if let Some(stripped) = editor_text.strip_prefix("[]:") {
            *text_editor_content = text_editor::Content::with_text(stripped);
            current_balloon.btype = TYPES::SQUARE;
            *selected_bln_type = Some(BlnTypes::Square);
        }
    } else if editor_text.starts_with("():") {
        if let Some(stripped) = editor_text.strip_prefix("():") {
            *text_editor_content = text_editor::Content::with_text(stripped);
            current_balloon.btype = TYPES::DIALOGUE;
            *selected_bln_type = Some(BlnTypes::Dialogue);
        }
    } else if editor_text.starts_with("{}:") {
        if let Some(stripped) = editor_text.strip_prefix("{}:") {
            *text_editor_content = text_editor::Content::with_text(stripped);
            current_balloon.btype = TYPES::THINKING;
            *selected_bln_type = Some(BlnTypes::Thinking);
        }
    }
}

fn clipboard_img_paste(app: &mut TestApp) {
    #[cfg(target_os = "windows")]
    {
        use clipboard_win::{formats, Clipboard, Getter};
        if let Ok(_clip) = Clipboard::new_attempts(10) {
            let mut img_bmp_data: Vec<u8> = vec![];
            if formats::Bitmap.read_clipboard(&mut img_bmp_data).is_ok() {
                app.translation_document.balloons[app.current_balloon]
                    .add_image("bmp".into(), img_bmp_data);
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        use clipboard_rs::{common::RustImage, Clipboard, ClipboardContext};
        if let Ok(clipboard) = ClipboardContext::new() {
            if let Ok(rust_img) = clipboard.get_image() {
                // let _ = rust_img.to_bitmap().unwrap();

                if let Ok(img_png) = rust_img.to_png() {
                    let img_data = img_png.get_bytes().to_vec();
                    app.translation_document.balloons[app.current_balloon]
                        .add_image("png".into(), img_data);
                }
            }
        }
    }
}
