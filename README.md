# Wein2D.js-WASM
Wein2D.js bindings for creating browser games in Rust using WebAssembly.

## Code example
This is a simple Example for a program (rust):
```rust
mod wein2d_js;

use wein2d_js::*;
use wasm_bindgen::prelude::*;

// code for statically storing a GameState-struct

static mut STATIC_GAME_STATE: Option<GameState> = None;

fn set_game_state(game_state: GameState) { unsafe { STATIC_GAME_STATE = Some(game_state); } }

fn get_game_state_mut() -> &'static mut GameState { unsafe {
    match STATIC_GAME_STATE {
        Some(ref mut game_state) => { game_state }
        None => { panic!("Static Game State not defined.") }
    }
}}

struct GameState {
    height: f32,
    velocity: f32,
}

// on_start and on_frame functions

#[wasm_bindgen]
pub fn on_start() { // gets called directly after initialization
    set_game_state(GameState {
        height: 0.0,
        velocity: 0.0,
    });
}

// cube values (feel free to play with these!)
static CUBE_SIZE: f32 = 25.0; // the width and height of the cube
static CUBE_JUMP_VELOCITY: f32 = 400.0; // the cube's jump velocity (in pixels per second)
static CUBE_GRAVITATION: f32 = 800.0; // gravitation (how much velocity gets removed per second)
static CUBE_BOUNCEBACK_MULTIPLIER: f32 = 0.4; // how much velocity the cube keeps after hitting the ground

#[wasm_bindgen]
pub fn on_frame() { // gets called once per frame
    // get game state
    let mut game_state: &mut GameState = get_game_state_mut();

    // update calls //////////////////////////////////////////////////

    // if space bar or left mouse button pressed, set the cubes velocity (let the cube jump up)
    if get_key(Key::SPACE) || get_mouse_l() { game_state.velocity = CUBE_JUMP_VELOCITY; }

    // move the cube up and down according to it's velocity
    game_state.height += game_state.velocity * get_delta_time();
    // if the cube is not on the ground remove some of the cube's velocity (gravitation)
    if game_state.height > 0.0 { game_state.velocity -= CUBE_GRAVITATION * get_delta_time() }

    // if the cube is below or on the ground, set him onto the ground, invert the cube's velocity (movement) and remove some of it's velocity
    if game_state.height <= 0.0 {
        game_state.velocity = -game_state.velocity * CUBE_BOUNCEBACK_MULTIPLIER;
        game_state.height = 0.0;
    }

    // render calls //////////////////////////////////////////////////

    // fill the screen with blue
    fill(255, 11, 138, 143);

    // draw the cube
    draw_rect(
        (get_width() - CUBE_SIZE) / 2.0, // draw at the center of the screen (x axis)
        get_height() - CUBE_SIZE - game_state.height, // draw at the cube's height (y axis)
        CUBE_SIZE, CUBE_SIZE, // draw the cube with it's width and height
        255, 255, 255, 255 // draw the cube white
    );
}
```

Include the build in a webpage like this:
```html
<!DOCTYPE html>
<html>
    <head>
        <style>
            * { margin: 0px; overflow: hidden; }
            canvas { width: 100vw; height: 100vh; }
        </style>
        <script src="wein2d.js"></script> <!-- include Wein2D.js -->
        <script src="wein2d-wasm.js"></script> <!-- include Wein2D.js-WASM -->
        <script>
            window.onload = function()
            {
                wein2dWASM_init(document.getElementById("canvas"), "/pkg/Wein2D_js_wasm.js")
            }
        </script>
    </head>
    <body>
        <canvas id="canvas"></canvas> <!-- normal canvas element -->
    </body>
</html>
```

# Documentation
Documentation for Wein2D.js-WASM can be found at [https://wein2ddocs.netlify.app](https://wein2ddocs.netlify.app).
