use std::collections::LinkedList;

use crate::{
    data::{model::client::Client, repo::client_repo::SearchCriteria},
    data_management::{Finder, Repository},
    logic::client_manager::ClientManager,
};

use super::console_view::ConsoleView;

#[allow(unused)]
pub struct ClientConsoleView {
    manager: ClientManager<SearchCriteria>,
}

#[allow(unused)]
impl ClientConsoleView {
    pub fn new(page_size: u128) -> Self {
        Self {
            manager: ClientManager::new(page_size),
        }
    }

    fn add_client(&mut self) {
        let mut client = Client::default();
        println!("Add a client");
        client.client_name = Self::capture_string("Type the client name");
        client.client_active = true;
        self.manager.add(&client).map_err(|e| println!("{}", e));
    }

    fn search_with_err_map(
        &mut self,
        page: u128,
        criteria: &SearchCriteria,
    ) -> Result<Vec<Client>, ()> {
        let search = match self.manager.search_by(&SearchCriteria::default(), page) {
            Ok(search) => (search),
            Err(e) => {
                println!("Error {}", e);
                return Err(());
            }
        };

        let result: Vec<Client> = match serde_json::from_str(&search.result) {
            Ok(result) => result,
            Err(e) => {
                println!("Error {}", e);
                return Err(());
            }
        };

        Ok(result)
    }

    fn list_clients(&mut self) {
        let mut page = 1;
        loop {
            let result = match self.search_with_err_map(page, &SearchCriteria::default()) {
                Ok(result) => result,
                Err(_) => return,
            };

            for client in result {
                println!("{}", client)
            }
            println!("page {}", page);
            loop {
                let opc: u8 = Self::capture_atributte("1) prev page\n2) next page\n3) exit", "u8");
                match opc {
                    1 => {
                        if page > 1 {
                            page -= 1;
                        }
                        break;
                    }
                    2 => {
                        page += 1;
                        break;
                    }
                    _ => println!("Invalid option"),
                }
            }
        }
    }
}

impl ConsoleView for ClientConsoleView {
    fn menu(&mut self) {
        loop {
            println!("Client Management");
            println!("1) List clients");
            println!("2) Add client");
            println!("3) Modify client");
            println!("4) Logic client deletion");
            println!("5) Complete client deletion");
            println!("6) Search client deletion");
            println!("7) Exit");
            match Self::capture_atributte::<u8>("Select an option: ", "u8") {
                1 => self.list_clients(),
                2 => self.add_client(),
                7 => return,
                _ => println!("Invalid option"),
            }
        }
    }
}
