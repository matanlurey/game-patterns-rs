//! Provide a global point of access to a service without coupling users to the concrete class that
//! implements it.
//!
//! Systems represent facilities that are fundamentally singular in nature. Your game probably only
//! has one audio device or display system that it can talk to. It is an ambient property of the
//! environment, so plumbing it through ten layers of methods just so one deeply nested call can get
//! to it is adding needless complexity to your code.
//!
//! ```bash
//! cargo run --example decouple-service-locator
//! ```

use std::cell::RefCell;

thread_local! {
    // ^^^^^^
    // thread_local gives us "static-like" access.
    //
    //                Interior mutability, checked at runtime.
    //                vvvvvvv
    pub static AUDIO: RefCell<Box<dyn Audio>> = RefCell::new(Box::new(ConsoleAudio));
    //                        ^^^^^^^^^^^^^^^
    //                        Virtual dispatch.
    //
    // This could be combined further with #[cfg(feature = "...")] tags in order to have different
    // implementations wired up at compile-time (e.g. a Debug-variant, a Null-variant for tests).
}

fn main() {
    AUDIO.with(|cell| cell.borrow_mut().play_sound());
}

pub trait Audio {
    fn play_sound(&mut self);
}

pub struct ConsoleAudio;

impl Audio for ConsoleAudio {
    fn play_sound(&mut self) {
        println!("Playing sound");
    }
}

pub struct NullAudio;

impl Audio for NullAudio {
    fn play_sound(&mut self) {
        // Intentionally left blank.
    }
}
