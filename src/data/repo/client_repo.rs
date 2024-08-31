use std::collections::LinkedList;

use super::conector::Connector;
use super::conector::Error as ConnectorError;
use crate::data_management::Finder;
use crate::data_management::LastSearch;
use crate::{data::model::client::Client, data_management::Repository};
use rusqlite::Row;
use rusqlite::{params, Error as RusqliteError};
use serde_json::Error as SerdeJsonError;

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
    SqlExecutionError {
        source: RusqliteError,
        query: String,
        file: &'a str,
        line: u32,
    },
    RowShouldReturned {
        source: &'a str,
        file: &'a str,
        line: u32,
    },
    ItemShouldExists {
        source: Client,
        file: &'a str,
        line: u32,
    },
    FromRowError {
        source: &'a str,
        file: &'a str,
        line: u32,
    },
    SerdeError {
        source: SerdeJsonError,
        file: &'a str,
        line: u32,
    },
    PageCountError {
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
            Self::SqlExecutionError {
                source,
                query,
                file,
                line,
            } => {
                write!(
                    f,
                    "RusqliteError: {} Query: {} (on {}: {})",
                    source, query, file, line
                )
            }
            Self::ItemShouldExists { source, file, line } => {
                write!(
                    f,
                    "ItemShouldExists: el item {} no se encontró (on {}: {})",
                    source, file, line
                )
            }
            Self::FromRowError { source, file, line } => {
                write!(
                    f,
                    "FromRowError: no se pudo obtener un dato de la fila {} (on {}: {})",
                    source, file, line
                )
            }
            Self::SerdeError { source, file, line } => {
                write!(f, "SerdeError: {} (on {}: {})", source, file, line)
            }
            Self::RowShouldReturned { source, file, line } => {
                write!(f, "RowShouldReturned: {} (on {}: {})", source, file, line)
            }
            Self::PageCountError { file, line } => {
                write!(
                    f,
                    "PageCountError: Page size must be non-zero (on {}: {})",
                    file, line
                )
            }
        }
    }
}

impl<'a> std::error::Error for Error<'a> {}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct ClientRepo {
    page_size: u64,
}

impl ClientRepo {
    pub fn new(page_size: u64) -> Self {
        Self { page_size }
    }
}

impl<'a> Repository<Client, Error<'a>> for ClientRepo {
    fn add(&mut self, item: &Client) -> Result<(), Error<'a>> {
        let conn = Connector::get_connection().map_err(|e| Error::ConnectorError {
            source: e,
            file: file!(),
            line: line!(),
        })?;

        let sql = "INSERT INTO Client (client_active, client_name) VALUES (?,?)";

        conn.prepare(sql)
            .map_err(|e| Error::SqlExecutionError {
                source: e.into(),
                query: sql.to_string(),
                file: file!(),
                line: line!(),
            })?
            .execute(params![item.client_active, item.client_name])
            .map_err(|e| Error::RusqliteError {
                source: e,
                file: file!(),
                line: line!(),
            })?;

        Ok(())
    }

    fn drop(&mut self, item: &mut Client) -> Result<(), Error<'a>> {
        let conn = Connector::get_connection().map_err(|e| Error::ConnectorError {
            source: e,
            file: file!(),
            line: line!(),
        })?;

        let query = "UPDATE Client SET client_active = 0 WHERE id_client = ?";
        conn.prepare(query)
            .map_err(|e| Error::SqlExecutionError {
                source: e,
                query: query.to_string(),
                file: file!(),
                line: line!(),
            })?
            .execute(params![item.id_client])
            .map_err(|e| Error::RusqliteError {
                source: e,
                file: file!(),
                line: line!(),
            })?;

        item.client_active = false;
        Ok(())
    }

    fn delete(&mut self, item: &Client) -> Result<(), Error<'a>> {
        let conn = Connector::get_connection().map_err(|e| Error::ConnectorError {
            source: e,
            file: file!(),
            line: line!(),
        })?;

        let query = "DELETE FROM Client WHERE id_client = ?";
        conn.prepare(query)
            .map_err(|e| Error::SqlExecutionError {
                source: e,
                query: query.to_string(),
                file: file!(),
                line: line!(),
            })?
            .execute(params![item.id_client])
            .map_err(|e| Error::RusqliteError {
                source: e,
                file: file!(),
                line: line!(),
            })?;

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
        let conn = Connector::get_connection().map_err(|e| Error::ConnectorError {
            source: e,
            file: file!(),
            line: line!(),
        })?;
        conn.execute(&query, params.as_slice())
            .map_err(|e| Error::RusqliteError {
                source: e,
                file: file!(),
                line: line!(),
            })?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SearchCriteria {
    pub id_client: Option<u32>,
    pub client_active: Option<bool>,
    pub client_name: Option<String>,
}

#[allow(unused)]
impl<'a> Finder<Client, SearchCriteria, Error<'a>> for ClientRepo {
    fn from_row(row: &Row) -> Result<Client, Error<'a>> {
        let client = Client {
            id_client: row.get(0).map_err(|_| Error::FromRowError {
                source: "id_client",
                file: file!(),
                line: line!(),
            })?,
            client_active: row.get(1).map_err(|_| Error::FromRowError {
                source: "client_active",
                file: file!(),
                line: line!(),
            })?,
            client_name: row.get(2).map_err(|_| Error::FromRowError {
                source: "client_name",
                file: file!(),
                line: line!(),
            })?,
        };
        Ok(client)
    }

    fn page_size(&self) -> u64 {
        self.page_size
    }

    fn search_by_id(&self, id: u32) -> Result<Option<Client>, Error<'a>> {
        let conn = Connector::get_connection().map_err(|e| Error::ConnectorError {
            source: e,
            file: file!(),
            line: line!(),
        })?;

        let sql = "SELECT id_client, client_active, client_name FROM Client WHERE id_client = ?";

        let mut stmt = conn.prepare(sql).map_err(|e| Error::SqlExecutionError {
            source: e,
            query: sql.to_string(),
            file: file!(),
            line: line!(),
        })?;
        let mut rows = stmt.query(params![id]).map_err(|e| Error::RusqliteError {
            source: e,
            file: file!(),
            line: line!(),
        })?;

        if let Some(row) = rows.next().map_err(|e| Error::RusqliteError {
            source: e,
            file: file!(),
            line: line!(),
        })? {
            Ok(Some(Self::from_row(row)?))
        } else {
            Ok(None)
        }
    }

    fn search_by(
        &mut self,
        criteria: &SearchCriteria,
        page_number: u64,
    ) -> Result<LastSearch<SearchCriteria>, Error<'a>> {
        let mut where_str = String::new();

        if let Some(id_client) = criteria.id_client {
            let str = format!(" id_client LIKE %{}% ", id_client);
            where_str.push_str(&str);
        }

        if let Some(client_active) = criteria.client_active {
            let str = format!(" client_active LIKE %{}% ", client_active);
            where_str.push_str(&str);
        }

        if let Some(client_name) = &criteria.client_name {
            let str = format!(" client_active LIKE %{}% ", client_name);
            where_str.push_str(&str);
        }

        if where_str.len() != 0 {
            where_str = format!("WHERE {}", where_str);
        }

        let page_system = format!(
            "ORDER BY client_name LIMIT {} OFFSET ( {} - 1 ) * {}",
            self.page_size(),
            page_number,
            self.page_size()
        );

        let count_query = format!("SELECT COUNT(*) FROM Client {}", &where_str);
        let query = format!(
            "SELECT id_client, client_active, client_name FROM Client {} {}",
            &where_str, &page_system
        );

        let conn = Connector::get_connection().map_err(|e| Error::ConnectorError {
            source: e,
            file: file!(),
            line: line!(),
        })?;

        let mut stmt = conn
            .prepare(&count_query)
            .map_err(|e| Error::SqlExecutionError {
                source: e,
                query: count_query.to_string(),
                file: file!(),
                line: line!(),
            })?;

        let mut count_row = stmt.query(params![]).map_err(|e| Error::RusqliteError {
            source: e,
            file: file!(),
            line: line!(),
        })?;

        let count_row = count_row.next().map_err(|e| Error::RusqliteError {
            source: e,
            file: file!(),
            line: line!(),
        })?;

        let total_registers: u64 = if let Some(row) = count_row {
            row.get(0).map_err(|_| Error::FromRowError {
                source: "id_client",
                file: file!(),
                line: line!(),
            })?
        } else {
            return Err(Error::RowShouldReturned {
                source: "Un SELECT COUNT() deberia devolver al menos una fila",
                file: file!(),
                line: line!(),
            });
        };

        let mut stmt = conn.prepare(&query).map_err(|e| Error::SqlExecutionError {
            source: e,
            query: query.to_string(),
            file: file!(),
            line: line!(),
        })?;

        let mut rows = stmt.query(params![]).map_err(|e| Error::RusqliteError {
            source: e,
            file: file!(),
            line: line!(),
        })?;

        let mut result: LinkedList<Client> = LinkedList::new();

        while let Some(row) = rows.next().map_err(|e| Error::RusqliteError {
            source: e,
            file: file!(),
            line: line!(),
        })? {
            result.push_back(Self::from_row(row)?);
        }

        let result = serde_json::to_string(&result).map_err(|e| Error::SerdeError {
            source: e,
            file: file!(),
            line: line!(),
        })?;

        println!(
            "{}/{} = {}",
            total_registers,
            self.page_size(),
            Self::total_pages(total_registers, self.page_size())
        );

        let search: LastSearch<SearchCriteria> = LastSearch::new(
            page_number,
            Self::total_pages(total_registers, self.page_size()),
            criteria.clone(),
            result,
        );

        Ok(search)
    }
}
