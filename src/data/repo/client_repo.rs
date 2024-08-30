use std::any::Any;
use std::collections::HashMap;

use super::conector::Connector;
use super::conector::Error as ConnectorError;
use crate::{data::model::client::Client, data_management::Repository};
use rusqlite::{params, Error as RusqliteError};

#[derive(Debug)]
pub enum Error<'a> {
    ConnectorError {
        source: ConnectorError<'a>,
        file: &'a str,
        line: u32,
    },
    RusqliteError {
        source: RusqliteError,
        file: &'a str,
        line: u32,
    },
    ItemShouldExists {
        source: Client,
        file: &'a str,
        line: u32,
    },
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConnectorError { source, file, line } => {
                write!(f, "ConnectorError: {} (on {}: {})", source, file, line)
            }
            Self::RusqliteError { source, file, line } => {
                write!(f, "RusqliteError: {} (on {}: {})", source, file, line)
            }
            Self::ItemShouldExists { source, file, line } => {
                write!(
                    f,
                    "ItemShouldExists: el item {} no se encontró (on {}: {})",
                    source, file, line
                )
            }
        }
    }
}

impl<'a> std::error::Error for Error<'a> {}

impl<'a> From<ConnectorError<'a>> for Error<'a> {
    fn from(source: ConnectorError<'a>) -> Self {
        Self::ConnectorError {
            source,
            file: file!(),
            line: line!(),
        }
    }
}

impl<'a> From<RusqliteError> for Error<'a> {
    fn from(source: RusqliteError) -> Self {
        Self::RusqliteError {
            source,
            file: file!(),
            line: line!(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct ClientRepo {
    page_size: u32,
}

impl ClientRepo {
    pub fn new(page_size: u32) -> Self {
        Self { page_size }
    }
}

impl<'a> Repository<Client, Error<'a>> for ClientRepo {
    fn add(&mut self, item: &Client) -> Result<(), Error<'a>> {
        let conn = Connector::get_connection()?;
        let sql = "INSERT INTO Client (client_active, client_name) VALUES (?,?)";

        conn.prepare(sql)?
            .execute(params![item.client_active, item.client_name])?;

        Ok(())
    }

    fn drop(&mut self, item: &mut Client) -> Result<(), Error<'a>> {
        let conn = Connector::get_connection()?;
        let query = "UPDATE Client SET client_active = 0 WHERE id_client = ?";
        conn.prepare(query)?.execute(params![item.id_client])?;

        item.client_active = false;
        Ok(())
    }

    fn delete(&mut self, item: &Client) -> Result<(), Error<'a>> {
        let conn = Connector::get_connection()?;
        let query = "DELETE FROM Client WHERE id_client = ?";
        conn.prepare(query)?.execute(params![item.id_client])?;

        Ok(())
    }

    fn modify(&mut self, item: &Client) -> Result<(), Error<'a>> {
        let id = item.id_client.ok_or_else(|| Error::ItemShouldExists {
            source: item.clone(),
            file: file!(),
            line: line!(),
        })?;

        let original = self
            .search_by_id(id)?
            .ok_or_else(|| Error::ItemShouldExists {
                source: item.clone(),
                file: file!(),
                line: line!(),
            })?;

        if original == *item {
            return Ok(());
        }

        // Construir la consulta SQL
        let mut query = "UPDATE Client SET".to_string();
        let mut params: Vec<&dyn rusqlite::types::ToSql> = Vec::new();

        // Construir la lista de columnas a actualizar y los parámetros
        if original.client_name != item.client_name {
            query.push_str(" client_name = ?,");
            params.push(&item.client_name);
        }

        if original.client_active != item.client_active {
            query.push_str(" client_active = ?,");
            params.push(&item.client_active);
        }

        // Eliminar la última coma y añadir la cláusula WHERE
        query.pop(); // Elimina la última coma
        query.push_str(" WHERE id_client = ?");

        // Agregar el ID al final de los parámetros
        params.push(&id);

        // Ejecutar la consulta SQL
        let conn = Connector::get_connection()?;
        conn.execute(&query, params.as_slice())?;

        Ok(())
    }

    fn search_by_id(&self, id: u32) -> Result<Option<Client>, Error<'a>> {
        let conn = Connector::get_connection()?;

        let sql = "SELECT id_client, client_active, client_name FROM Client WHERE id_client = ?";

        let mut stmt = conn.prepare(sql)?;
        let mut rows = stmt.query(params![id])?;

        if let Some(row) = rows.next()? {
            let client = Client {
                id_client: row.get(0)?,
                client_active: row.get(1)?,
                client_name: row.get(2)?,
            };
            Ok(Some(client))
        } else {
            Ok(None)
        }
    }

    fn search_by_attributes(
        &mut self,
        page: usize,
        hash_map: HashMap<String, Box<dyn Any>>,
    ) -> Result<String, Error<'a>> {
        todo!()
    }
}
