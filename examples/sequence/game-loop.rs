//! A game loop runs continuously during gameplay.
//!
//! Each turn of the loop, it:
//!
//! - Processes user input without blocking
//! - Updates the game state
//! - Renders the game
//!
//! It tracks the passage of time to control the rate of gameplay.
//!
//! ```bash
//! cargo run --example sequence-game-loop
//! ```

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {}

/// Simple, but the problem with it is you have no control over how fast the game runs.
#[allow(dead_code)]
fn simple_game_loop() {
    fn process_input() {}
    fn update() {}
    fn render() {}

    loop {
        process_input();
        update();
        render();
    }
}

/// Maximum speed of 60FPS.
#[allow(dead_code)]
fn timed_game_loop() {
    const MS_PER_FRAME: u128 = 1000 / 60;

    fn process_input() {}
    fn update() {}
    fn render() {}

    loop {
        let start = Instant::now();
        process_input();
        update();
        render();

        // Sleep to ensure the game doesn't run too quickly, i.e. not more than 60 FPS.
        let elapsed = start.elapsed().as_millis();
        if elapsed < MS_PER_FRAME {
            let delta = MS_PER_FRAME - elapsed;
            thread::sleep(Duration::from_millis(delta as u64));
        }
    }
}

/// Pick a dynamic maximum based on how much time the frame really takes.
#[allow(dead_code)]
fn scaled_game_loop() {
    fn process_input() {}
    fn update(_elapsed: Duration) {}
    fn render() {}

    let mut last_time = Instant::now();

    loop {
        let current = Instant::now();
        let elapsed = current - last_time;

        process_input();
        update(elapsed);
        render();

        last_time = current;
    }
}

/// Update is always done at 60FPS, but reduce rendering as-needed.
#[allow(dead_code)]
fn fixed_update_scaled_render_game_loop() {
    const MS_PER_FRAME: u128 = 1000 / 60;

    fn process_input() {}
    fn update() {}
    fn render(_next_frame: f64) {}

    let mut previous = Instant::now();
    let mut lag = 0.0;

    loop {
        let current = Instant::now();
        let elapsed = current - previous;

        previous = current;
        lag += elapsed.as_millis() as f64;

        process_input();

        while lag >= MS_PER_FRAME as f64 {
            update();
            lag -= MS_PER_FRAME as f64;
        }

        render(lag / MS_PER_FRAME as f64);
    }
}
