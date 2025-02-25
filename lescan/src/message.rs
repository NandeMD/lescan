use crate::app::modals::ModalType;
use crate::app::widgets::main_content::BlnTypes;
use iced::widget::{pane_grid, scrollable, text_editor};

#[derive(Debug, Clone)]
pub enum Message {
    BlnTypeSelected(BlnTypes),
    T1ContentChanged(text_editor::Action),
    T2ContentChanged(text_editor::Action),
    T3ContentChanged(text_editor::Action),

    TabPressed,
    EnterPressed,

    PaneGridDragged(pane_grid::DragEvent),
    PaneGridResized(pane_grid::ResizeEvent),

    ImageTabSelected(crate::utils::tabs::ImageTabs),
    ImageScrolled(scrollable::Viewport),
    FileDropped(std::path::PathBuf),

    BalloonTypeCycleUp,
    BalloonTypeCycleDown,
    CurrentBlnImgPaste,

    FileOperation(FileOperation),
    BalloonSelected(usize),

    ShowModal(ModalType),
    HideModal(ModalType),

    LinkClicked(iced::widget::markdown::Url),

    ExitApp,
}

#[derive(Debug, Clone)]
pub enum FileOperation {
    New,
    Open,
    Save(Option<std::path::PathBuf>),
    SaveFileDialog,
    SaveAsFileDialog,
    NewFileDialog,
}
