use iced::widget::{pane_grid, scrollable, text_editor};

#[derive(Debug, Clone)]
pub enum Message {
    T1ContentChanged(text_editor::Action),
    T2ContentChanged(text_editor::Action),
    T3ContentChanged(text_editor::Action),

    TabPressed,
    EnterPressed,
    SyncHeader(scrollable::AbsoluteOffset),

    TableColumnResizing(usize, f32),
    TableColumnResized,

    PaneGridDragged(pane_grid::DragEvent),
    PaneGridResized(pane_grid::ResizeEvent),

    ImageTabSelected(usize),
    ImageScrolled(scrollable::Viewport),
    FileDropped(std::path::PathBuf),
}
