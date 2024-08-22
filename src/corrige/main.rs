mod game;

use std::fs;
use game::*;

// Fonction principale du programme
fn main() {
    // Lecture du fichier de configuration (setting.txt) pour obtenir les paramètres du jeu
    let contents = fs::read_to_string("setting.txt")
        .expect("Should have been able to read the file");

    let mut actual = 0;
    let mut x_lenght = String::new();
    let mut y_lenght = String::new();
    let mut mines = String::new();

    // Analyse des caractères du fichier pour extraire les dimensions de la grille et le nombre de mines
    contents.chars().for_each(|cha| {
        if cha.is_ascii_digit() {
            if actual == 0 {
                x_lenght += cha.to_string().as_str(); // Stocke la longueur en X
            } else if actual == 1 {
                y_lenght += cha.to_string().as_str(); // Stocke la longueur en Y
            } else if actual == 2 {
                mines += cha.to_string().as_str(); // Stocke le nombre de mines
            }
        } else if cha == '\n' {
            actual += 1; // Passe à la prochaine ligne du fichier pour les paramètres suivants
        }
    });

    // Vérifie si le nombre de mines n'excède pas le nombre de cases disponibles
    if x_lenght.parse::<i32>().unwrap() * y_lenght.parse::<i32>().unwrap() < mines.parse::<i32>().unwrap() {
        println!("Too many mines for the grid size");
    } else {
        // Démarre le jeu avec les paramètres spécifiés
        game(x_lenght.parse().unwrap(), y_lenght.parse().unwrap(), mines.parse().unwrap())
    }
}
