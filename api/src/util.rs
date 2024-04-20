use std::{env, fs, path};

use chrono::{Local, NaiveDateTime};
use url::Url;

use crate::error::Error;

pub fn create_file(p: String) -> Result<bool, Error> {
    let path = path::Path::new(&p);
    if path.exists() {
        return Ok(false);
    }
    let parent = path.parent().ok_or("Invalid pathname")?;
    if !parent.exists() {
        fs::create_dir_all(parent).map_err(|_| "Failed to create parent directory")?;
    }
    fs::File::create(path).map_err(|_| "Failed to create file")?;
    Ok(true)
}

pub fn now() -> String {
    let created_date: NaiveDateTime = Local::now().naive_local();
    created_date.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn validate_url(url: String) -> Result<(), Error> {
    Url::parse(url.as_ref()).map_err(|e| format!("Invalid URL: {}", e))?;
    Ok(())
}

pub fn get_home_env_var() -> Result<String, Error> {
    env::var("HOME").map_err(|_| "HOME environment variable not set.".to_owned())
}
