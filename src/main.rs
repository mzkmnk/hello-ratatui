use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

fn main() -> Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();

    let app_result = App::new().run(terminal);
}

struct App {
    input: String,
    charactor_index: usize,
    input_mode: InputMode,
    messages: Vec<String>,
}

enum InputMode {
    Normal,
    Editing,
}

impl App {
    const fn new() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            charactor_index: 0,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.charactor_index.saturating_sub(1);

        self.charactor_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.charactor_index.saturating_add(1);

        self.charactor_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();

        self.input.insert(index, new_char);

        self.move_cursor_right();
    }

    fn byte_index(&mut self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.charactor_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.charactor_index != 0;

        if is_not_cursor_leftmost {
            let current_index = self.charactor_index;

            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);

            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();

            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&mut self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.charactor_index = 0;
    }

    fn submit_message(&mut self) {
        self.messages.push(self.input.clone());
        self.input.clear();
        self.reset_cursor();
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        _ => {
                            todo!()
                        }
                    },

                    InputMode::Editing => {
                        todo!()
                    }

                    _ => {
                        todo!()
                    }
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        todo!()
    }
}
