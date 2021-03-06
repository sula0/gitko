use gitko_render::{Component, Line, Window};

pub struct TextWindow<'text> {
    pub lines: Vec<&'text str>
}

impl <'text> Component<TextWindow<'text>> for TextWindow<'text> {
    fn on_start(&mut self, window: &mut Window) {
        window.show_cursor(false);
        window.set_lines(
            self.lines
                .iter()
                .map(|s| Line::plain(s))
                .collect()
        );
    }
}
