use crate::data::model::client::Client;
use crate::data_management::{Manager, Repository};
use crate::data::repo::client_repo::{ClientRepo, Error as RepoError};

#[allow(unused)]
#[derive(Debug)]
pub enum Error<'a>{
    RepoError(RepoError<'a>),
    InvalidField {
        source: String,
        file: &'a str,
        line: u32,
    }
}

#[allow(unused)]
impl <'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => todo!(),
        }
    }
}

impl <'a>From<RepoError<'a>> for Error<'a> {
    fn from(value: RepoError<'a>) -> Self {
        Self::RepoError(value)
    }
}

pub struct ClientManager {
    repository: ClientRepo,
    last_search: Option<String>,
    last_selected: Option<Client>,
}

#[allow(unused)]
impl <'a>ClientManager {
    pub fn new() -> Self {
        Self { 
            repository: ClientRepo::new(25),
            last_search: None,
            last_selected: None,
        }
    }
}

#[allow(unused)]
impl <'a> Manager<Client, Error<'a>> for ClientManager {
    fn valid_item(&self, item: &Client) -> Result<(), Error<'a>> {
        let mut errors = Vec::new();
    
        if item.id_client.is_some() {
            errors.push("se intenta agregar un elemento existente".to_string());
        }
    
        if item.client_name.is_empty() {
            errors.push("el nombre del cliente no puede estar vacío".to_string());
        }

        if !errors.is_empty() {
            let mut error_message = errors.join(", ");
            if let Some(first_char) = error_message.get_mut(0..1) {
                first_char.make_ascii_uppercase();
            }

            return Err(
                Error::InvalidField {
                    source: error_message,
                    file: file!(),
                    line: line!(),
                }
            );
        }
    
        Ok(())
    }

    fn last_search(&self) -> Option<String> {
        self.last_search.clone()
    }

    fn last_selected(&self) -> Option<Client> {
        self.last_selected.clone()
    }

    fn set_last_search(&mut self, search: String) {
        self.last_search = Some(search)
    }

    fn set_last_selected(&mut self, item: Client) {
        self.last_selected = Some(item)
    }
}

impl <'a> Repository<Client, Error<'a>> for ClientManager { 
    fn add(&mut self, item: &Client) -> Result<(), Error<'a>> {
        self.valid_item(item)?;
        self.repository.add(item)?;
        Ok(())
    }

    fn drop(&mut self, item: &Client) -> Result<(), Error<'a>> {
        self.repository.drop(item)?;
        Ok(())
    }

    fn delete(&mut self, item: &Client) -> Result<(), Error<'a>> {
        self.repository.delete(item)?;
        Ok(())
    }

    fn modify(&mut self, item: &Client) -> Result<(), Error<'a>> {
        self.repository.modify(item)?;
        Ok(())
    }

    fn search_by_attributes(&mut self, page: usize, json_hashmap: String) 
            -> Result<String,Error<'a>> {
        let search = self.repository.search_by_attributes(page, json_hashmap)?;
        self.set_last_search(search.clone());
        Ok(search)
    }
}
