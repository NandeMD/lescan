use iced::widget::{image, scrollable, text, Column};
use iced::{Element, Length, Renderer, Theme};

use crate::{message::Message, LeScan};

use rust_i18n::t;

// Ask discord about Message::ImageScrolled firing
// after creation so when panes change, scroll state
// returns to RelativeOffset::START

pub fn generate_image_viewer(app: &LeScan) -> Element<'_, Message, Theme, Renderer> {
    if let Some(img_paths) = &app.translation_document.images {
        let img_paths_to_img_widget_iter = img_paths.iter().map(|path_string| {
            let img_handle = image::Handle::from_path(path_string);
            image::Image::new(img_handle).width(Length::Fill).into()
        });
        let img_scroller =
            scrollable(Column::from_iter(img_paths_to_img_widget_iter).width(Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .id(app.img_scroller.clone())
                .on_scroll(Message::ImageScrolled);

        img_scroller.into()
    } else {
        text!("{}", t!("imgtabbar.document_no_img")).into()
    }
}
