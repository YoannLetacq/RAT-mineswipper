mod game;


use std::fs;
use game::*;


fn main() {
    let contents = fs::read_to_string("setting.txt")
        .expect("Should have been able to read the file");

    let mut actual = 0;
    let mut x_lenght = String::new();
    let mut y_lenght = String::new();
    let mut mines = String::new();

    contents.chars().for_each(|cha| {
        if cha.is_ascii_digit() {
            if actual == 0 {
                x_lenght += cha.to_string().as_str();
            } else if actual == 1 {
                y_lenght += cha.to_string().as_str();
            } else if actual == 2 {
                mines += cha.to_string().as_str();
            }
        } else if cha == '\n' {
            actual += 1;
        }
    }) ;
    if x_lenght.parse::<i32>().unwrap() * y_lenght.parse::<i32>().unwrap() < mines.parse::<i32>().unwrap() {
        println!("To Much Mines");
    } else {
        game(x_lenght.parse().unwrap() ,y_lenght.parse().unwrap() , mines.parse().unwrap())
    }
}