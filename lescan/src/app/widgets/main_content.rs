use super::document_img_viewer::generate_image_viewer;
use crate::message::Message;
use crate::utils::handlers::editor_kp_bindings;
use crate::utils::{panes::MainPanes, tabs::ImageTabs};
use iced::widget::{
    column, container, mouse_area, pane_grid, pick_list, responsive, text, text_editor,
};
use iced::{Fill, Length};
use iced_table::table;

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
        match self {
            BlnTypes::Dialogue => write!(f, "Dialogue"),
            BlnTypes::Square => write!(f, "Square"),
            BlnTypes::Thinking => write!(f, "Thinking"),
            BlnTypes::ST => write!(f, "ST"),
            BlnTypes::OT => write!(f, "OT"),
        }
    }
}

pub fn main_content_pane_grid(app: &crate::app::TestApp) -> pane_grid::PaneGrid<Message> {
    pane_grid::PaneGrid::new(&app.panes, move |_id, pane, _is_max| {
        let title_bar = pane_grid::TitleBar::new(
            container(match pane.id {
                MainPanes::Image => text!("Images"),
                MainPanes::Editor => text!("Editor"),
                MainPanes::Table => text!("Balloons"),
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
                        iced_aw::TabLabel::IconText('\u{1F5CE}', "Document".into()),
                    )
                    .push(
                        ImageTabs::Balloon,
                        iced_aw::TabLabel::IconText('\u{1F5BC}', "Image".into()),
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
                        container(text!("No image for this balloon"))
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
                    .placeholder("Default text...")
                    .on_action(Message::T1ContentChanged)
                    .height(100)
                    .padding(10)
                    .key_binding(editor_kp_bindings);

                let editor_2 = text_editor(&app.t2_content)
                    .placeholder("Default text...")
                    .on_action(Message::T2ContentChanged)
                    .height(100)
                    .padding(10)
                    .key_binding(editor_kp_bindings);

                let editor_3 = text_editor(&app.t3_content)
                    .placeholder("Default text...")
                    .on_action(Message::T3ContentChanged)
                    .height(100)
                    .padding(10)
                    .key_binding(editor_kp_bindings);
                container(column![bln_type_picker, editor_1, editor_2, editor_3].spacing(3))
                    .center(Length::Fill)
            }
            MainPanes::Table => {
                let table = responsive(|size| {
                    let t = table(
                        app.table_header_scroller.clone(),
                        app.table_body_scroller.clone(),
                        &app.columns,
                        &app.translation_document.balloons,
                        Message::SyncHeader,
                    )
                    .on_column_resize(Message::TableColumnResizing, Message::TableColumnResized)
                    .footer(app.table_footer_scroller.clone())
                    .min_width(size.width);

                    t.into()
                });
                container(mouse_area(table))
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
