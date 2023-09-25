pub use sdl2::rect::Rect;
use sdl2::pixels::Color;
use rand::Rng;
pub use sdl2::rect::Point;
use sdl2::render::{WindowCanvas, Texture};



 
#[derive(Debug,Eq,PartialEq)]
pub struct Case {
    pub rect: Rect,
    pub text: String,
    pub action: String
}

impl Case {
    pub fn new(width: i32 , height: i32 , case_w: u32 , case_h: u32 ,) -> Self {
        Case{
            rect : Rect::new(width, height, case_w, case_h),
            text: String::new(),
            action: String::from("hidden")
        }
    }
}
 

pub fn render(canvas: &mut WindowCanvas, color: Color, texture: &Vec<Texture> , all_rect: &mut Vec<Vec<Case>> , texture_loc : [Rect; 14] ) -> Result<(), String>  {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.set_draw_color(Color{r: 255 , g: 255 , b: 255 , a: 255});
    for a in all_rect {
        for b in a {
            if b.action == "hidden".to_string() {
                canvas.copy(&texture[0], texture_loc[9], b.rect)?;
            } else if b.action == "flag".to_string()   {
                canvas.copy(&texture[0], texture_loc[10], b.rect)?;
            } else if b.action == "see".to_string(){
                canvas.copy(&texture[0], texture_loc[0], b.rect)?;
            } else {
                if b.text == "".to_string() {
                    canvas.copy(&texture[0], texture_loc[0], b.rect)?;
                } else if b.text == "mine".to_string() {
                    canvas.copy(&texture[0], texture_loc[12], b.rect)?;
                } else if b.text == "mineRed".to_string() {
                    canvas.copy(&texture[0], texture_loc[13], b.rect)?;
                } else {
                    let numb: usize = b.text.clone().parse().unwrap();
                    
                    canvas.copy(&texture[0], texture_loc[numb], b.rect)?;
                }
            }
        }
    }    

    canvas.present();

    Ok(())
}


pub fn setup_mines(all_rect: &mut Vec<Vec<Case>> , number_mine : i32 , y_case: i32 , x_case : i32) {
    let mut mines = 0;
    let mine = String::from("mine");
    while mines != number_mine {
        let x = rand::thread_rng().gen_range(0..x_case as usize);
        let y = rand::thread_rng().gen_range(0..y_case as usize);
        if all_rect[y][x].text != mine {
            mines += 1;
            all_rect[y][x].text = mine.clone();
        }
    }

}

pub fn reset_all(all_rect: &mut Vec<Vec<Case>>) {
    for a in all_rect {
        for b in a {
            b.action = "hidden".to_string();
            b.text = String::new();
        }
    }
}

pub fn texture_location() -> [Rect;14] {
    [
    Rect::new(0, 0, 16, 16), //nothing : 0
    Rect::new(16, 0, 16, 16), //One : 1
    Rect::new(32, 0, 16, 16), //Two : 2
    Rect::new(48, 0, 16, 16), //...
    Rect::new(0, 16, 16, 16), //...
    Rect::new(16, 16, 16, 16), //...
    Rect::new(32, 16, 16, 16), //...
    Rect::new(48, 16, 16, 16), //...
    Rect::new(0, 32, 16, 16), // 8
    Rect::new(16, 32, 16, 16), //Hidden
    Rect::new(32, 32, 16, 16), //Flag 
    Rect::new(48, 32, 16, 16), //Red Flag
    Rect::new(0, 48, 16, 16), //Mine
    Rect::new(16, 48, 16, 16), //Red Mine
]
}

pub fn setup_number(all_rect: &mut Vec<Vec<Case>> , y_case: i32 , x_case: i32) {
    let mine = String::from("mine");
    for y in 0..y_case as usize {
        for x in 0..x_case as usize {
            if all_rect[y][x].text != mine {
                let mut count = 0;
                for add_y in 0..3 {
                    for add_x in 0..3 {
                        if y as i8 +1 - add_y as i8 >= 0 && y+1 - add_y <= y_case as usize - 1 && x as i8 + 1 - add_x as i8 >= 0 && x + 1 - add_x <= x_case as usize -1 {
                            if all_rect[y+1 - add_y][x+1 - add_x].text == mine {
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

pub fn unhidden(all_rect: &mut Vec<Vec<Case>> , x: i32 , y: i32, x_length: i32, y_length: i32 ,stop: &mut bool, y_case: i32 , x_case: i32 , first : &mut bool , number_mine : i32) {
    let (y_to_check,x_to_check) = get_tiles_mouse(all_rect, x, y, x_length, y_length, y_case, x_case);
                if all_rect[y_to_check][x_to_check].action != "flag".to_string() {
                                    all_rect[y_to_check][x_to_check].action = "".to_string();
                                    if all_rect[y_to_check][x_to_check].text == "0".to_string() {
                                        unhidden_non_flag(all_rect, y_to_check, x_to_check, stop , y_case , x_case);
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

pub fn toggle_flag(all_rect: &mut Vec<Vec<Case>> , x: i32 , y: i32, x_length: i32, y_length: i32 , y_case: i32 , x_case: i32 ) {
    let (y_to_check,x_to_check) = get_tiles_mouse(all_rect, x, y, x_length, y_length, y_case, x_case);
    
                if all_rect[y_to_check][x_to_check].action == "hidden".to_string() {
                                    all_rect[y_to_check][x_to_check].action = "flag".to_string();
                } else if all_rect[y_to_check][x_to_check].action == "flag".to_string() {
                    all_rect[y_to_check][x_to_check].action = "hidden".to_string();
                }
                
    
}

pub fn unhidden_non_flag(all_rect: &mut Vec<Vec<Case>> , y: usize , x: usize , stop : &mut bool, y_case: i32 , x_case: i32) {
    for add_y in 0..3 {
        for add_x in 0..3 {
            if y as i8 +1 - add_y as i8 >= 0 && y+1 - add_y <= y_case as usize - 1 && x as i8 + 1 - add_x as i8 >= 0 && x + 1 - add_x <= x_case as usize -1 {
                if all_rect[y+1 - add_y][x+1 - add_x].action == "hidden".to_string() {
                    all_rect[y+1 - add_y][x+1 - add_x].action = String::new();
                    if all_rect[y+1 - add_y][x+1 - add_x].text == "0".to_string() {
                        unhidden_non_flag(all_rect, y+1 - add_y, x+1 - add_x , stop , y_case , x_case);
                    } else if all_rect[y+1 - add_y][x+1 - add_x].text == "mine".to_string() {
                        all_rect[y+1 - add_y][x+1 - add_x].text = "mineRed".to_string();
                        see_all_mine(all_rect);
                        *stop = true;
                    }
                }
            }
        }   
    }
}

pub fn if_all_flag_unhidden(all_rect: &mut Vec<Vec<Case>> , x: i32 , y: i32, x_length: i32, y_length: i32 , stop: &mut bool , y_case: i32 , x_case: i32) {
    let (y_to_check,x_to_check) = get_tiles_mouse(all_rect, x, y, x_length, y_length, y_case, x_case);
            if all_rect[y_to_check][x_to_check].action == String::new() {
                let mut count = 0;
                    for add_y in 0..3 {
                        for add_x in 0..3 {
                            if y_to_check as i8 +1 - add_y as i8 >= 0 && y_to_check+1 - add_y <= y_case as usize -1 &&
                             x_to_check as i8 + 1 - add_x as i8 >= 0 && x_to_check + 1 - add_x <= x_case as usize -1 {
                                if all_rect[y_to_check+1 - add_y][x_to_check+1 - add_x].action == "flag".to_string() {
                                    count += 1;
                                }
                            }
                        }   
                    }
                    if all_rect[y_to_check][x_to_check].text == count.to_string() {
                        unhidden_non_flag(all_rect, y_to_check, x_to_check , stop, y_case , x_case)
                    }
               

    }
}

pub fn see_selected(all_rect: &mut Vec<Vec<Case>> , x: i32 , y: i32, x_length: i32, y_length: i32 , y_case: i32 , x_case: i32) {
    let (y_to_check,x_to_check) = get_tiles_mouse(all_rect, x, y, x_length, y_length, y_case, x_case);
                    for add_y in 0..3 {
                        for add_x in 0..3 {
                            if y_to_check as i8 +1 - add_y as i8 >= 0 && y_to_check+1 - add_y <= y_case as usize - 1 &&
                               x_to_check as i8 + 1 - add_x as i8 >= 0 && x_to_check + 1 - add_x <= x_case as usize - 1 {
                                if all_rect[y_to_check+1 - add_y][x_to_check+1 - add_x].action == "hidden".to_string() {
                                    all_rect[y_to_check+1 - add_y][x_to_check+1 - add_x].action = "see".to_string();
                                }
                            }
                        }   
                    }
}

pub fn see_click(all_rect: &mut Vec<Vec<Case>> , x: i32 , y: i32, x_length: i32, y_length: i32 , y_case: i32 , x_case: i32) {
    let (y_to_check,x_to_check) = get_tiles_mouse(all_rect, x, y, x_length, y_length, y_case, x_case);
           if  all_rect[y_to_check][x_to_check].action == "hidden".to_string() {
                    all_rect[y_to_check][x_to_check].action = "see".to_string();
            }
}

pub fn remove_see(all_rect: &mut Vec<Vec<Case>>) {
    for a in all_rect {
        for b in a {
            if b.action == "see".to_string() {
                b.action = "hidden".to_string();
            }
        }
    }
}

pub fn see_all_mine(all_rect: &mut Vec<Vec<Case>>) {
    let mine = "mine".to_string();
    for a in all_rect {
        for b in a {
            if b.text == mine {
                b.action = String::new();
            }
        }
    }
}

fn get_tiles_mouse(all_rect: &mut Vec<Vec<Case>> , x: i32 , y: i32 , x_length: i32, y_length: i32 , y_case: i32 , x_case: i32) -> (usize,usize) {
    let mut brk = false;
    let mut res: (usize, usize) = (0,0); 
    for y_to_check in 0..y_case as usize {
        for x_to_check in 0..x_case as usize {
            if x <= all_rect[y_to_check][x_to_check].rect.x() + x_length && x >= all_rect[y_to_check][x_to_check].rect.x() &&
            y <= all_rect[y_to_check][x_to_check].rect.y() + y_length && y >= all_rect[y_to_check][x_to_check].rect.y() {
                res = (y_to_check,x_to_check);
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