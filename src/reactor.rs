use futures::future::{BoxFuture, FutureExt};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::time::Duration;
use tokio::time::sleep;

pub fn spawn_tokio_thread() -> Receiver<String> {
    let (reactor_tx, main_rx) = mpsc::channel();
  
    // Use `move` to pass ownership to spawned thread
    thread::spawn(move || {
      let rt = tokio::runtime::Runtime::new().unwrap();
      rt.block_on(async {
        recursive_http_get_and_wait(reactor_tx).await;
        loop {
          sleep(Duration::from_millis(1000)).await;
        }
      })
    });
  
    main_rx
  }

  const TIMEOUT_MILLIS: u64 = 2000;
  const SLEEP_MILLIS: u64 = 1000;

  // Recursion requires this pattern to workaround compiler limitations
  fn recursive_http_get_and_wait(reactor_tx: Sender<String>) -> BoxFuture<'static, ()> {
    async move {
      let client = reqwest::Client::new();
      let request = client.get("http://localhost:4321/")
        .timeout(Duration::from_millis(TIMEOUT_MILLIS))
        .build()
        .unwrap();

      let result = client.execute(request).await;

      reactor_tx.send(format!("{:?}", result)).unwrap();

      sleep(Duration::from_millis(SLEEP_MILLIS)).await;
      recursive_http_get_and_wait(reactor_tx).await;
    }.boxed()
  }