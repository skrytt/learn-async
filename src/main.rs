mod http;
mod reactor;
mod user_option;

use crate::http::{HttpTestSummary, print_http_result};
use crate::user_option::get_test_type;
use crate::reactor::spawn_tokio_thread;

fn main() {
  if let Err(e) = get_test_type() {
    println!("{}", e);
  }

  let rx = spawn_tokio_thread();
  loop {
    let http_test_summary: HttpTestSummary = rx.recv().unwrap();
    print_http_result(http_test_summary);
  }
}
