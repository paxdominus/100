use std::io;
use chrono::prelude::*;

fn main() {
  let name = input("What is your name? ").expect("Something went wrong!");
  println!("Hello {}!", name);

  let age = input("What is yout age? ")
  .expect("Failed to get age")
  .parse::<i32>().expect("Invalid age.");
  
    let current_year = Utc::now().year();
    let hundred_year = 100 - age + current_year;

    
  println!("You are {} years old", age);

  if age > 100{
    println!("Hey grands {} you turned 100 in the year {}", name , hundred_year);
  } else {
  println!("Hey {} you'll turn 100 in the year {}", name, hundred_year);
  }
}


fn input(user_message: &str) -> io::Result<String> {
    use std::io::Write;
    println!("{}", user_message);
    io::stdout().flush()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}