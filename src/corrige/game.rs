extern crate sdl2;

mod mine;
use sdl2::mouse::MouseButton;
use mine::*;
use sdl2::event::Event;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn game(x_case: i32, y_case: i32, number_mine: i32) {
    // Calcul de la largeur et de la hauteur de la fenêtre en fonction du nombre de cases
    let width: i32 = 32 * x_case;
    let height: i32 = 32 * y_case;
    let mut first = true;

    // Initialisation de la bibliothèque SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Création de la fenêtre du jeu
    let window = video_subsystem
        .window("Minesweeper", width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();

    // Création du canvas pour dessiner la fenêtre
    let mut canvas = window.into_canvas().build().unwrap();
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);
    let texture_creator = canvas.texture_creator();
    let texture = vec![texture_creator.load_texture("assets/Minesweeper_simple_texture_atlas.png").unwrap()];

    // Configuration initiale du canvas (arrière-plan noir)
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // Gestionnaire d'événements SDL2
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut all_rct: Vec<Vec<Case>> = vec![];

    // Initialisation de la grille de jeu avec des cases vides
    for a in 0..y_case {
        let mut stock: Vec<Case> = Vec::new();
        for e in 0..x_case {
            let case = Case::new((width / x_case) * e, (height / y_case) * a, (width / x_case) as u32, (height / y_case) as u32);
            stock.push(case)
        }
        all_rct.push(stock);
    }

    // Placement des mines et calcul des nombres de mines adjacentes pour chaque case
    setup_mines(&mut all_rct, number_mine, y_case, x_case);
    setup_number(&mut all_rct, y_case, x_case);

    let mut stop = false;
    let mut redo_mouse_ckick = false;
    let mut redo_left_ckick = false;
    let texture_location: [Rect; 14] = texture_location();

    'running: loop {
        remove_see(&mut all_rct); // Réinitialise l'état visuel des cases sélectionnées

        // Gestion des événements (clavier et souris)
        for event in event_pump.poll_iter() {
            match event {
                // Gestion du clic gauche pour révéler une case
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => if !stop {
                    redo_left_ckick = false;
                    unhidden(&mut all_rct, state.x(), state.y(), width / x_case, height / y_case, &mut stop, y_case, x_case, &mut first, number_mine);
                },
                // Gestion du clic droit pour poser un drapeau
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Right,
                    ..
                } => if !stop {
                    toggle_flag(&mut all_rct, state.x(), state.y(), width / x_case, height / y_case, y_case, x_case);
                },
                // Gestion du clic central pour révéler les cases adjacentes si toutes les mines adjacentes sont marquées
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Middle,
                    ..
                } => if !stop {
                    redo_mouse_ckick = false;
                    remove_see(&mut all_rct);
                    if_all_flag_unhidden(&mut all_rct, state.x(), state.y(), width / x_case, height / y_case, &mut stop, y_case, x_case)
                },
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Middle,
                    ..
                } => if !stop {
                    redo_mouse_ckick = true;
                    see_selected(&mut all_rct, state.x(), state.y(), width / x_case, height / y_case, y_case, x_case);
                },
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => if !stop {
                    redo_left_ckick = true;
                    see_click(&mut all_rct, state.x(), state.y(), width / x_case, height / y_case, y_case, x_case);
                },
                // Gestion des actions par le clavier (touches A, E, Z)
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => if !stop {
                    redo_left_ckick = false;
                    unhidden(&mut all_rct, state.x(), state.y(), width / x_case, height / y_case, &mut stop, y_case, x_case, &mut first, number_mine);
                },
                Event::KeyUp {
                    keycode: Some(Keycode::E),
                    ..
                } => if !stop {
                    toggle_flag(&mut all_rct, state.x(), state.y(), width / x_case, height / y_case, y_case, x_case);
                },
                Event::KeyUp {
                    keycode: Some(Keycode::Z),
                    ..
                } => if !stop {
                    redo_mouse_ckick = false;
                    remove_see(&mut all_rct);
                    if_all_flag_unhidden(&mut all_rct, state.x(), state.y(), width / x_case, height / y_case, &mut stop, y_case, x_case)
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => if !stop {
                    redo_mouse_ckick = true;
                    see_selected(&mut all_rct, state.x(), state.y(), width / x_case, height / y_case, y_case, x_case);
                },
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => if !stop {
                    redo_left_ckick = true;
                    see_click(&mut all_rct, state.x(), state.y(), width / x_case, height / y_case, y_case, x_case);
                },
                // Réinitialisation du jeu avec la touche R
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    reset_all(&mut all_rct);
                    setup_mines(&mut all_rct, number_mine, y_case, x_case);
                    setup_number(&mut all_rct, y_case, x_case);
                    stop = false;
                    first = true;
                },

                /*coriger exo
                 */
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => {
                    see_all_mine(&mut all_rct); // Appelle la fonction pour révéler toutes les mines
                }
                Event::KeyDown {
                    keycode: Some(Keycode::G),
                    ..
                } => {
                    hide_all_mine(&mut all_rct); // Appelle la fonction pour révéler toutes les mines
                }
/*
              fin corrige exo
                 */


                // Quitter le jeu avec la touche Echap ou en fermant la fenêtre
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Réaffiche l'état visuel des cases sélectionnées si nécessaire
        if redo_mouse_ckick {
            see_selected(&mut all_rct, state.x(), state.y(), width / x_case, height / y_case, y_case, x_case);
        } else if redo_left_ckick {
            see_click(&mut all_rct, state.x(), state.y(), width / x_case, height / y_case, y_case, x_case);
        }

        // Efface le canvas et prépare le dessin
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Dessine la grille de jeu avec les textures correspondantes
        render(&mut canvas, Color::RGB(0, 0, 0), &texture, &mut all_rct, texture_location).unwrap();

        // Limite la vitesse de rafraîchissement à environ 60 FPS
        std::thread::sleep(Duration::from_millis(16));
        canvas.present();
    }
}
