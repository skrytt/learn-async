use inquire::Text;

pub fn get_url() -> String {

  let url = Text::new("Enter URL to test:")
    .with_initial_value("http://localhost:4321")
    .prompt();

  match url {
    Err(_) => std::process::exit(1),
    Ok(s) => s,
  }
}