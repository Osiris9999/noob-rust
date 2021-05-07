use std::io;
const MULTIPLY: f32 = 1.8;
const ADD: f32 = 32.0;
fn main() {
    // program to change farenheit to celsius
    
    
    let mut cel =  String::new();
    println!("Input your temperature");

    io::stdin().read_line(&mut cel).expect("wrong shit");

    let cel: f32 = cel.trim().parse().expect("please input integer");
    
    println!("input was {}", cel);
    
    conversion(cel);
}

fn conversion(s: f32) -> f32 {
    
    let s = (s * MULTIPLY) + ADD;
    println!("The temperature in farenheit is {}", s);
    return s;
}
