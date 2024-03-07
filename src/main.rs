mod http;
mod reactor;
mod tui;
mod user_input;

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

fn main() -> io::Result<()> {
  let mut terminal = tui::init()?;
  let app_result = App::default().run(&mut terminal);
  tui::restore()?;
  app_result
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
  /// runs the application's main loop until the user quits
  pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
      while !self.exit {
          terminal.draw(|frame| self.render_frame(frame))?;
          self.handle_events()?;
      }
      Ok(())
  }

  fn render_frame(&self, frame: &mut Frame) {
    frame.render_widget(self, frame.size());
  }

  fn handle_events(&mut self) -> io::Result<()> {
    match event::read()? {
        // it's important to check that the event is a key press event as
        // crossterm also emits key release and repeat events on Windows.
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            self.handle_key_event(key_event)
        }
        _ => {}
    };
    Ok(())
  }

  fn exit(&mut self) {
    self.exit = true;
  }

  fn increment_counter(&mut self) {
      self.counter += 1;
  }

  fn decrement_counter(&mut self) {
      self.counter -= 1;
  }

  fn handle_key_event(&mut self, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => self.exit(),
        KeyCode::Left => self.decrement_counter(),
        KeyCode::Right => self.increment_counter(),
        _ => {}
    }
  }
}

impl Widget for &App {
  fn render(self, area: Rect, buf: &mut Buffer) {
      let title = Title::from(" Counter App Tutorial ".bold());
      let instructions = Title::from(Line::from(vec![
          " Decrement ".into(),
          "<Left>".blue().bold(),
          " Increment ".into(),
          "<Right>".blue().bold(),
          " Quit ".into(),
          "<Q> ".blue().bold(),
      ]));
      let block = Block::default()
          .title(title.alignment(Alignment::Center))
          .title(
              instructions
                  .alignment(Alignment::Center)
                  .position(Position::Bottom),
          )
          .borders(Borders::ALL)
          .border_set(border::THICK);

      let counter_text = Text::from(vec![Line::from(vec![
          "Value: ".into(),
          self.counter.to_string().yellow(),
      ])]);

      Paragraph::new(counter_text)
          .centered()
          .block(block)
          .render(area, buf);
  }
}

// ================
// === OLD MAIN ===
// ================
//
// use crate::http::{HttpTestSummary, print_http_result};
// use crate::user_input::get_url;
// use crate::reactor::spawn_tokio_thread;
//
// fn main() {
//   let url = get_url();
//   let rx = spawn_tokio_thread(url);
//   loop {
//     let http_test_summary: HttpTestSummary = rx.recv().unwrap();
//     print_http_result(http_test_summary);
//   }
// }