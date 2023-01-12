//! Decouple when a message or event is sent from when it is processed.
//!
//! If you only want to decouple who receives a message from its sender, patterns like Observer and
//! Command will take care of this with less complexity. You only need a queue when you want to
//! decouple something in time.
//!
//! ```bash
//! cargo run --example decouple-event-queue
//! ```

fn main() {
    let mut audio = SimpleAudioQueue::<16>::new();

    audio.play(SoundId, 0.1);
    audio.play(SoundId, 0.2);
    audio.play(SoundId, 0.3);

    audio.update();
}

#[derive(Clone, Copy)]
pub struct SoundId;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct PlayMessage {
    id: SoundId,
    volume: f32,
}

// This works fine, but it does presume we can process every sound request in 1 call to update().
pub struct SimpleAudioQueue<const MAX: usize> {
    buffer: [Option<PlayMessage>; MAX],
    pending: usize,
}

impl<const MAX: usize> SimpleAudioQueue<MAX> {
    pub fn new() -> Self {
        Self {
            buffer: [None; MAX],
            pending: 0,
        }
    }

    pub fn play(&mut self, id: SoundId, volume: f32) {
        assert!(self.pending < MAX);
        self.buffer[self.pending] = Some(PlayMessage { id, volume });
        self.pending += 1;
    }

    pub fn update(&mut self) {
        // In practice, we'd find sound channels, load sounds, and play them here.
        self.pending = 0;
    }
}

impl<const MAX: usize> Default for SimpleAudioQueue<MAX> {
    fn default() -> Self {
        Self::new()
    }
}
