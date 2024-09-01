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
    pub fn new(page_size: u64) -> Self {
        Self {
            manager: ClientManager::new(page_size),
        }
    }

    fn add_client(&mut self) {
        Self::clear_linux_console();
        let mut client = Client::default();
        println!("Add a client");
        client.client_name = Self::capture_string("Type the client name");
        client.client_active = true;
        self.manager.add(&client).map_err(|e| println!("{}", e));
    }

    fn search_with_err_map(
        &mut self,
        page: u64,
        criteria: &SearchCriteria,
    ) -> Result<(Vec<Client>, u64), ()> {
        let search = match self.manager.search_by(criteria, page) {
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

        Ok((result, search.total_pages))
    }

    fn get_clients_from_criteria(&mut self, criteria: &SearchCriteria, page: u64) -> Option<u64> {
        Self::clear_linux_console();
        let (clients, total_pages) = match self.search_with_err_map(page, criteria) {
            Ok(result) => result,
            Err(_) => return None,
        };

        let mut client_number = (page - 1) * self.manager.page_size() + 1;
        for client in clients {
            println!(
                "{}) ID: {}, Name: {}, Active: {}",
                client_number,
                client.id_client.unwrap(),
                client.client_name,
                client.client_active
            );
            client_number += 1;
        }
        println!("page {} of {}", page, total_pages);
        return Some(total_pages);
    }

    fn get_criteria() -> SearchCriteria {
        let mut curr_criteria = SearchCriteria::default();
        loop {
            println!(
                "Current criteria:\nID: {}\nActive: {}\nName: {}",
                curr_criteria
                    .id_client
                    .map_or("None".to_string(), |value| value.to_string()),
                curr_criteria
                    .client_active
                    .map_or("None".to_string(), |value| value.to_string()),
                curr_criteria
                    .client_name
                    .clone()
                    .map_or("None".to_string(), |value| value)
            );

            let mut options = String::new();
            options.push_str("1) Set id criteria\n");
            options.push_str("2) Set active criteria\n");
            options.push_str("3) Set name criteria\n");
            options.push_str("4) Continue");

            let opc = Self::capture_atributte::<u8>(&options, "u8");
            match opc {
                1 => {
                    curr_criteria.id_client = Self::capture_option_attribute("Add criteria?", "u32")
                }
                2 => {
                    curr_criteria.client_active =
                        Self::capture_option_attribute("Add criteria?", "bool")
                }
                3 => {
                    curr_criteria.client_name =
                        Self::capture_option_attribute("Add criteria?", "String")
                }
                4 => return curr_criteria,
                _ => println!("Invalid option"),
            }
        }
    }

    fn search_client(&mut self) {
        let criteria = Self::get_criteria();
        let mut page = 1;
        loop {
            let total_pages = self.get_clients_from_criteria(&criteria, page);

            if total_pages.is_none() {
                println!("No hay resultados");
            } else {
                let total_pages = total_pages.unwrap();

                let mut title = String::new();
                title.push_str("1) prev page\n");
                title.push_str("2) next page\n");
                title.push_str("3) exit");

                let opc: u8 = Self::capture_atributte(&title, "u8");
                match opc {
                    1 => {
                        if page > 1 {
                            page -= 1;
                        }
                    }
                    2 => {
                        if page < total_pages {
                            page += 1;
                        }
                    }
                    3 => break,
                    _ => println!("Invalid option"),
                }
            }
        }
    }

    fn list_clients(&mut self) {
        let criteria = SearchCriteria::default();
        let mut page = 1;
        loop {
            let total_pages = self.get_clients_from_criteria(&criteria, page);

            if total_pages.is_none() {
                println!("No hay resultados");
            } else {
                let total_pages = total_pages.unwrap();
                let opc: u8 = Self::capture_atributte("1) prev page\n2) next page\n3) exit", "u8");
                match opc {
                    1 => {
                        if page > 1 {
                            page -= 1;
                        }
                    }
                    2 => {
                        if page < total_pages {
                            page += 1;
                        }
                    }
                    3 => break,
                    _ => println!("Invalid option"),
                }
            }
        }
    }
}

impl ConsoleView for ClientConsoleView {
    fn menu(&mut self) {
        loop {
            Self::clear_linux_console();
            println!("Client Management");
            println!("1) List clients");
            println!("2) Add client");
            println!("3) Modify client");
            println!("4) Logic client deletion");
            println!("5) Complete client deletion");
            println!("6) Search client");
            println!("7) Exit");
            match Self::capture_atributte::<u8>("Select an option: ", "u8") {
                1 => self.list_clients(),
                2 => self.add_client(),
                6 => self.search_client(),
                7 => return,
                _ => println!("Invalid option"),
            }
        }
    }
}
