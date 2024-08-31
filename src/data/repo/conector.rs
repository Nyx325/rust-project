use dotenv::dotenv;
use dotenv::Error as DotenvError;
use rusqlite::Connection;
use rusqlite::Error as RusqliteError;
use std::env::VarError;
use std::fs;
use std::io::Error as IoError;
use std::path::Path;

#[derive(Debug)]
pub enum Error<'a> {
    RusqliteError {
        source: RusqliteError,
        file: &'a str,
        line: u32,
    },
    DotenvError {
        source: DotenvError,
        file: &'a str,
        line: u32,
    },
    DotenvVarError {
        source: VarError,
        file: &'a str,
        line: u32,
    },
    IoError {
        source: IoError,
        file: &'a str,
        line: u32,
    },
    MissingEnvVarError {
        key: String,
        file: &'a str,
        line: u32,
    },
    SqlExecutionError {
        source: RusqliteError,
        file: &'a str,
        line: u32,
    },
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RusqliteError { source, file, line } => {
                write!(f, "RusqliteError: {} (on {}: {})", source, file, line)
            }
            Self::DotenvError { source, file, line } => {
                write!(f, "DotenvError: {} (on {}: {})", source, file, line)
            }
            Self::IoError { source, file, line } => {
                write!(f, "IoError: {} (on {}: {})", source, file, line)
            }
            Self::MissingEnvVarError { key, file, line } => {
                write!(
                    f,
                    "MissingEnvVarError: Missing var {} (on {}: {})",
                    key, file, line
                )
            }
            Self::DotenvVarError { source, file, line } => {
                write!(f, "DotenvVarError: {} (on {}: {})", source, file, line)
            }
            Self::SqlExecutionError { source, file, line } => {
                write!(f, "SqlExecutionError: {} (on {}: {})", source, file, line)
            }
        }
    }
}

impl<'a> std::error::Error for Error<'a> {}

fn get_env_var<'a>(key: &'a str, file: &'a str, line: u32) -> Result<String, Error<'a>> {
    match dotenv::var(key) {
        Ok(key) => Ok(key),
        Err(e) => match e {
            DotenvError::EnvVar(var_err) => match var_err {
                VarError::NotPresent => {
                    let key = key.to_string();
                    let e = Error::MissingEnvVarError { key, file, line };
                    return Err(e);
                }
                _ => {
                    let e = Error::DotenvVarError {
                        source: var_err,
                        line,
                        file,
                    };
                    return Err(e);
                }
            },
            _ => {
                let e = Error::DotenvError {
                    source: e,
                    file: file!(),
                    line: line!(),
                };
                return Err(e);
            }
        },
    }
}

pub struct Connector;

#[allow(unused)]
impl Connector {
    pub fn get_connection<'a>() -> Result<Connection, Error<'a>> {
        dotenv().ok(); // Load environment variables from the .env file
        let database_url = get_env_var("DATABASE_URL", file!(), line!())?;
        let conn = Connection::open(database_url).map_err(|e| Error::RusqliteError {
            source: e,
            file: file!(),
            line: line!(),
        })?;
        Ok(conn)
    }

    pub fn db_exists<'a>() -> Result<bool, Error<'a>> {
        dotenv().ok(); // Load environment variables from the .env file
        let database_url = get_env_var("DATABASE_URL", file!(), line!())?;
        let exists = Path::new(&database_url).exists();
        Ok(exists)
    }

    pub fn db_init<'a>() -> Result<(), Error<'a>> {
        // Check if the database exists
        let db_exists = Self::db_exists()?;
        if !db_exists {
            let database_url = get_env_var("DATABASE_URL", file!(), line!())?;
            println!("The database does not exist, proceeding to create it...");

            // Create the database
            let script_path = get_env_var("DATABASE_INIT_SCRIPT", file!(), line!())?;
            let conn = Self::get_connection()?;
            println!("Initialization script located at: {}", script_path);

            // Read the SQL script
            let sql = fs::read_to_string(&script_path).map_err(|e| Error::IoError {
                source: e,
                file: file!(),
                line: line!(),
            })?;

            // Temporarily create the database file
            let result = conn.execute_batch(&sql);
            // Delete the database file if an error occurs
            if let Err(e) = result {
                if Path::new(&database_url).exists() {
                    fs::remove_file(&database_url).map_err(|e| Error::IoError {
                        source: e,
                        file: file!(),
                        line: line!(),
                    })?;
                }
                return Err(Error::SqlExecutionError {
                    source: e,
                    file: file!(),
                    line: line!(),
                });
            }

            // If the result was Ok, the database file was created successfully
            println!("The database was created successfully.");
        } else {
            println!("The database already exists.");
        }
        Ok(())
    }
}
