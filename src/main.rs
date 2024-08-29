use data::{model::client::Client, repo::{client_repo::ClientRepo, conector::Connector}};
use data_management::Repository;

mod data;
mod logic;
mod view;
mod data_management;

fn main() {
    Connector::db_init().unwrap();
    let mut repo = ClientRepo::new(50);
    let item = Client::default();
    println!("{}", item);
    repo.add(&item).unwrap();
}
