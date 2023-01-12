//! Define behavior in a subclass using a set of operations provided by its base class.
//!
//! Subclass Sandbox is a good fit when:
//! - You have a base class with a number of derived classes.
//! - The base class is able to provide all of the operations that a derived class may need to
//!   perform.
//! - There is behavioral overlap in the subclasses and you want to make it easier to share code
//!   between them.
//! - You want to minimize coupling between those derived classes and the rest of the program.
//!
//! ```bash
//! cargo run --example behavior-sandbox
//! ```

fn main() {
    // This pattern is not a great fit for Rust, which does not have traditional sub-typing.
    //
    // We could use a trait, but the trait methods would be public, defeating some of the benefit.
}
