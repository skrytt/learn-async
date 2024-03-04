mod http;
mod reactor;
mod user_input;

use crate::http::{HttpTestSummary, print_http_result};
use crate::user_input::get_url;
use crate::reactor::spawn_tokio_thread;

fn main() {
  let url = get_url();

  let rx = spawn_tokio_thread(url);

  loop {
    let http_test_summary: HttpTestSummary = rx.recv().unwrap();
    print_http_result(http_test_summary);
  }
}
