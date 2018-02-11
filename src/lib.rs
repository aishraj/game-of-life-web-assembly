#[macro_use]
extern crate lazy_static;

mod models;
mod game_state;


use std::sync::Mutex;
use std::os::raw::c_int;

use game_state::GameState;
use self::models::Size;

lazy_static! {
    static  ref DATA: Mutex<GameState> = Mutex::new(new_game_state(1024, 786));
}

fn new_game_state(width: i32, height: i32) -> GameState {
    GameState::new(Size::new(width, height))
}

// These functions are provided by the JavaScript code
extern "C" {
    fn clear_screen();
    fn draw_dead_cell(_: c_int, _: c_int);
    fn draw_living_cell(_: c_int, _: c_int);
}

#[no_mangle]
pub extern "C" fn resize(width: c_int, height: c_int) {
    *DATA.lock().unwrap() = new_game_state(width, height);
    //Set some handpicked "random" cells as alive.
    let random_living = [(1, 5),(1, 6),(2, 5),(2, 6),(11, 5),(11, 6),(11, 7),(12, 4),(12, 8),(13, 3),(13, 9),(14, 3),(14, 9),(15, 6),(16, 4),(16, 8),(17, 5),(17, 6),(17, 7),(18, 6),(21, 3),(21, 4),(21, 5),(22, 3),(22, 4),(22, 5),(23, 2),(23, 6),(25, 1),(25, 2),(25, 6),(25, 7),(35, 3),(35, 4),(36, 3),(36, 4),(60, 47),(61,47),(62,47),
        (60, 48),(61,48),(62,48),
        (60, 49),(61,49),(62,49),
        (60, 51),(61,51),(62,51)];
    let game_state: &mut GameState = &mut DATA.lock().unwrap() ;
    let current_generation = &mut game_state.current_generation;
    for &(x,y) in random_living.iter() {
        current_generation.set_living(y,x);
    }
}

#[no_mangle]
pub extern "C" fn draw() {
    let game_state = &mut DATA.lock().unwrap();
    let drawing_height = game_state.current_generation.size.height as i32;
    let drawing_width = game_state.current_generation.size.width as i32;
    unsafe { clear_screen() };
    for i in 0..drawing_width {
        for j in 0..drawing_height {
            if game_state.current_generation.is_living(i, j) {
                unsafe { draw_living_cell(i, j); }
            } else {
                unsafe { draw_dead_cell(i, j); }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn calculate_next_gen() {
    let game_state: &mut GameState = &mut DATA.lock().unwrap() ;
    let current_generation = &mut game_state.current_generation;
    let next_generation = &mut game_state.next_generation;
    let drawing_height = current_generation.size.height as i32;
    let drawing_width = current_generation.size.width as i32;
    for x in 0..drawing_width {
        for y in 0..drawing_height {
            let neighbour_count = current_generation.get_neighbour_count(x,y);
            let cell_state = current_generation.is_living(x, y);
            match neighbour_count {
                0...1 if cell_state == true => next_generation.set_dead(x, y),
                2...3 if cell_state == true => next_generation.set_living(x, y),
                3 if cell_state == false => next_generation.set_living(x, y),
                _ if cell_state == true => next_generation.set_dead(x, y),
                _ => next_generation.set_dead(x, y)
            };
        }
    }
}

#[no_mangle]
pub extern "C" fn update_state() {
    let game_state: &mut GameState = &mut DATA.lock().unwrap() ;
    //Now we're done with the current generation, we use the next generation as the current one.
    game_state.flip_generations();
}
