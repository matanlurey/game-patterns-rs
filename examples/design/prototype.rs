//! Like Bob, I don't find the prototype pattern super useful aside from in data.
//!
//! Rust already provides [`Clone`], which combined with structs is enough to express prototypes.
//!
//! So instead, this is an example of storing and retrieving data from an external source (in this
//! case TOML, but it could be anything, JSON, XML if you hate yourself, etc).
//!
//! ```bash
//! cargo run --example design-prototype
//! ```
//!
//! Some things I would have done different for a more production system:
//!
//! - Deserialize eagerly (if the TOML is malformed find out early in test time)
//! - Deserialize gracefully (i.e. using try patterns instead of panics)
//! - Make prototypes able to have prototypes themselves, as long as its not a circular loop

use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use serde_derive::Deserialize;

fn main() {
    // Read "prototype.toml".
    let data = include_str!("prototype.toml");

    // Parse data as TOML map.
    let data: HashMap<String, MonsterConfig> = toml::from_str(data).unwrap();

    // Output what was loaded.
    println!(
        "Loaded {} monsters with the following keys: {}",
        data.len(),
        data.keys().cloned().collect::<Vec<_>>().join(", ")
    );

    // Load each entry as a MonsterConfig.
    let monsters = data.values().map(|config| {
        let mut prototypes = vec![config.clone()];

        // Load the prototype data.
        for prototype in &config.prototype {
            let prototype = data.get(prototype).unwrap();
            prototypes.push(prototype.clone());
        }

        // Create the finalized monster.
        Monster::from(prototypes.as_slice())
    });

    // Print out the monsters.
    for monster in monsters {
        println!("{monster}");
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct MonsterConfig {
    pub name: Option<String>,
    pub min_health: Option<u8>,
    pub max_health: Option<u8>,

    #[serde(default)]
    pub prototype: Vec<String>,

    #[serde(default)]
    pub resist: Vec<String>,

    #[serde(default)]
    pub weakness: Vec<String>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Monster {
    name: String,
    min_health: u8,
    max_health: u8,
    resist: HashSet<String>,
    weakness: HashSet<String>,
}

impl Display for Monster {
    // Just delegate to Debug.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl From<&[MonsterConfig]> for Monster {
    fn from(value: &[MonsterConfig]) -> Self {
        let mut iter = value.iter();
        let mut build = iter.next().unwrap().clone();

        // Iterate over the remaining and override/merge.
        for merge in iter {
            if let Some(name) = merge.name.clone() {
                build.name = Some(name);
            }

            if let Some(min_health) = merge.min_health {
                build.min_health = Some(min_health);
            }

            if let Some(max_health) = merge.max_health {
                build.max_health = Some(max_health);
            }

            build.resist.extend(merge.resist.clone());
            build.weakness.extend(merge.weakness.clone());
        }

        // Sort the resist and weakness for consistency.
        build.resist.sort();
        build.weakness.sort();

        Self {
            name: build.name.expect("Name is required"),
            min_health: build.min_health.expect("Min health is required"),
            max_health: build.max_health.expect("Max health is required"),
            resist: build.resist.into_iter().collect(),
            weakness: build.weakness.into_iter().collect(),
        }
    }
}
