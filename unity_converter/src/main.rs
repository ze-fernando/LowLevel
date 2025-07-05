mod converter;

use crate::converter::Converter;
use std::io::{self, Write};


fn main() {
    loop {
        println!("\n==== MENU ====");
        println!("1 - Celsius para Fahrenheit");
        println!("2 - Fahrenheit para Celsius");
        println!("3 - Km para Milhas");
        println!("4 - Milhas para Km");
        println!("0 - Sair");

        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();

        print!("\n");
        match option.trim() {
            "1" => {
                print!("Digite a temperatura em Celsius: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Falha ao ler entrada");

                let celsius: f64 = input.trim().parse().expect("Digite um número válido");

                let fahrenheit: f64 = Converter::celsius_to_fahrenheit(celsius);

                println!("Valor normal: {}", fahrenheit);
                println!("Valor arredondado: {}", (fahrenheit * 100.0).round() / 100.0);
            }
            "2" => {
                print!("Digite a temperatura em Fahrenheit: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Falha ao ler entrada");

                let fahrenheit: f64 = input.trim().parse().expect("Digite um número válido");

                let celsius: f64 = Converter::fahrenheit_to_celsius(fahrenheit);

                println!("Valor normal: {}", celsius);
                println!("Valor arredondado: {}", (celsius * 100.0).round() / 100.0);
            }
            "3" => {
                print!("Digite a distância em Km: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Falha ao ler entrada");

                let km: f64 = input.trim().parse().expect("Digite um número válido");

                let miles: f64 = Converter::km_to_milles(km);

                println!("Valor normal: {}", miles);
                println!("Valor arredondado: {}", (miles * 100.0).round() / 100.0);
            }
            "4" => {
                print!("Digite a distância em Milhas: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Falha ao ler entrada");

                let miles: f64 = input.trim().parse().expect("Digite um número válido");

                let km: f64 = Converter::milles_to_km(miles);

                println!("Valor normal: {}", km);
                println!("Valor arredondado: {}", (km * 100.0).round() / 100.0);
            }
            "0" => break,
            _ => println!("Opção inválida."),
        }
    }

}
