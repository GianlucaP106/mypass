use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::env;

use crate::util;

pub async fn connect() -> DatabaseConnection {
    // TODO: Wrap in Result
    let home_dir: String = env::var("HOME").expect("HOME env var not set.");
    let path_to_db = format!("{}/{}", home_dir, String::from(".mypass/db.sqlite"));
    let db_url = format!("sqlite://{}", path_to_db);
    let is_new = util::create_file(path_to_db.to_owned()).expect("");
    let conn = Database::connect(db_url)
        .await
        .expect("Failed to connect to store");
    if is_new {
        Migrator::up(&conn, None).await.unwrap();
    }
    conn
}
