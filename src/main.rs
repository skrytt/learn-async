mod user_option;

use ansi_term::{
  Colour::{Green, Red, RGB},
  Style,
};
use inquire::{
    InquireError, Select
};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;
use tokio::time::sleep;
use user_option::UserOption;

fn get_test_type() -> Result<UserOption, InquireError> {
    let options = vec![
        UserOption::new("HTTP".to_string()),
        UserOption::new("TCP".to_string()),
        UserOption::new("DNS".to_string()),
        UserOption::new("CloudFormation".to_string()),
    ];

    let result = Select::new("Select Test Mode:", options).prompt()?;
    Ok(result)
}

// TODO: a way to spawn this in a separate thread to TUI
fn spawn_tokio_thread() -> Receiver<String> {
  let (reactor_tx, main_rx) = mpsc::channel();

  // Use `move` to pass ownership to spawned thread
  thread::spawn(move || {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
      let mut i = 0;
      let sleep_millis = 1000;
      loop {
        sleep(Duration::from_millis(sleep_millis)).await;
        i += sleep_millis;
        let val = format!("Slept {} millis", i);
        reactor_tx.send(val).unwrap();
      }
    })
  });

  main_rx
}

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
  println!("{}", rx.recv().unwrap());
}
