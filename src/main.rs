mod http;
mod reactor;
mod tui;
mod user_input;

use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::KeyCode::Char;
use http::HttpTestSummary;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::{self, UnboundedSender};
use tui::Event;

// App state
struct App<'a> {
    counter: i64,
    should_quit: bool,
    action_tx: UnboundedSender<Action>,
    test_summary_line: Line<'a>,
}

// App actions
#[derive(Clone)]
pub enum Action {
    Tick,
    Increment,
    Decrement,
    NetworkRequestAndThenIncrement, // new
    NetworkRequestAndThenDecrement, // new
    Quit,
    Render,
    None,
}

// App ui render function
fn ui(frame: &mut Frame, app: &mut App) {
  let num_rows: u16 = 1;
  let area = Rect::new(0, frame.size().height-num_rows, frame.size().width, num_rows);

  frame.render_widget(
    app.test_summary_line.clone()
    .style(Style::default().fg(Color::Cyan))
    .alignment(Alignment::Left),
    area,
  );
}

fn get_action(_app: &App, event: Event) -> Action {
    match event {
        Event::Error => Action::None,
        Event::Tick => Action::Tick,
        Event::Render => Action::Render,
        Event::Key(key) => {
            match key.code {
                Char('j') => Action::Increment,
                Char('k') => Action::Decrement,
                Char('J') => Action::NetworkRequestAndThenIncrement, // new
                Char('K') => Action::NetworkRequestAndThenDecrement, // new
                Char('q') => Action::Quit,
                _ => Action::None,
            }
        }
        _ => Action::None,
    }
}

fn update_from_tui(app: &mut App, action: Action) {
    match action {
        Action::Increment => {
            app.counter += 1;
        }
        Action::Decrement => {
            app.counter -= 1;
        }
        Action::NetworkRequestAndThenIncrement => {
            let tx = app.action_tx.clone();
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(5)).await; // simulate network request
                tx.send(Action::Increment).unwrap();
            });
        }
        Action::NetworkRequestAndThenDecrement => {
            let tx = app.action_tx.clone();
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(5)).await; // simulate network request
                tx.send(Action::Decrement).unwrap();
            });
        }
        Action::Quit => app.should_quit = true,
        _ => {}
    };
}

fn update_from_reactor(app: &mut App, test_summary: HttpTestSummary) {
    app.test_summary_line = http::get_test_summary_line(test_summary)
}

async fn run() -> Result<()> {
    let url = user_input::get_url();
    let main_rx_from_reactor = reactor::spawn_tokio_thread(url);

    let (tui_tx, mut main_rx_from_tui) = mpsc::unbounded_channel(); // new

    let mut tui = tui::Tui::new()?.tick_rate(1.0).frame_rate(10.0);
    tui.enter()?;

    let mut app = App {
        counter: 0,
        should_quit: false,
        action_tx: tui_tx.clone(),
        test_summary_line: Line::from("Initial Test Summary String"),
    };

    loop {
        let e = tui.next().await?;
        match e {
            tui::Event::Quit => tui_tx.send(Action::Quit)?,
            tui::Event::Tick => tui_tx.send(Action::Tick)?,
            tui::Event::Render => tui_tx.send(Action::Render)?,
            tui::Event::Key(_) => {
                let action = get_action(&app, e);
                tui_tx.send(action.clone())?;
            }
            _ => {}
        };

        while let Ok(action) = main_rx_from_tui.try_recv() {
            update_from_tui(&mut app, action.clone());

            if let Action::Render = action {
                // First get the latest test summary from the network reactor
                while let Ok(test_summary) = main_rx_from_reactor.try_recv() {
                    update_from_reactor(&mut app, test_summary);
                }

                tui.draw(|f| {
                    ui(f, &mut app);
                })?;
            }
        }

        if app.should_quit {
            break;
        }
    }
    tui.exit()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let result = run().await;

    result?;

    Ok(())
}