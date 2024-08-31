use std::process::Command;
use std::{io::stdin, str::FromStr};

#[allow(dead_code)]
pub trait ConsoleView {
    fn menu(&mut self);

    fn capture_string(title: &str) -> String {
        loop {
            let mut input = String::new();
            println!("{}", title);

            if let Err(e) = stdin().read_line(&mut input) {
                println!("Error: {}", e);
                continue;
            }

            return input.trim().to_string();
        }
    }

    fn capture_atributte<T>(title: &str, type_expect: &str) -> T
    where
        T: FromStr,
    {
        loop {
            let mut input = String::new();
            println!("{}", title);

            if let Err(e) = stdin().read_line(&mut input) {
                println!("Error: {}", e);
                continue;
            }

            let input = input.trim();
            match input.parse::<T>() {
                Ok(input) => return input,
                Err(_) => {
                    println!("No se pudo convertir \"{}\" en \"{}\"", input, type_expect);
                    continue;
                }
            };
        }
    }

    fn capture_option_attribute<T>(title: &str, type_expected: &str) -> Option<T>
    where
        T: FromStr,
    {
        loop {
            println!("{title}");
            match Self::capture_atributte::<u8>("1) Si\n2) No", "u8") {
                1 => {
                    return Some(Self::capture_atributte::<T>(
                        "Ingresa el dato:",
                        &type_expected,
                    ))
                }
                2 => return None,
                _ => {
                    println!("Opción no válida");
                    continue;
                }
            };
        }
    }

    fn clear_linux_console() {
        /*
        if let Err(e) = Command::new("clear").status() {
            println!("Error al limpiar consola: {}\n\n\n\n\n", e);
        }
        */
    }
}
