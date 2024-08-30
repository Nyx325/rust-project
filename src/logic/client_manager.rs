use crate::data::model::client::Client;
use crate::data::repo::client_repo::{ClientRepo, Error as RepoError, SearchCriteria};
use crate::data_management::{LastSearch, Manager, Repository};

#[allow(unused)]
#[derive(Debug)]
pub enum Error<'a> {
    RepoError(RepoError<'a>),
    InvalidField {
        source: String,
        file: &'a str,
        line: u32,
    },
}

#[allow(unused)]
impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => todo!(),
        }
    }
}

impl<'a> From<RepoError<'a>> for Error<'a> {
    fn from(value: RepoError<'a>) -> Self {
        Self::RepoError(value)
    }
}

pub struct ClientManager<SearchCriteria> {
    repository: ClientRepo,
    last_search: Option<LastSearch<SearchCriteria>>,
    last_selected: Option<Client>,
}

#[allow(unused)]
impl<'a> ClientManager<SearchCriteria> {
    pub fn new() -> Self {
        Self {
            repository: ClientRepo::new(25),
            last_search: None,
            last_selected: None,
        }
    }

    fn update_last_search(&mut self) -> Result<(), Error<'a>> {
        todo!()
    }
}

#[allow(unused)]
impl<'a> Manager<Client, SearchCriteria, Error<'a>> for ClientManager<SearchCriteria> {
    fn valid_item(&self, item: &Client) -> Result<(), Error<'a>> {
        let mut errors = Vec::new();

        if item.id_client.is_some() {
            errors.push("se intenta agregar un elemento existente".to_string());
        }

        if !item.client_active {
            errors.push("el campo client_active debe ser true".to_string());
        }

        if item.client_name.is_empty() {
            errors.push("el nombre del cliente no puede estar vacío".to_string());
        }

        if !errors.is_empty() {
            let mut error_message = errors.join(", ");
            if let Some(first_char) = error_message.get_mut(0..1) {
                first_char.make_ascii_uppercase();
            }

            return Err(Error::InvalidField {
                source: error_message,
                file: file!(),
                line: line!(),
            });
        }

        Ok(())
    }

    fn last_search(&self) -> Option<LastSearch<SearchCriteria>> {
        todo!()
    }

    fn last_selected(&self) -> Option<Client> {
        self.last_selected.clone()
    }

    fn set_last_search(&mut self, search: LastSearch<SearchCriteria>) {
        self.last_search = Some(search)
    }

    fn set_last_selected(&mut self, item: Client) {
        self.last_selected = Some(item)
    }
}

impl<'a> Repository<Client, Error<'a>> for ClientManager<SearchCriteria> {
    fn add(&mut self, item: &Client) -> Result<(), Error<'a>> {
        self.valid_item(item)?;
        self.repository.add(item)?;
        self.update_last_search()?;
        Ok(())
    }

    fn drop(&mut self, item: &mut Client) -> Result<(), Error<'a>> {
        self.repository.drop(item)?;
        self.update_last_search()?;
        Ok(())
    }

    fn delete(&mut self, item: &Client) -> Result<(), Error<'a>> {
        self.repository.delete(item)?;
        self.update_last_search()?;
        Ok(())
    }

    fn modify(&mut self, item: &Client) -> Result<(), Error<'a>> {
        self.repository.modify(item)?;
        self.update_last_search()?;
        Ok(())
    }
}
