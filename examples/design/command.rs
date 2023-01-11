//! A command is a request to perform an action.
//!
//! This example uses a [`Command`] trait combined with `dyn Command` to reference implementations,
//! for example in a single vector or other form of data structure. It keeps strongly-typed objects
//! with implementation locality, and, in theory, the ability to revert.
//!
//! ```bash
//! cargo run --example design-command
//! ```

use std::io;

fn main() {
    // Read in any command (i.e. from an AI system, network, direct from client UI, etc).
    let command = input();

    // Execute the command.
    let mut actor = GameActor { x: 0, y: 0 };
    command.execute(&actor);

    // Execute a command that acts on itself.
    let mut command = MoveUnitCommand {
        unit: &mut actor,
        x: 10,
        y: 20,
    };

    command.run();
    command.undo();
    println!("Run + Undo: {:?}", &actor);
}

/// A command pattern that takes in what is being acted on.
trait UnaryCommand {
    fn execute(&self, actor: &GameActor);
}

/// Read from stdin and either jump (j) or fire (f).
fn input() -> Box<dyn UnaryCommand> {
    loop {
        // Read next line.
        println!("Enter j to jump or f to fire:");
        let line = io::stdin().lines().next().unwrap();

        // Parse input.
        match line.unwrap().as_str() {
            "j" => break Box::new(JumpCommand),
            "f" => break Box::new(FireCommand),
            _ => {
                eprintln!("Invalid input, try again.");
            }
        }
    }
}

#[derive(Debug)]
struct GameActor {
    x: i32,
    y: i32,
}

impl GameActor {
    fn jump(&self) {
        println!("Jumping!");
    }

    fn fire(&self) {
        println!("Firing!");
    }
}

struct JumpCommand;

impl UnaryCommand for JumpCommand {
    fn execute(&self, actor: &GameActor) {
        actor.jump();
    }
}

struct FireCommand;

impl UnaryCommand for FireCommand {
    fn execute(&self, actor: &GameActor) {
        actor.fire();
    }
}

/// A command pattern that acts on itself.
///
/// Because they encapsulate the target, they are reverse-able.
trait Command {
    fn run(&mut self);
    fn undo(&mut self);
}

struct MoveUnitCommand<'a> {
    unit: &'a mut GameActor,
    x: i32,
    y: i32,
}

impl<'a> Command for MoveUnitCommand<'a> {
    fn run(&mut self) {
        self.unit.x += self.x;
        self.unit.y += self.y;
    }

    // Another way to support this could be to store a (before_x and before_y) internally.
    // When run is called, assign, and for undo restore.
    fn undo(&mut self) {
        self.unit.x -= self.x;
        self.unit.y -= self.y;
    }
}
