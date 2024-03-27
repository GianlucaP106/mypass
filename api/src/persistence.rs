use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::env;

use crate::util;

pub async fn connect() -> Result<DatabaseConnection, String> {
    let home_dir: String = env::var("HOME").map_err(|_| "HOME env var not set.")?;
    let path_to_db = format!("{}/{}", home_dir, String::from(".mypass/db.sqlite"));
    let db_url = format!("sqlite://{}", path_to_db);
    let is_new = util::create_file(path_to_db.to_owned())?;
    let conn = Database::connect(db_url)
        .await
        .map_err(|_| "Failed to connect to database")?;
    if is_new {
        Migrator::up(&conn, None)
            .await
            .map_err(|_| "Failed to push migration to database")?;
    }
    Ok(conn)
}
