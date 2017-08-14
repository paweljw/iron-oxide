use std::io;

fn main() {
  let mut text = String::new();
  io::stdin().read_line(&mut text).expect("Failed to read line");

  println!("First word is {}", first_word(&text));
  println!("Second word is {}", second_word(&text));
}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
        return &s[0..i];
      }
  }
  &s[..]
}

fn second_word(s: &String) -> &str {
  let bytes = s.as_bytes();
  let mut first_index = 0;

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
        if first_index == 0 {
          first_index = i;
        } else {
          return &s[first_index + 1..i];
        }
      }
  }

  if first_index == 0 {
    return ""
  } else {
    &s[first_index + 1..]
  }
}