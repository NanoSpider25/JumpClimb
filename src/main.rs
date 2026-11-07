mod background;

use macroquad::prelude::*;
use background::Mountain;

#[macroquad::main("JumpClimb")]
async fn main() {

    let mut bg = Mountain::new();
    bg.init();
    loop {
        clear_background(DARKGRAY);

        bg.draw();

        next_frame().await
    }
}
