use super::document_img_viewer::generate_image_viewer;
use crate::message::Message;
use crate::utils::handlers::editor_kp_bindings;
use crate::utils::{panes::MainPanes, tabs::ImageTabs};
use iced::widget::{column, container, pane_grid, pick_list, text, text_editor};
use iced::{Fill, Length, Theme};
use iced_aw::SelectionList;
use rust_i18n::t;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BlnTypes {
    Dialogue,
    Square,
    Thinking,
    ST,
    OT,
}

impl BlnTypes {
    pub const ALL: [Self; 5] = [
        BlnTypes::Dialogue,
        BlnTypes::Square,
        BlnTypes::Thinking,
        BlnTypes::ST,
        BlnTypes::OT,
    ];
}

impl std::fmt::Display for BlnTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            BlnTypes::Dialogue => t!("btype.dialogue"),
            BlnTypes::Square => t!("btype.square"),
            BlnTypes::Thinking => t!("btype.thinking"),
            BlnTypes::ST => t!("btype.st"),
            BlnTypes::OT => t!("btype.ot"),
        };
        write!(f, "{}", text)
    }
}

pub fn main_content_pane_grid(app: &crate::app::TestApp) -> pane_grid::PaneGrid<Message> {
    pane_grid::PaneGrid::new(&app.panes, move |_id, pane, _is_max| {
        let title_bar = pane_grid::TitleBar::new(
            container({
                let title = match pane.id {
                    MainPanes::Image => t!("pg.images"),
                    MainPanes::Editor => t!("pg.editor"),
                    MainPanes::Table => t!("pg.balloons"),
                };
                text!("{}", title)
            })
            .padding(10)
            .style(|_| {
                let thm = app.theme().clone();

                container::Style {
                    border: iced::Border {
                        color: thm.extended_palette().primary.strong.color,
                        width: 2.0,
                        radius: iced::border::Radius {
                            top_left: 5.0,
                            top_right: 5.0,
                            bottom_right: 5.0,
                            bottom_left: 5.0,
                        },
                    },
                    text_color: Some(thm.extended_palette().primary.weak.color),
                    ..Default::default()
                }
            }),
        );

        pane_grid::Content::new(match pane.id {
            MainPanes::Image => {
                let tab_br = iced_aw::TabBar::new(Message::ImageTabSelected)
                    .push(
                        ImageTabs::Document,
                        iced_aw::TabLabel::IconText('\u{1F5CE}', t!("imgtabbar.document").into()),
                    )
                    .push(
                        ImageTabs::Balloon,
                        iced_aw::TabLabel::IconText('\u{1F5BC}', t!("imgtabbar.balloon").into()),
                    )
                    .set_active_tab(&app.current_img_tab);

                let cnt = match app.current_img_tab {
                    ImageTabs::Document => {
                        container(generate_image_viewer(app)).center(Length::Fill)
                    }
                    ImageTabs::Balloon => if let Some(img) =
                        &app.translation_document.balloons[app.current_balloon].balloon_img
                    {
                        let img_handle =
                            iced::widget::image::Handle::from_bytes(img.img_data.clone());
                        container(
                            iced::widget::image::viewer(img_handle)
                                .width(Length::Fill)
                                .height(Length::Fill),
                        )
                    } else {
                        container(text!("{}", t!("imgtabbar.balloon_no_img")))
                    }
                    .center(Length::Fill),
                };

                container(column![tab_br, cnt].spacing(10).padding(10))
            }
            MainPanes::Editor => {
                let bln_type_picker = pick_list(
                    [
                        BlnTypes::Dialogue,
                        BlnTypes::Square,
                        BlnTypes::Thinking,
                        BlnTypes::OT,
                        BlnTypes::ST,
                    ],
                    app.selected_bln_type,
                    Message::BlnTypeSelected,
                )
                .width(Length::Fill)
                .padding(5);
                let editor_1 = text_editor(&app.t1_content)
                    .placeholder(t!("text_editors.translation_editor_placeholder"))
                    .on_action(Message::T1ContentChanged)
                    .height(100)
                    .padding(10)
                    .key_binding(editor_kp_bindings);

                let editor_2 = text_editor(&app.t2_content)
                    .placeholder(t!("text_editors.proofread_editor_placeholder"))
                    .on_action(Message::T2ContentChanged)
                    .height(100)
                    .padding(10)
                    .key_binding(editor_kp_bindings);

                let editor_3 = text_editor(&app.t3_content)
                    .placeholder(t!("text_editors.comment_editor_placeholder"))
                    .on_action(Message::T3ContentChanged)
                    .height(100)
                    .padding(10)
                    .key_binding(editor_kp_bindings);
                container(column![bln_type_picker, editor_1, editor_2, editor_3].spacing(3))
                    .center(Length::Fill)
            }
            MainPanes::Table => {
                let table = SelectionList::new_with(
                    &app.translation_document.balloons,
                    |i, _| Message::BalloonSelected(i),
                    16.0,
                    5,
                    selection_list_style,
                    Some(app.current_balloon),
                    iced::Font::MONOSPACE,
                );
                container(table)
            }
        })
        .title_bar(title_bar)
    })
    .width(Fill)
    .height(Fill)
    .on_drag(Message::PaneGridDragged)
    .on_resize(10, Message::PaneGridResized)
}

#[derive(Debug, Clone, Copy)]
pub struct Pane {
    pub id: MainPanes,
}

fn selection_list_style(
    theme: &Theme,
    status: iced_aw::style::Status,
) -> iced_aw::style::selection_list::Style {
    let ep = theme.extended_palette();
    match status {
        iced_aw::style::Status::Hovered => iced_aw::style::selection_list::Style {
            text_color: ep.background.weak.text,
            background: ep.background.weak.color.into(),
            border_width: 1.0,
            border_color: ep.primary.strong.color,
        },
        iced_aw::style::Status::Selected => iced_aw::style::selection_list::Style {
            text_color: ep.background.weak.text,
            background: ep.primary.weak.color.into(),
            border_width: 1.0,
            border_color: ep.primary.strong.color,
        },
        _ => iced_aw::style::selection_list::Style {
            text_color: ep.primary.weak.color,
            background: iced::Color::TRANSPARENT.into(),
            border_width: 1.0,
            border_color: iced::Color::TRANSPARENT,
        },
    }
}
