use chrono::Local;
use futures::future::{BoxFuture, FutureExt};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::time::Duration;
use tokio::time::sleep;

use crate::http::{HttpTestSummary, HttpResult};

pub fn spawn_tokio_thread(url: String) -> Receiver<HttpTestSummary> {
    let (reactor_tx, main_rx) = mpsc::channel();
  
    // Use `move` to pass ownership to spawned thread
    thread::spawn(move || {
      let rt = tokio::runtime::Runtime::new().unwrap();
      rt.block_on(async {
        recursive_http_get_and_wait(reactor_tx, url).await;
        loop {
          sleep(Duration::from_millis(1000)).await;
        }
      })
    });
  
    main_rx
  }

  const TIMEOUT_MILLIS: u64 = 2000;
  const SLEEP_MILLIS: u64 = 5000;

  // Recursion requires this pattern to workaround compiler limitations
  fn recursive_http_get_and_wait(reactor_tx: Sender<HttpTestSummary>, url: String) -> BoxFuture<'static, ()> {
    let url_cloned = url.clone();
    async move {
      let client = reqwest::Client::new();
      let time = Local::now();
      let request = client.get(url.clone())
        .timeout(Duration::from_millis(TIMEOUT_MILLIS))
        .build()
        .unwrap();

      let result = client.execute(request).await;

      let result: HttpResult = match result {
        Ok(resp) => {
          HttpResult::HttpResponse{status: resp.status()}
        },
        Err(e) => {
          if e.is_connect() {
            HttpResult::ConnectFail
          }
          else if e.is_timeout() {
            HttpResult::Timeout
          }
          else if let Some(status) = e.status() {
            HttpResult::HttpResponse{status}
          }
          else {
            panic!("Don't know how to handle reqwest::Error: {}", e);
          }
        }
      };

      let http_test_summary = HttpTestSummary::new(time, url, result);
      reactor_tx.send(http_test_summary).unwrap();

      sleep(Duration::from_millis(SLEEP_MILLIS)).await;
      recursive_http_get_and_wait(reactor_tx, url_cloned).await;
    }.boxed()
  }