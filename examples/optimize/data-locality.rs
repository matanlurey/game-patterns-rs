//! Accelerate memory access by arranging data to take advantage of CPU caching.
//!
//! > Like most optimizations, the first guideline for using the Data Locality pattern is when you
//! > have a performance problem. Don’t waste time applying this to some infrequently executed
//! > corner of your codebase. Optimizing code that doesn’t need it just makes your life harder
//! > since the result is almost always more complex and less flexible.
//! >
//! > With this pattern specifically, you’ll also want to be sure your performance problems are
//! > caused by cache misses. If your code is slow for other reasons, this won’t help.
//!
//! One suggested (free) tool is [CacheGrind](http://valgrind.org/docs/manual/cg-manual.html).
//!
//! ```bash
//! cargo run --example optimize-data-locality
//! ```

fn main() {
    let mut system = ParticleSystem::new();

    // Activate a bunch of particles.
    for i in 0..100 {
        system.activate(i);
    }

    // Update the system.
    system.update();

    // Deactivate a bunch of particles.
    for i in (0..100).rev() {
        system.deactivate(i);
    }

    // Update the system.
    system.update();
}

#[derive(Clone, Copy)]
pub struct Particle;

impl Particle {
    pub fn update(&self) {
        println!("Updating particle");
    }
}

pub struct ParticleSystem {
    particles: [Particle; ParticleSystem::MAX_PARTICLES],
    active_len: usize,
}

impl ParticleSystem {
    const MAX_PARTICLES: usize = 100_000;

    pub fn new() -> Self {
        Self {
            particles: [Particle; Self::MAX_PARTICLES],
            active_len: 0,
        }
    }

    pub fn update(&self) {
        for i in 0..self.active_len {
            self.particles[i].update();
        }
    }

    pub fn activate(&mut self, index: usize) {
        assert!(index >= self.active_len, "Already active!");

        // Swap it with the first inactive particle right after the active ones.
        self.particles.swap(index, self.active_len);
        self.active_len += 1;
    }

    pub fn deactivate(&mut self, index: usize) {
        assert!(
            index <= self.active_len,
            "Cannot deactivate inactive particle {} of {}",
            index,
            self.active_len
        );

        self.active_len -= 1;
        self.particles.swap(index, self.active_len);
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new()
    }
}
