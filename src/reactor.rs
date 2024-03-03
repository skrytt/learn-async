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
        recursive(reactor_tx, 0).await;
        loop {
          sleep(Duration::from_millis(1000)).await;
        }
      })
    });
  
    main_rx
  }

  const SLEEP_MILLIS: u64 = 1000;

  // Recursion requires this pattern to workaround compiler limitations
  fn recursive(reactor_tx: Sender<String>, mut total_slept_millis: u64) -> BoxFuture<'static, ()> {
    async move {
        sleep(Duration::from_millis(SLEEP_MILLIS)).await;
        total_slept_millis += SLEEP_MILLIS;
        let val: String = format!("Slept {} millis", total_slept_millis);
        reactor_tx.send(val).unwrap();
        recursive(reactor_tx, total_slept_millis).await;
    }.boxed()
  }