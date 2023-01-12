//! Allow the flexible creation of new “classes” by creating a single class, each instance of which
//! represents a different type of object.
//!
//! This pattern is useful anytime you need to define a variety of different “kinds” of things, but
//! baking the kinds into your language’s type system is too rigid. In particular, it’s useful when
//! either of these is true:
//!
//! - You don’t know what types you will need up front. (For example, what if our game needed to
//! support downloading content that contained new breeds of monsters?)
//!
//! - You want to be able to modify or add new types without having to recompile or change code.
//!
//! ```bash
//! cargo run --example behavior-intent
//! ```

use std::{borrow::Borrow, rc::Rc};

fn main() {
    let goblin = Rc::new(Breed::new(10, "bite"));
    let monster = Monster::from(goblin.borrow());
    println!(
        "A monster with {} HP attacks with {}",
        monster.current_health,
        monster.attack()
    );
}

pub struct Breed<'a> {
    health: Option<u8>,
    attack: Option<String>,
    parent: Option<&'a Breed<'a>>,
}

impl Breed<'_> {
    pub fn new(health: u8, attack: impl Into<String>) -> Self {
        Breed {
            health: Some(health),
            attack: Some(attack.into()),
            parent: None,
        }
    }

    // TODO: Add a try_inherit method that fails if the parent(s) don't have the required fields.

    pub fn get_attack(&self) -> &str {
        let attack = &self.attack;
        match attack {
            Some(attack) => attack,
            None => self.parent.unwrap().get_attack(),
        }
    }

    pub fn get_health(&self) -> u8 {
        let health = self.health;
        match health {
            Some(health) => health,
            None => self.parent.unwrap().get_health(),
        }
    }
}

pub struct Monster<'a> {
    #[allow(dead_code)]
    current_health: u8,
    breed: &'a Breed<'a>,
}

impl Monster<'_> {
    pub fn attack(&self) -> &str {
        self.breed.get_attack()
    }
}

impl<'a> From<&'a Breed<'a>> for Monster<'a> {
    fn from(value: &'a Breed) -> Self {
        Monster {
            current_health: value.get_health(),
            breed: value,
        }
    }
}
