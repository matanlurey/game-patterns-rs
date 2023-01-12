//! Allow a single entity to span multiple domains without coupling the domains to each other.
//!
//! Components are most commonly found within the core class that defines the entities in a game,
//! but they may be useful in other places as well. This pattern can be put to good use when any of
//! these are true:
//!
//! - You have a class that touches multiple domains which you want to keep decoupled from each
//!   other.
//! - A class is getting massive and hard to work with.
//! - You want to be able to define a variety of objects that share different capabilities, but
//!   using inheritance doesn’t let you pick the parts you want to reuse precisely enough.
//!
//! ```bash
//! cargo run --example decouple-component
//! ```

// cSpell: ignore: Bjorn

fn main() {
    let controller = Controller;
    let world = World;
    let graphics = Graphics;

    // Example of a monolith.
    let mut bjorn = BjornMonolith {
        x: 0.0,
        y: 0.0,
        velocity: 0.0,
        volume: Volume,
        sprite_stand: Sprite,
        sprite_walk_left: Sprite,
        sprite_walk_right: Sprite,
    };

    bjorn.update(&controller, &world, &graphics);

    // Example of components.
    let mut bjorn = Bjorn {
        position: PositionData {
            velocity: 0.0,
            x: 0.0,
            y: 0.0,
        },
        input: InputComponent,
        physics: PhysicsComponent { volume: Volume },
        graphics: GraphicsComponent {
            sprite_stand: Sprite,
            sprite_walk_left: Sprite,
            sprite_walk_right: Sprite,
        },
    };

    bjorn.update(&controller, &world, &graphics);
}

pub struct Controller;

impl Controller {
    pub fn get_joystick_direction(&self) -> Direction {
        Direction::Left
    }
}

pub enum Direction {
    Left,
    Right,
}

pub struct World;

impl World {
    pub fn resolve_collision(&self, _volume: &Volume, x: f32, y: f32, velocity: f32) {
        println!(
            "Resolving collision at ({}, {}) with velocity {}",
            x, y, velocity
        );
    }
}

pub struct Graphics;

impl Graphics {
    pub fn draw(&self, _sprite: &Sprite, x: f32, y: f32) {
        println!("Drawing sprite at ({}, {})", x, y);
    }
}

pub struct Sprite;

pub struct Volume;

/// An example of what an API might look like _before_ using components.
#[allow(dead_code)]
pub struct BjornMonolith {
    x: f32,
    y: f32,
    velocity: f32,
    volume: Volume,
    sprite_stand: Sprite,
    sprite_walk_left: Sprite,
    sprite_walk_right: Sprite,
}

impl BjornMonolith {
    const WALK_ACCELERATION: f32 = 0.1;

    pub fn update(&mut self, controller: &Controller, world: &World, graphics: &Graphics) {
        match controller.get_joystick_direction() {
            Direction::Left => {
                self.velocity -= Self::WALK_ACCELERATION;
            }
            Direction::Right => {
                self.velocity += Self::WALK_ACCELERATION;
            }
        };

        self.x += self.velocity;
        world.resolve_collision(&self.volume, self.x, self.x, self.velocity);

        let sprite = if self.velocity < 0.0 {
            &self.sprite_walk_left
        } else if self.velocity > 0.0 {
            &self.sprite_walk_right
        } else {
            &self.sprite_stand
        };

        graphics.draw(sprite, self.x, self.y);
    }
}

#[allow(dead_code)]
pub struct Bjorn {
    position: PositionData,
    input: InputComponent,
    physics: PhysicsComponent,
    graphics: GraphicsComponent,
}

impl Bjorn {
    pub fn update(&mut self, controller: &Controller, world: &World, graphics: &Graphics) {
        self.input.update(&mut self.position, controller);
        self.physics.update(&mut self.position, world);
        self.graphics.update(&mut self.position, graphics);
    }
}

// This could be split into Move and Position, but ... this is an example.
pub struct PositionData {
    pub velocity: f32,
    pub x: f32,
    pub y: f32,
}

pub struct InputComponent;

impl InputComponent {
    const WALK_ACCELERATION: f32 = 0.1;

    pub fn update(&self, target: &mut PositionData, controller: &Controller) {
        match controller.get_joystick_direction() {
            Direction::Left => target.velocity -= Self::WALK_ACCELERATION,
            Direction::Right => target.velocity += Self::WALK_ACCELERATION,
        }
    }
}

pub struct PhysicsComponent {
    volume: Volume,
}

impl PhysicsComponent {
    pub fn update(&self, target: &mut PositionData, world: &World) {
        target.x += target.velocity;
        world.resolve_collision(&self.volume, target.x, target.y, target.velocity)
    }
}

pub struct GraphicsComponent {
    sprite_stand: Sprite,
    sprite_walk_left: Sprite,
    sprite_walk_right: Sprite,
}

impl GraphicsComponent {
    pub fn update(&self, target: &mut PositionData, graphics: &Graphics) {
        let sprite = if target.velocity < 0.0 {
            &self.sprite_walk_left
        } else if target.velocity > 0.0 {
            &self.sprite_walk_right
        } else {
            &self.sprite_stand
        };

        graphics.draw(sprite, target.x, target.y);
    }
}
