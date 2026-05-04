//A small CLI program that converts:

// Fahrenheit to Celsius
// Celsius to Fahrenhei


// Core formulas
// C = (F - 32) * 5 / 9
// F = (C * 9 / 5) + 3

// Choose conversion:
// 1. Fahrenheit to Celsius
// 2. Celsius to Fahrenheit

// Enter choice: 1
// Enter temperature: 98.6
// Result: 37.0 C

use std::io;

fn fahrenheit_to_celsius(f: &f64) -> f64 {
    (f-32f64) * 5f64 / 9f64
}

fn celsius_to_fahrenheit(c: &f64) -> f64 {
    (c * 9f64/5f64) + 3f64
}

fn main() {

    println!("Choose conversion");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");


    let mut choice = String::new();

    io::stdin().read_line(&mut choice)
                .expect("Failed to read input");
    let choice = choice.trim();
    match choice {
        "1" => println!("Enter Fahrenheit value"),
        "2" => println!("Enter celsius value"),
        _ => println!("Invalid input, enter 1 or 2")
    };

    let mut temp_value = String::new();

    io::stdin().read_line(&mut temp_value)
                .expect("Failed to read value");
                
    let temp_value: f64 = temp_value.trim().parse().expect("Failed to parse value to int");

    match choice {
        "1" => {
          let c = fahrenheit_to_celsius(&temp_value);
          println!("{}°F is equivalent to {}°C", temp_value, c);
        },
        "2" => {
           let f = celsius_to_fahrenheit(&temp_value);
           println!("{}°c is equivalent to {}°F", temp_value, f)
        },
        _ => println!("Invalid choice")
    }

    
}
