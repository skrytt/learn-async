use chrono::{DateTime, Local};
use ratatui::{style::{Color, Modifier, Style}, text::Span};
use reqwest::StatusCode;
use std::fmt::Display;

pub struct HttpTestSummary {
  time: DateTime<Local>,
  url: String,
  result: HttpResult,
}

impl HttpTestSummary {
  pub fn new(time: DateTime<Local>, url: String, result: HttpResult) -> Self {
    Self {
      time,
      url,
      result,
    }
  }
}

pub enum HttpResult {
  ConnectFail,
  Timeout,
  HttpResponse{status: StatusCode},
}

impl Display for HttpResult {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      HttpResult::ConnectFail => write!(f, "CONNFAIL"),
      HttpResult::Timeout => write!(f, "TIMEOUT"),
      HttpResult::HttpResponse{status} => write!(f, "{}", status),
    }
}
}

pub fn get_test_summary_line<'a>(summary: HttpTestSummary) -> ratatui::text::Line<'a> {
  let result_colour: Color = match summary.result {
    HttpResult::ConnectFail | HttpResult::Timeout => Color::Red,
    HttpResult::HttpResponse{status} => match status.as_u16() {
      200 => Color::Green,
      _ => Color::Red,
    }
  };

  //A ratatui::text::line has Spans, these are individually stylable

  let spans = vec![
    Span::styled(format!(" {} ", summary.time.format("%H:%M:%S")), Style::new().fg(Color::White).bg(Color::DarkGray)),
    Span::styled(format!(" {} ", summary.result), Style::new().add_modifier(Modifier::BOLD).fg(Color::White).bg(result_colour)),
    Span::styled(format!(" {}", summary.url), Style::new().add_modifier(Modifier::BOLD)),
  ];

  return ratatui::text::Line::from(spans);
}
