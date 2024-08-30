use crate::{data::model::client::Client, data_management::Repository};
use super::conector::Connector;
use super::conector::Error as ConnectorError;
use rusqlite::{Error as RusqliteError, params};

#[derive(Debug)]
pub enum Error<'a>{
    ConnectorError{
        source: ConnectorError<'a>,
        file: &'a str,
        line: u32,
    },
    RusqliteError{
        source: RusqliteError,
        file: &'a str,
        line: u32,
    }
}

impl <'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConnectorError { source, file, line } => {
                write!(f, "ConnectorError: {} (on {}: {})", source, file, line)
            },
            Self::RusqliteError { source, file, line } => {
                write!(f, "RusqliteError: {} (on {}: {})", source, file, line)
            }
        }
    }
}

impl <'a> std::error::Error for Error<'a> {}

impl <'a> From<ConnectorError<'a>> for Error<'a> {
    fn from(source: ConnectorError<'a>) -> Self {
        Self::ConnectorError { 
            source, 
            file: file!(), 
            line: line!(),
        }
    }
}

impl <'a> From<RusqliteError> for Error<'a> {
    fn from(source: RusqliteError) -> Self {
        Self::RusqliteError { 
            source, 
            file: file!(), 
            line: line!(),
        }
    }
}


#[allow(dead_code)]
#[derive(Debug,Clone,PartialEq)]
pub struct ClientRepo {
    page_size: u32,
}

impl ClientRepo {
    pub fn new(page_size: u32) -> Self {
        Self { page_size }
    }
}

impl <'a> Repository<Client, Error<'a>> for ClientRepo {
    fn add(&mut self, item: &Client) -> Result<(), Error<'a>> {
        let conn = Connector::get_conection()?;
        let sql = "INSERT INTO Client (client_active, client_name) VALUES (?,?)";

        conn
            .prepare(sql)?
            .execute(params![item.client_active, item.client_name])?;

        Ok(())
    }

    fn drop(&mut self, item: &Client) -> Result<(), Error<'a>> {
        todo!()
    }

    fn delete(&mut self, item: &Client) -> Result<(), Error<'a>> {
        todo!()
    }

    fn modify(&mut self, item: &Client) -> Result<(), Error<'a>> {
        todo!()
    }

    fn search_by_attributes(&mut self, page: usize, json_hashmap: String) 
            -> Result<String,Error<'a>> {
        todo!()
    }
}
