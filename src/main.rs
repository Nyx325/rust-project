use data::model::client::Client;
use data::repo::conector::Connector;
use data_management::Repository;
use logic::client_manager::ClientManager;

mod data;
mod data_management;
mod logic;
mod view;

fn main() {
    Connector::db_init().unwrap();
    let mut manager = ClientManager::new();
    let mut item = Client::default();
    item.client_name = "Famsa".to_string();
    item.client_active = true;
    println!("{}", item);
    manager.add(&item).unwrap();
}
