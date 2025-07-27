mod generator;
mod options;

use crate::{generator::generate, options::Options};
use std::io::{self};

fn main() {
    let mut config = Options {
        lowercase: false,
        uppercase: false,
        number: false,
        symbols: false,
    };

    let mut input = String::new();

    println!("Qual tamanho da sua senha?");

    io::stdin()
        .read_line(&mut input)
        .expect("Insira uma respota valida");

    let size: i32 = input.trim().parse().expect("Digite um número");

    loop {
        config.lowercase = get_input("Letras Minusculas");
        config.uppercase = get_input("Letras Maiusculas");
        config.number = get_input("Numeros");
        config.symbols = get_input("Simbolos");

        if config.any_enabled() {
            break;
        } else {
            println!("\nPor favor escolha ao menos uma opçao\n")
        }
    }

    let password = generate(config, size);

    println!("\nSua senha: {}", password);
}

fn get_input(option: &str) -> bool {
    loop {
        let mut input = String::new();

        println!("Deseja que sua senha tenha {}? [y/N]", option);

        io::stdin()
            .read_line(&mut input)
            .expect("Insira uma respota valida");

        match input.trim().to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            "" => return false,
            _ => println!("\nPor favor digite um valor valido y/n \n"),
        }
    }
}
