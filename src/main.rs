// use cli::run;

mod api;
mod cli;
mod database;

#[tokio::main]
async fn main() {
    cli::run().await
}
