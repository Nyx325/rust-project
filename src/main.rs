use data::repo::conector::Connector;

mod data;
mod logic;
mod view;
mod data_management;

fn main() {
    Connector::db_init().unwrap();
}
