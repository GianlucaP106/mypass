mod entries;
mod entrypoint;
mod master;
mod util;
mod view;

pub async fn run() {
    entrypoint::run().await
}
