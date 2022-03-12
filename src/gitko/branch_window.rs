use crate::git;
use crate::ascii_table::*;
use crate::render::{Component, Renderer, ScreenSize, Window, Position};

use crate::gitko::prompt_window::PromptWindow;

pub struct BranchWindow {
    data: Vec<String>,
}

impl BranchWindow {
    pub fn new() -> BranchWindow {
        BranchWindow { data: vec![] }
    }

    fn open_delete_branch_prompt(
        &mut self, window:
        &mut Window<BranchWindow>) -> bool {
        let line = window.get_cursor_line();

        if !line.starts_with('*') {
            let branch = line.trim();
            let prompt = PromptWindow::new(
                &format!("Are you sure you want to delete branch '{}'? y/n", branch),
                || { git::delete_branch(branch); },
                || { });

            Renderer::new(
                prompt,
                ScreenSize { lines: 1, cols: 0 }, // TODO
                Position { x: 0, y: window.height() - 1 }
            ).render();

            self.get_branches();
        }

        true
    }

    fn checkout_branch(&mut self, window: &mut Window<BranchWindow>) -> bool {
        let line = window.get_cursor_line();
        if !line.starts_with('*') {
            git::checkout_branch(line.trim());
        }

        self.get_branches();

        true
    }

    fn get_branches(&mut self) {
        self.data = git::branch();
    }
}

impl Component<BranchWindow> for BranchWindow {
    fn on_start(&mut self, _window: &mut Window<BranchWindow>) {
        self.get_branches();
    }

    fn data(&self) -> &[String] {
        &self.data
    }

    fn register_handlers(&self, window: &mut Window<BranchWindow>) {
        window.register_handler(KEY_LF, BranchWindow::checkout_branch);
        window.register_handler(KEY_D_LOWER, BranchWindow::open_delete_branch_prompt);
    }
}
