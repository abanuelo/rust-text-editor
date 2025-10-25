use crate::editor::buffer::Buffer;
use crate::editor::terminal::{Size, Terminal};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    pub buffer: Buffer,
}

impl View {
    fn draw_welcome_message(&self) -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_message.len();
        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit to the left or right.
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;

        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;
        Ok(())
    }
    fn draw_empty_row(&self) -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }
    pub fn load(&mut self, path: &str) -> Result<(), Error> {
        self.buffer.buffer.clear();
        let file_contents = std::fs::read_to_string(path)?;
        for line in file_contents.lines() {
            self.buffer.buffer.push(String::from(line));
        }
        Ok(())
    }

    pub fn render(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;

        for current_row in 0..height {
            Terminal::clear_line()?;
            // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
            // it's allowed to be a bit up or down
            if self.buffer.is_empty() {
                #[allow(clippy::integer_division)]
                if current_row == height / 3 {
                    self.draw_welcome_message()?;
                } else {
                    self.draw_empty_row()?;
                }
            } else {
                if let Some(line) = self.buffer.buffer.get(current_row) {
                    Terminal::print(line)?;
                    Terminal::print("\r\n")?;
                } else {
                    self.draw_empty_row()?;
                }
            }

            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}
