pub fn generate_goodbyes(names: Vec<&str>, caps: bool, surname: Option<&str>) {
  if !names.is_empty() {
      for name in names {
          let full_name = match surname {
              Some(surname) => format!("{} {}", name, surname),
              None => name.to_string(),
          };

          if caps {
              println!("Goodbye, {}!", full_name.to_uppercase());
          } else {
              println!("Goodbye, {}!", full_name);
          }
      }
  } else {
      println!("Goodbye, world!");
  }
}