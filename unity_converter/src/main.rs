mod converter;

use std::io::{self, Write};
fn main() {
    loop {
        println!("\n==== MENU ====");
        println!("1 - Celsius to Fahrenheit");
        println!("2 - Fahrenheit to Celsius");
        println!("3 - Km to Miles");
        println!("4 - Miles to Km");
        println!("0 - Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();

        print!("\n");
        match option.trim() {
            "1" => {
                print!("Enter temperature in Celsius: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");

                let celsius: f64 = input.trim().parse().expect("Please enter a valid number");

                let fahrenheit: f64 = converter::celsius_to_fahrenheit(celsius);

                println!("Exact value: {}", fahrenheit);
                println!("Rounded value: {}", (fahrenheit * 100.0).round() / 100.0);
            }
            "2" => {
                print!("Enter temperature in Fahrenheit: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");

                let fahrenheit: f64 = input.trim().parse().expect("Please enter a valid number");

                let celsius: f64 = converter::fahrenheit_to_celsius(fahrenheit);

                println!("Exact value: {}", celsius);
                println!("Rounded value: {}", (celsius * 100.0).round() / 100.0);
            }
            "3" => {
                print!("Enter distance in Km: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");

                let km: f64 = input.trim().parse().expect("Please enter a valid number");

                let miles: f64 = converter::km_to_miles(km);

                println!("Exact value: {}", miles);
                println!("Rounded value: {}", (miles * 100.0).round() / 100.0);
            }
            "4" => {
                print!("Enter distance in Miles: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");

                let miles: f64 = input.trim().parse().expect("Please enter a valid number");

                let km: f64 = converter::miles_to_km(miles);

                println!("Exact value: {}", km);
                println!("Rounded value: {}", (km * 100.0).round() / 100.0);
            }
            "0" => break,
            _ => println!("Invalid option."),
        }
    }
}
