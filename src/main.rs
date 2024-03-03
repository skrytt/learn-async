mod reactor;
mod user_option;

use ansi_term::{
  Colour::{Green, Red, RGB},
  Style,
};

use crate::user_option::get_test_type;
use crate::reactor::spawn_tokio_thread;

fn main() {
  match get_test_type() {
    Err(e) => println!("{}", e),
    _ => (),
  }

  const DARKGREY: ansi_term::Colour = RGB(60, 60, 60);

  println!("{}{} {}", 
    Style::new().on(DARKGREY).paint(" 14:45:54 "),
    Style::new().bold().on(Green).paint(" 200 ↻ "),
    Style::new().bold().paint("https://www.rust-lang.org")
  );
  println!("{}{} {}", 
    Style::new().on(DARKGREY).paint(" 14:45:59 "),
    Style::new().bold().on(Red).paint(" TIMEOUT [5s] "),
    Style::new().bold().paint("https://www.rust-lang.org")
  );

  let rx = spawn_tokio_thread();
  loop {
    let received: String = rx.recv().unwrap();
    println!("{}", received);
  }
}
