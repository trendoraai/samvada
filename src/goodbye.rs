pub fn generate_goodbyes(names: Vec<&str>, caps: bool, surname: Option<&str>, date_after: Option<&str>) {
  if !names.is_empty() {
      for name in names {
          let full_name = match surname {
              Some(surname) => format!("{} {}", name, surname),
              None => name.to_string(),
          };

          let message = if caps {
              format!("Goodbye, {}!", full_name.to_uppercase())
          } else {
              format!("Goodbye, {}!", full_name)
          };

          if let Some(date) = date_after {
              println!("{}, see you after {}.", message, date);
          } else {
              println!("{}", message);
          }
      }
  } else {
      println!("Goodbye, world!");
  }
}