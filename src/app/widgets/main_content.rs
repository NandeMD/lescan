use crate::message::Message;
use crate::utils::handlers::editor_kp_bindings;
use iced::widget::{column, container, pane_grid, responsive, text, text_editor};
use iced::{Fill, Length};
use iced_table::table;

pub fn main_content_pane_grid(app: &crate::app::TestApp) -> pane_grid::PaneGrid<Message> {
    pane_grid::PaneGrid::new(&app.panes, move |_id, pane, _is_max| {
        let title_bar = pane_grid::TitleBar::new(
            container(match pane.id {
                0 => text!("Images"),
                1 => text!("Editor"),
                2 => text!("Balloons"),
                _ => panic!("Wut id is dis?!"),
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
            0 => {
                let tab_br = iced_aw::TabBar::new(Message::ImageTabSelected)
                    .push(
                        0,
                        iced_aw::TabLabel::IconText('\u{1F5CE}', "Document".into()),
                    )
                    .push(1, iced_aw::TabLabel::IconText('\u{1F5BC}', "Image".into()))
                    .set_active_tab(&app.current_img_tab);

                let cnt = match app.current_img_tab {
                    0 => {
                        if let Some(_imgs) = &app.translation_document.images {
                            container(text!("This will be images"))
                        } else {
                            container(text!("There is no image in this document"))
                                .center(Length::Fill)
                        }
                    }
                    1 => if let Some(img) =
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
                    _ => panic!("WHAT TAB IS DIS?!"),
                };

                container(column![tab_br, cnt].spacing(10).padding(10))
            }
            1 => {
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
                container(column![editor_1, editor_2, editor_3].spacing(3)).center(Length::Fill)
            }
            2 => {
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
                container(table)
            }
            _ => panic!("What is dis pane?!"),
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
    pub id: usize,
}
