use std::{fs, path};

use chrono::{Local, NaiveDateTime};

pub fn create_file(p: String) -> Result<bool, String> {
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
