use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::{env, fs, path};

pub async fn connect() -> DatabaseConnection {
    // TODO: Wrap in Result
    let home_dir: String = env::var("HOME").expect("HOME env var not set.");
    let path_to_db = format!("{}/{}", home_dir, String::from(".mypass/db.sqlite"));
    let db_url = format!("sqlite://{}", path_to_db);
    let path = path::Path::new(&path_to_db);
    if path.exists() {
        return Database::connect(db_url)
            .await
            .expect("Failed to connect to store");
    }
    let parent = path.parent().unwrap();
    fs::create_dir_all(parent).expect("Failed to create store");
    fs::File::create(&path_to_db).expect("Failed to create store");
    let conn = Database::connect(db_url)
        .await
        .expect("Failed to connect to store");

    Migrator::up(&conn, None).await.unwrap();
    conn
}
