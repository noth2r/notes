use std::io::{self, Write};

pub fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();
}

pub fn input() -> Result<String, io::Error> {
    let mut input = String::new();

    if let Err(error) = io::stdin().read_line(&mut input) {
        return Err(error);
    }

    Ok(input)
}

pub fn user_choice<'a>(msg: &'a str) -> Result<u8, &'a str> {
    println!("{msg}");

    if let Ok(string) = input() {
        return match string.trim().parse() {
            Ok(num) => Ok(num),
            Err(_) => Err("Can't parse string as number"),
        };
    } else {
        return Err("Can't parse user input");
    }
}
