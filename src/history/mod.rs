pub mod history;
pub mod history_manager;

pub use history::{HistoryManager, HistoryAction, HistorySnapshot, ActionType};
pub use history_manager::{DocumentHistoryManager, HistoryEntry, UndoRedoState, HistoryListener};
