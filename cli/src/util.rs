use std::{fmt::Display, io::Write};

use api::error::Error;

pub fn get_master_password() -> Result<String, ()> {
    get_password_with_prompt_print("Master Password: ")
}

pub fn get_password_with_prompt_print(prompt: &str) -> Result<String, ()> {
    get_password_with_prompt(prompt).map_err(|_| println!("Password is required"))
}

pub fn get_password_with_prompt(prompt: &str) -> Result<String, Error> {
    rpassword::prompt_password(prompt)
        .map_err(|_| ())
        .and_then(|pass| {
            if pass.trim().is_empty() {
                Err(())
            } else {
                Ok(pass)
            }
        })
        .map_err(|_| "Failed to prompt password".to_owned())
}

pub fn get_input(prompt: &str) -> Result<String, Error> {
    print!("{}", prompt);
    let err = "Failed to get input from console";
    std::io::stdout().flush().map_err(|_| err)?;
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).map_err(|_| err)?;
    Ok(line.trim().to_owned())
}

pub fn get_input_required(prompt: &str) -> Option<String> {
    get_input(prompt).ok().and_then(|input| {
        if input.trim().is_empty() {
            None
        } else {
            Some(input)
        }
    })
}

pub fn unwrap_or_input(item: Option<String>, prompt: &str) -> Option<String> {
    item.or_else(|| get_input_required(prompt))
}

pub fn unwrap_or_input_number(
    item: Option<usize>,
    prompt: &str,
    err_msg: &str,
) -> Result<usize, ()> {
    item.or_else(|| {
        get_input(prompt)
            .print_err()
            .ok()
            .and_then(|val| val.parse::<usize>().ok())
            .and_then(|val| if val > 0 { Some(val) } else { None })
    })
    .ok_or_else(|| println!("{}", err_msg))
}

pub fn copy_to_clipboard(item: String) -> Result<(), Error> {
    cli_clipboard::set_contents(item).map_err(|_| "Failed to copy to clipboard".to_owned())
}

pub trait PrintError<T, E> {
    fn print_err(self) -> Result<T, ()>;
}

impl<T, E: Display> PrintError<T, E> for Result<T, E> {
    fn print_err(self) -> Result<T, ()> {
        self.map_err(|e| println!("{}", e))
    }
}
