//! Give behavior the flexibility of data by encoding it as instructions for a virtual machine.
//!
//! ```bash
//! cargo run --example behavior-bytecode
//! ```

use std::{collections::VecDeque, vec::IntoIter};

fn main() {
    // LITERAL 0    [0]            # Wizard index
    // LITERAL 0    [0, 0]         # Wizard index
    // GET_HEALTH   [0, 45]        # getHealth()
    // LITERAL 0    [0, 45, 0]     # Wizard index
    // GET_AGILITY  [0, 45, 7]     # getAgility()
    // LITERAL 0    [0, 45, 7, 0]  # Wizard index
    // GET_WISDOM   [0, 45, 7, 11] # getWisdom()
    // ADD          [0, 45, 18]    # Add agility and wisdom
    // LITERAL 2    [0, 45, 18, 2] # Divisor
    // DIVIDE       [0, 45, 9]     # Average agility and wisdom
    // ADD          [0, 54]        # Add average to current health
    // SET_HEALTH   []             # Set health to result
}

#[allow(dead_code)]
pub struct VM {
    stack: VecDeque<u64>,
    bytes: IntoIter<u64>,
}

#[allow(dead_code)]
impl VM {
    const INST_LITERAL: u64 = 100_000_000;
    const INST_SET_HEALTH: u64 = 100_000_001;
    const INST_SET_WISDOM: u64 = 100_000_002;
    const INST_SET_AGILITY: u64 = 100_000_003;
    const INST_PLAY_SOUND: u64 = 100_000_004;
    const INST_SPAWN_PARTICLES: u64 = 100_000_005;

    pub fn new(bytes: Vec<u64>) -> Self {
        VM {
            stack: Default::default(),
            bytes: bytes.into_iter(),
        }
    }

    pub fn push(&mut self, value: u64) {
        self.stack.push_front(value)
    }

    pub fn pop(&mut self) -> Option<u64> {
        self.stack.pop_front()
    }

    fn execute(&mut self, value: u64) {
        match value {
            VM::INST_LITERAL => {
                let next = self.bytes.next().unwrap();
                self.push(next);
            }

            VM::INST_SET_HEALTH | VM::INST_SET_WISDOM | VM::INST_SET_AGILITY => {
                let (amount, wizard) = (self.pop().unwrap(), self.pop().unwrap());
                let name = match value {
                    VM::INST_SET_HEALTH => "Health",
                    VM::INST_SET_WISDOM => "Wisdom",
                    VM::INST_SET_AGILITY => "Agility",
                    _ => unreachable!(),
                };
                println!("set{}({}. {})", name, amount, wizard);
            }

            VM::INST_PLAY_SOUND => {
                let sound = self.pop().unwrap();
                println!("playSound({})", sound);
            }

            VM::INST_SPAWN_PARTICLES => {
                let texture = self.pop().unwrap();
                println!("spawnParticles({})", texture);
            }

            _ => {
                panic!()
            }
        }
    }
}
