//! Simulate a collection of independent objects by telling each to process one frame at a time.
//!
//! In other words, each entity in the game should encapsulate its own behavior.
//!
//! Update methods work best when:
//!
//! - Your game has a number of objects or systems that need to run simultaneously.
//! - Each object’s behavior is mostly independent of the others.
//! - The objects need to be simulated over time.
//!
//! ```bash
//! cargo run --example sequencing-update
//! ```

use std::time::Duration;

fn main() {
    let mut skeleton = Skeleton {
        patrol_left: false,
        x: 0,
        y: 0,
    };

    skeleton.update(Duration::from_millis(500));
    println!("The skeleton's x-coordinate after 500ms: {}", skeleton.x());

    skeleton.update(Duration::from_millis(500));
    println!("The skeleton's x-coordinate after 500ms: {}", skeleton.x());

    skeleton.update(Duration::from_millis(500));
    println!("The skeleton's x-coordinate after 500ms: {}", skeleton.x());
}

trait Entity {
    fn x(&self) -> u64;
    fn y(&self) -> u64;

    fn set_x(&mut self, x: u64);
    fn set_y(&mut self, y: u64);

    fn update(&mut self, elapsed: Duration);
}

struct Skeleton {
    patrol_left: bool,
    x: u64,
    y: u64,
}

impl Entity for Skeleton {
    fn x(&self) -> u64 {
        self.x
    }

    fn y(&self) -> u64 {
        self.y
    }

    fn set_x(&mut self, x: u64) {
        self.x = x;
    }

    fn set_y(&mut self, y: u64) {
        self.y = y;
    }

    fn update(&mut self, elapsed: Duration) {
        let mut x = self.x as i64;
        let elapsed = elapsed.as_secs_f64();
        if self.patrol_left {
            x -= (elapsed * 100.0) as i64;
            if x <= 0 {
                x = 0;
                self.patrol_left = false;
            }
        } else {
            x += (elapsed * 100.0) as i64;
            if x >= 100 {
                x = 100;
                self.patrol_left = true;
            }
        }
        self.x = x as u64;
    }
}
