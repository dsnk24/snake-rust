/*

   Copyright 2020 Tabacaru Eric

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.

*/


extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use crate::game::Game;
use crate::draw::to_coord_u32;


const BG_COLOR: Color = [0.50, 0.50, 0.50, 1.00];


fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow = WindowSettings::new(
        "Snake game with Rust",
        [to_coord_u32(width), to_coord_u32(height)]
    ).exit_on_esc(true).build().unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _w| {
            clear(BG_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });


    }
}
