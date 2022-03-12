use crate::git;
use crate::ascii_table::*;
use crate::render::{Component, Renderer, ScreenSize, Window, Position};
use crate::gitko::commit_diff_window::CommitDiffWindow;

pub struct LogWindow { }

impl LogWindow {
    pub fn new() -> LogWindow {
        LogWindow { }
    }

    fn get_commit_log(&mut self, window: &mut Window<LogWindow>) -> bool {
        let line = window.get_cursor_line();
        let trimmed_line = line
            .trim_matches(|c| c == '|' || c == '\\' || c == '*' || c == ' ');

        if trimmed_line.is_empty() {
            return true;
        }

        let commit_hash = &trimmed_line[0..7];

        Renderer::new(
            CommitDiffWindow::new(commit_hash),
            ScreenSize::max(),
            Position::default()
        ).render();

        true
    }
}

impl Component<LogWindow> for LogWindow {
    fn on_start(&mut self, window: &mut Window<LogWindow>) {
        window.data = git::log(None);
    }

    fn register_handlers(&self, window: &mut Window<LogWindow>) {
        window.register_handler(KEY_LF, LogWindow::get_commit_log);
    }
}
