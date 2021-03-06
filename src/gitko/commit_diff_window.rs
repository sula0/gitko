use crate::git;
use crate::gitko::diff_display::color_diff_line;
use crate::searchable::{SearchableComponent, register_search_handlers};
use gitko_render::{Component, KeyHandlers, Window};

use gitko_common::ascii_table::{KEY_J_LOWER, KEY_K_LOWER};

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
}

impl Component<CommitDiffWindow> for CommitDiffWindow {
    fn on_start(&mut self, window: &mut Window) {
        window.show_cursor(false);

        window.set_lines(
            git::diff_commit(&self.commit_hash)
                .iter()
                .map(|l| color_diff_line(l))
                .collect()
        );
    }

    fn on_exit(&mut self, window: &mut Window) {
        window.show_cursor(true);
    }

    fn register_handlers(&self, handlers: &mut KeyHandlers<CommitDiffWindow>) {
        handlers.insert(KEY_J_LOWER, CommitDiffWindow::move_screen_down);
        handlers.insert(KEY_K_LOWER, CommitDiffWindow::move_screen_up);

        handlers.insert(4, CommitDiffWindow::jump_screen_down);
        handlers.insert(21, CommitDiffWindow::jump_screen_up);

        register_search_handlers(handlers);
    }
}

impl SearchableComponent<CommitDiffWindow> for CommitDiffWindow {
    fn term(&self) -> String {
        self.term.clone()
    }

    fn set_term(&mut self, term: String) {
        self.term = term;
    }
}
