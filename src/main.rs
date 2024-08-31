use data::repo::conector::Connector;
use view::client_console_view::ClientConsoleView;
use view::console_view::ConsoleView;

mod data;
mod data_management;
mod logic;
mod view;

fn main() {
    Connector::db_init().unwrap();
    let mut client_view = ClientConsoleView::new(50);
    client_view.menu();
}
