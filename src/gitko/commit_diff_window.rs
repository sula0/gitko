use crate::git;
use crate::ascii_table::*;
use crate::gitko::input_window::InputWindow;
use crate::render::{Renderer, ScreenSize, Position, Colored, Component, KeyHandlers, Line, Window};

pub struct CommitDiffWindow {
    commit_hash: String,
    term: String
}

impl CommitDiffWindow {
    pub fn new(commit_hash: &str) -> CommitDiffWindow {
        CommitDiffWindow {
            commit_hash: commit_hash.to_owned(),
            term: "".to_owned()
        }
    }

    fn move_screen_up(&mut self, window: &mut Window) -> bool {
        window.move_screen_up(1);
        true
    }

    fn move_screen_down(&mut self, window: &mut Window) -> bool {
        window.move_screen_down(1);
        true
    }

    fn jump_screen_up(&mut self, window: &mut Window) -> bool {
        for _ in 0..20 {
            self.move_screen_up(window);
        }

        true
    }

    fn jump_screen_down(&mut self, window: &mut Window) -> bool {
        for _ in 0..20 {
            self.move_screen_down(window);
        }

        true
    }

    fn search_commits(&mut self, window: &mut Window) -> bool {
        self.clear_search();
        let mut search_window = InputWindow::new();

        Renderer::new(
            &mut search_window,
            ScreenSize { lines: 2, cols: window.width() },
            Position { x: 0, y: window.height() - 2 }
        ).render();

        self.term = search_window.text;
        window.move_next(&self.term);

        true
    }

    fn next_search_result(&mut self, window: &mut Window) -> bool {
        window.move_next(&self.term);
        true
    }

    fn prev_search_result(&mut self, window: &mut Window) -> bool {
        window.move_prev(&self.term);
        true
    }

    fn clear_search(&mut self) {
        self.term = "".to_owned();
    }
}

fn map_line(line: String) -> Line {
    if line.starts_with('+') {
        Line::new(vec![
            Box::new(
                Colored::new(
                    line,
                    ncurses::COLOR_GREEN,
                    ncurses::COLOR_BLACK
                )
            )
        ])
    } else if line.starts_with('-') {
        Line::new(vec![
            Box::new(
                Colored::new(
                    line,
                    ncurses::COLOR_RED,
                    ncurses::COLOR_BLACK
                )
            )
        ])
    } else if line.starts_with("@@") {
        Line::new(vec![
            Box::new(
                Colored::new(
                    line,
                    ncurses::COLOR_CYAN,
                    ncurses::COLOR_BLACK
                )
            )
        ])
    } else {
        Line::from_string(line)
    }
}

impl Component<CommitDiffWindow> for CommitDiffWindow {
    fn on_start(&mut self, window: &mut Window) {
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        window.lines = git::diff_commit(&self.commit_hash)
            .iter()
            .map(|l| map_line(l.to_owned()))
            .collect();
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<CommitDiffWindow>) {
        handlers.insert(KEY_J_LOWER, CommitDiffWindow::move_screen_down);
        handlers.insert(KEY_K_LOWER, CommitDiffWindow::move_screen_up);

        handlers.insert(4, CommitDiffWindow::jump_screen_down);
        handlers.insert(21, CommitDiffWindow::jump_screen_up);

        handlers.insert(KEY_N_LOWER, CommitDiffWindow::next_search_result);
        handlers.insert(KEY_N_UPPER, CommitDiffWindow::prev_search_result);
        handlers.insert(KEY_FORWARD_SLASH, CommitDiffWindow::search_commits);
    }
}

impl std::ops::Drop for CommitDiffWindow {
    fn drop(&mut self) {
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_VISIBLE);
    }
}
