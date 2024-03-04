use ansi_term::{
  Colour::{Green, Red, RGB},
  Style,
};
use chrono::{DateTime, Local};
use reqwest::StatusCode;
use std::fmt::Display;

const DARKGREY: ansi_term::Colour = RGB(60, 60, 60);

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

pub fn print_http_result(summary: HttpTestSummary) {
  let result_colour: ansi_term::Colour = match summary.result {
    HttpResult::ConnectFail | HttpResult::Timeout => Red,
    HttpResult::HttpResponse{status} => match status.as_u16() {
      200 => Green,
      _ => Red,
    }
  };

  println!("{}{} {}", 
    Style::new().on(DARKGREY).paint(format!(" {} ", summary.time.format("%H:%M:%S"))),
    Style::new().bold().on(result_colour).paint(format!(" {} ", summary.result)),
    Style::new().bold().paint(summary.url)
  )
}



