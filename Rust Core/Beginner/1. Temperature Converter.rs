// Assignment: Temperature Converter Program
// Create a Rust program that converts temperatures between Celsius and Fahrenheit using variables, constants, and proper naming conventions.
// Requirements:
// Constants:
// Define constants for freezing and boiling points of water in both Celsius and Fahrenheit
// Use proper naming conventions for constants
// Variables:
// Use immutable variables where appropriate
// Use mutable variables only when necessary
// Demonstrate variable shadowing
// Use proper type annotations
// Functions:
// Create functions to convert from Celsius to Fahrenheit and vice versa
// Use descriptive function names following Rust conventions
// User Input:
// Accept user input for a temperature value
// Accept user input for the unit (C or F)
// Convert to the other unit and display the result
// Naming Conventions:
// Follow Rust naming conventions throughout your code
// Use descriptive variable and function names

// Temperature Converter
// Enter temperature value: 25
// Enter unit (C for Celsius, F for Fahrenheit): C
// 25°C is equal to 77°F


use std::io;

// Constants for temperature reference points using SCREAMING_SNAKE_CASE
const FREEZING_POINT_C: f64 = 0.0;
const BOILING_POINT_C: f64 = 100.0;
const FREEZING_POINT_F: f64 = 32.0;
const BOILING_POINT_F: f64 = 212.0;

fn main() {
    println!("Temperature Converter");
    
    // Get temperature value from user
    let mut temp_input = String::new();
    println!("Enter temperature value:");
    io::stdin()
        .read_line(&mut temp_input)
        .expect("Failed to read temperature");
    
    // Parse the input, handling both integer and floating-point values
    let temp_value: f64 = match temp_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };
    
    // Get unit from user
    let mut unit_input = String::new();
    println!("Enter unit (C for Celsius, F for Fahrenheit):");
    io::stdin()
        .read_line(&mut unit_input)
        .expect("Failed to read unit");
    
    // Convert based on input unit
    let unit = unit_input.trim().to_uppercase();
    
    if unit == "C" {
        // Convert Celsius to Fahrenheit
        let fahrenheit = celsius_to_fahrenheit(temp_value);
        println!("{}°C is equal to {:.1}°F", temp_value, fahrenheit);
    } else if unit == "F" {
        // Convert Fahrenheit to Celsius
        let celsius = fahrenheit_to_celsius(temp_value);
        println!("{}°F is equal to {:.1}°C", temp_value, celsius);
    } else {
        println!("Invalid unit! Please enter C or F.");
        return;
    }
    
    // Demonstrate variable shadowing by formatting the output message
    let message = "Water freezes and boils at:";
    let message = format!("{} {}°C ({}°F) and {}°C ({}°F)", 
                         message, 
                         FREEZING_POINT_C, FREEZING_POINT_F, 
                         BOILING_POINT_C, BOILING_POINT_F);
    println!("\n{}", message);
    
    // Temperature comparison using shadowing
    let temp_status = if unit == "C" {
        compare_celsius_temp(temp_value)
    } else {
        compare_fahrenheit_temp(temp_value)
    };
    
    // Shadow the variable with a more detailed message
    let temp_status = format!("Your temperature is {}", temp_status);
    println!("{}", temp_status);
}

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 1.8) + 32.0
}

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

// Function to compare Celsius temperature to freezing/boiling points
fn compare_celsius_temp(temp: f64) -> &'static str {
    if temp <= FREEZING_POINT_C {
        "at or below freezing point"
    } else if temp >= BOILING_POINT_C {
        "at or above boiling point"
    } else {
        "between freezing and boiling points"
    }
}

// Function to compare Fahrenheit temperature to freezing/boiling points
fn compare_fahrenheit_temp(temp: f64) -> &'static str {
    if temp <= FREEZING_POINT_F {
        "at or below freezing point"
    } else if temp >= BOILING_POINT_F {
        "at or above boiling point"
    } else {
        "between freezing and boiling points"
    }
}