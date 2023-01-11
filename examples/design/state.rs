//! State pattern.
//!
//! Finite state machines are useful when:
//!
//! - You have an entity whose behavior changes based on some internal state.
//! - That stage can be rigidly divided into one of a relatively small number of distinct options.
//! - The entity responds to a series of inputs or events over time.
//!
//! ```bash
//! cargo run --example design-state
//! ```
//!
//! See also "concurrent state machines" (below) and "hierarchical state machines" (below).

// cSpell: ignore: Legolas pushdown

fn main() {
    let mut hero = Hero::new("Legolas".to_string());
    hero.notch();
    hero.fire();

    // Can't fire without notching
    hero.fire();
    hero.notch();

    // Can't notch without firing
    hero.notch();
    hero.fire();
}

pub struct Hero {
    state: Option<Box<dyn State>>,
    name: String,
}

impl Hero {
    pub fn new(name: String) -> Self {
        Self {
            state: Some(Box::new(StandingState)),
            name,
        }
    }

    pub fn fire(&mut self) {
        self.state = self.state.take().map(|state| state.fire(self));
    }

    pub fn notch(&mut self) {
        self.state = self.state.take().map(|state| state.notch(self));
    }
}

pub trait State {
    fn fire(self: Box<Self>, hero: &mut Hero) -> Box<dyn State>;

    fn notch(self: Box<Self>, hero: &mut Hero) -> Box<dyn State>;
}

struct StandingState;

impl State for StandingState {
    fn fire(self: Box<Self>, hero: &mut Hero) -> Box<dyn State> {
        println!("{} failed to fire (NO_ARROW_NOTCHED)", hero.name);
        self
    }

    fn notch(self: Box<Self>, hero: &mut Hero) -> Box<dyn State> {
        println!("{} Notched...", hero.name);
        Box::new(NotchedState)
    }
}

struct NotchedState;

impl State for NotchedState {
    fn fire(self: Box<Self>, hero: &mut Hero) -> Box<dyn State> {
        println!("{} Fired!", hero.name);
        Box::new(StandingState)
    }

    fn notch(self: Box<Self>, hero: &mut Hero) -> Box<dyn State> {
        println!("{} failed to notch (ALREADY_NOTCHED)", hero.name);
        self
    }
}

mod concurrent_state_machines {
    //! With a traditional state machine, adding an ability, say, to jump, complicates things.
    //!
    //! Above, you would need a `JumpingAndNotchedState` in order to fire in the air.
    //!
    //! One way to get around that different (and concurrently running) state machines.

    use super::*;

    /// Similar to a hero, but has one state for each.
    #[allow(dead_code)]
    pub struct BunnyHero {
        name: String,
        state: Option<Box<dyn BunnyState>>,
        holster: Option<Box<dyn BunnyState>>,
    }

    pub trait BunnyState: State {
        fn jump(self: Box<Self>, bunny: &mut BunnyHero) -> Box<dyn State>;
    }
}

mod hierarchical_state_machines {
    //! What if you have a bunch of similar states, i.e. standing, walking, running, sliding?
    //!
    //! Inheritance to the rescue (?)

    use super::*;

    /// Similar to a state, but for all states that occur on the ground.
    pub trait GroundedState: State {}
}

mod pushdown_automata {
    //! A _stack_ of states.
    //!
    //! States have no concept of _history_, or the ability to go back to the previous state.
    //!
    //! What if the hero fires an arrow (changing the sprite), and then goes back to just standing?

    use std::collections::VecDeque;

    use super::*;

    #[allow(dead_code)]
    pub fn example_of_stack() {
        let mut states = VecDeque::<Box<dyn State>>::new();

        states.push_back(Box::new(StandingState));
        states.push_back(Box::new(NotchedState));

        // Now after say, firing (imagine that instead of Notched), we want to go back to Standing.
        // Easy, we just go back to the previous state.
    }
}
