pub use sdl2::rect::Rect;
use sdl2::pixels::Color;
use rand::Rng;
pub use sdl2::rect::Point;
use sdl2::render::{WindowCanvas, Texture};

#[derive(Debug, Eq, PartialEq)]
pub struct Case {
    pub rect: Rect,    // Représentation graphique de la case (sa position et ses dimensions)
    pub text: String,  // Contenu de la case (nombre de mines adjacentes, mine, etc.)
    pub action: String // État de la case (hidden, flag, see, etc.)
}

impl Case {
    // Constructeur pour initialiser une nouvelle case
    pub fn new(width: i32, height: i32, case_w: u32, case_h: u32) -> Self {
        Case {
            rect: Rect::new(width, height, case_w, case_h),
            text: String::new(),      // Par défaut, la case est vide
            action: String::from("hidden") // Par défaut, la case est cachée
        }
    }
}

// Fonction pour rendre (afficher) l'ensemble des cases sur le canvas
pub fn render(canvas: &mut WindowCanvas, color: Color, texture: &Vec<Texture>, all_rect: &mut Vec<Vec<Case>>, texture_loc: [Rect; 14]) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.set_draw_color(Color { r: 255, g: 255, b: 255, a: 255 });

    for row in all_rect {
        for case in row {
            // Choix de la texture à afficher en fonction de l'état de la case
            match case.action.as_str() {
                "hidden" => canvas.copy(&texture[0], texture_loc[9], case.rect)?,
                "flag" => canvas.copy(&texture[0], texture_loc[10], case.rect)?,
                "see" => canvas.copy(&texture[0], texture_loc[0], case.rect)?,
                _ => {
                    if case.text.is_empty() {
                        canvas.copy(&texture[0], texture_loc[0], case.rect)?
                    } else if case.text == "mine" {
                        canvas.copy(&texture[0], texture_loc[12], case.rect)?
                    } else if case.text == "mineRed" {
                        canvas.copy(&texture[0], texture_loc[13], case.rect)?
                    } else {
                        let numb: usize = case.text.parse().unwrap();
                        canvas.copy(&texture[0], texture_loc[numb], case.rect)?
                    }
                }
            }
        }
    }

    canvas.present();

    Ok(())
}

// Fonction pour placer les mines aléatoirement sur la grille
pub fn setup_mines(all_rect: &mut Vec<Vec<Case>>, number_mine: i32, y_case: i32, x_case: i32) {
    let mut mines = 0;
    let mine = String::from("mine");

    while mines != number_mine {
        let x = rand::thread_rng().gen_range(0..x_case as usize);
        let y = rand::thread_rng().gen_range(0..y_case as usize);

        // Si la case n'a pas déjà une mine, en place une
        if all_rect[y][x].text != mine {
            mines += 1;
            all_rect[y][x].text = mine.clone();
        }
    }
}

// Fonction pour réinitialiser toutes les cases de la grille
pub fn reset_all(all_rect: &mut Vec<Vec<Case>>) {
    for row in all_rect {
        for case in row {
            case.action = "hidden".to_string();
            case.text = String::new();
        }
    }
}

// Fonction pour définir la position des textures à utiliser
pub fn texture_location() -> [Rect; 14] {
    [
        Rect::new(0, 0, 16, 16),    // Rien : 0
        Rect::new(16, 0, 16, 16),   // Un : 1
        Rect::new(32, 0, 16, 16),   // Deux : 2
        Rect::new(48, 0, 16, 16),   // Trois : 3
        Rect::new(0, 16, 16, 16),   // Quatre : 4
        Rect::new(16, 16, 16, 16),  // Cinq : 5
        Rect::new(32, 16, 16, 16),  // Six : 6
        Rect::new(48, 16, 16, 16),  // Sept : 7
        Rect::new(0, 32, 16, 16),   // Huit : 8
        Rect::new(16, 32, 16, 16),  // Caché
        Rect::new(32, 32, 16, 16),  // Drapeau
        Rect::new(48, 32, 16, 16),  // Drapeau Rouge
        Rect::new(0, 48, 16, 16),   // Mine
        Rect::new(16, 48, 16, 16),  // Mine Rouge
    ]
}

// Fonction pour calculer le nombre de mines adjacentes pour chaque case
pub fn setup_number(all_rect: &mut Vec<Vec<Case>>, y_case: i32, x_case: i32) {
    let mine = String::from("mine");

    for y in 0..y_case as usize {
        for x in 0..x_case as usize {
            if all_rect[y][x].text != mine {
                let mut count = 0;

                // Calcul du nombre de mines adjacentes
                for add_y in 0..3 {
                    for add_x in 0..3 {
                        if y as i8 + 1 - add_y as i8 >= 0 && y + 1 - add_y <= y_case as usize - 1 &&
                            x as i8 + 1 - add_x as i8 >= 0 && x + 1 - add_x <= x_case as usize - 1 {
                            if all_rect[y + 1 - add_y][x + 1 - add_x].text == mine {
                                count += 1;
                            }
                        }
                    }
                }
                all_rect[y][x].text = count.to_string();
            }
        }
    }
}

// Fonction pour révéler une case lorsque le joueur clique dessus
pub fn unhidden(all_rect: &mut Vec<Vec<Case>>, x: i32, y: i32, x_length: i32, y_length: i32, stop: &mut bool, y_case: i32, x_case: i32, first: &mut bool, number_mine: i32) {
    let (y_to_check, x_to_check) = get_tiles_mouse(all_rect, x, y, x_length, y_length, y_case, x_case);
    if all_rect[y_to_check][x_to_check].action != "flag".to_string() {
        all_rect[y_to_check][x_to_check].action = "".to_string();
        if all_rect[y_to_check][x_to_check].text == "0".to_string() {
            unhidden_non_flag(all_rect, y_to_check, x_to_check, stop, y_case, x_case);
            *first = false;
        } else if all_rect[y_to_check][x_to_check].text == "mine".to_string() {
            if *first {
                reset_all(all_rect);
                setup_mines(all_rect, number_mine, y_case, x_case);
                setup_number(all_rect, y_case, x_case);
                unhidden(all_rect, x, y, x_length, y_length, stop, y_case, x_case, first, number_mine);
            } else {
                all_rect[y_to_check][x_to_check].text = "mineRed".to_string();
                see_all_mine(all_rect);
                *stop = true;
            }
        } else if *first {
            *first = false;
        }
    }
}

// Fonction pour basculer entre poser et retirer un drapeau sur une case
pub fn toggle_flag(all_rect: &mut Vec<Vec<Case>>, x: i32, y: i32, x_length: i32, y_length: i32, y_case: i32, x_case: i32) {
    let (y_to_check, x_to_check) = get_tiles_mouse(all_rect, x, y, x_length, y_length, y_case, x_case);

    if all_rect[y_to_check][x_to_check].action == "hidden".to_string() {
        all_rect[y_to_check][x_to_check].action = "flag".to_string();
    } else if all_rect[y_to_check][x_to_check].action == "flag".to_string() {
        all_rect[y_to_check][x_to_check].action = "hidden".to_string();
    }
}

// Fonction pour révéler les cases adjacentes si elles ne sont pas marquées comme drapeau
pub fn unhidden_non_flag(all_rect: &mut Vec<Vec<Case>>, y: usize, x: usize, stop: &mut bool, y_case: i32, x_case: i32) {
    for add_y in 0..3 {
        for add_x in 0..3 {
            if y as i8 + 1 - add_y as i8 >= 0 && y + 1 - add_y <= y_case as usize - 1 &&
                x as i8 + 1 - add_x as i8 >= 0 && x + 1 - add_x <= x_case as usize - 1 {
                if all_rect[y + 1 - add_y][x + 1 - add_x].action == "hidden".to_string() {
                    all_rect[y + 1 - add_y][x + 1 - add_x].action = String::new();
                    if all_rect[y + 1 - add_y][x + 1 - add_ax].text == "0".to_string() {
                        unhidden_non_flag(all_rect, y + 1 - add_y, x + 1 - add_ax, stop, y_case, x_case);
                    } else if all_rect[y + 1 - add_y][x + 1 - add_ax].text == "mine".to_string() {
                        all_rect[y + 1 - add_y][x + 1 - add_ax].text = "mineRed".to_string();
                        see_all_mine(all_rect);
                        *stop = true;
                    }
                }
            }
        }
    }
}

// Fonction pour vérifier si toutes les cases adjacentes sont marquées correctement
pub fn if_all_flag_unhidden(all_rect: &mut Vec<Vec<Case>>, x: i32, y: i32, x_length: i32, y_length: i32, stop: &mut bool, y_case: i32, x_case: i32) {
    let (y_to_check, x_to_check) = get_tiles_mouse(all_rect, x, y, x_length, y_length, y_case, x_case);
    if all_rect[y_to_check][x_to_check].action == String::new() {
        let mut count = 0;
        for add_y in 0..3 {
            for add_x in 0..3 {
                if y_to_check as i8 + 1 - add_y as i8 >= 0 && y_to_check + 1 - add_y <= y_case as usize - 1 &&
                    x_to_check as i8 + 1 - add_x as i8 >= 0 && x_to_check + 1 - add_x <= x_case as usize - 1 {
                    if all_rect[y_to_check + 1 - add_y][x_to_check + 1 - add_ax].action == "flag".to_string() {
                        count += 1;
                    }
                }
            }
        }
        if all_rect[y_to_check][x_to_check].text == count.to_string() {
            unhidden_non_flag(all_rect, y_to_check, x_to_check, stop, y_case, x_case);
        }
    }
}

// Fonction pour afficher temporairement les cases adjacentes lors d'un clic central
pub fn see_selected(all_rect: &mut Vec<Vec<Case>>, x: i32, y: i32, x_length: i32, y_length: i32, y_case: i32, x_case: i32) {
    let (y_to_check, x_to_check) = get_tiles_mouse(all_rect, x, y, x_length, y_length, y_case, x_case);
    for add_y in 0..3 {
        for add_x in 0..3 {
            if y_to_check as i8 + 1 - add_y as i8 >= 0 && y_to_check + 1 - add_y <= y_case as usize - 1 &&
                x_to_check as i8 + 1 - add_ax as i8 >= 0 && x_to_check + 1 - add_ax <= x_case as usize - 1 {
                if all_rect[y_to_check + 1 - add_y][x_to_check + 1 - add_ax].action == "hidden".to_string() {
                    all_rect[y_to_check + 1 - add_y][x_to_check + 1 - add_ax].action = "see".to_string();
                }
            }
        }
    }
}

// Fonction pour afficher temporairement une case lorsqu'elle est sélectionnée
pub fn see_click(all_rect: &mut Vec<Vec<Case>>, x: i32, y: i32, x_length: i32, y_length: i32, y_case: i32, x_case: i32) {
    let (y_to_check, x_to_check) = get_tiles_mouse(all_rect, x, y, x_length, y_length, y_case, x_case);
    if all_rect[y_to_check][x_to_check].action == "hidden".to_string() {
        all_rect[y_to_check][x_to_check].action = "see".to_string();
    }
}

// Fonction pour retirer l'affichage temporaire des cases sélectionnées
pub fn remove_see(all_rect: &mut Vec<Vec<Case>>) {
    for row in all_rect {
        for case in row {
            if case.action == "see".to_string() {
                case.action = "hidden".to_string();
            }
        }
    }
}

// Fonction pour révéler toutes les mines lorsque le joueur perd
pub fn see_all_mine(all_rect: &mut Vec<Vec<Case>>) {
    let mine = "mine".to_string();
    for row in all_rect {
        for case in row {
            if case.text == mine {
                case.action = String::new();
            }
        }
    }
}

// Fonction pour obtenir la case sur laquelle la souris a cliqué
fn get_tiles_mouse(all_rect: &mut Vec<Vec<Case>>, x: i32, y: i32, x_length: i32, y_length: i32, y_case: i32, x_case: i32) -> (usize, usize) {
    let mut brk = false;
    let mut res: (usize, usize) = (0, 0);
    for y_to_check in 0..y_case as usize {
        for x_to_check in 0..x_case as usize {
            if x <= all_rect[y_to_check][x_to_check].rect.x() + x_length && x >= all_rect[y_to_check][x_to_check].rect.x() &&
                y <= all_rect[y_to_check][x_to_check].rect.y() + y_length && y >= all_rect[y_to_check][x_to_check].rect.y() {
                res = (y_to_check, x_to_check);
                brk = true;
                break;
            }
        }
        if brk {
            break;
        }
    }
    res
}
