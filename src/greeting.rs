pub fn generate_greetings(names: Vec<&str>, caps: bool, surname: Option<&str>) {
  if !names.is_empty() {
      for name in names {
          let full_name = match surname {
              Some(surname) => format!("{} {}", name, surname),
              None => name.to_string(),
          };

          if caps {
              println!("Hello, {}!", full_name.to_uppercase());
          } else {
              println!("Hello, {}!", full_name);
          }
      }
  } else {
      println!("Hello, world!");
  }
}