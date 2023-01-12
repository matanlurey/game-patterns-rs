<!--
  cSpell: ignore: Nystrom
-->

# [Game Programming Patterns](https://gameprogrammingpatterns.com/) in Rust

This repository is an annotated set of examples of the patterns defined in Bob
Nystrom's [Game Programming Patterns](https://gameprogrammingpatterns.com/),
implemented in Rust.

> [For me][book-intro], _good_ design means that when I make a change, it’s as
> if the entire program was crafted in anticipation of it. I can solve a task
> with just a few choice function calls that slot in perfectly, leaving not the
> slightest ripple on the placid surface of the code.

[book-intro]: https://gameprogrammingpatterns.com/architecture-performance-and-games.html#what-is-*good*-software-architecture

## Behavioral

- [Bytecode](examples/behavior/bytecode.rs)
- [Intent](examples/behavior/intent.rs)
- [Sandbox](examples/behavior/sandbox.rs)

## Decoupling

- [Component](examples/decouple/component.rs)
- Event Queue
- Service Locator

## Design

- [Command](examples/design/command.rs)
- [Flyweight](examples/design/flyweight.rs)
- [Observer](examples/design/observer.rs)
- [Prototype](examples/design/prototype.rs)
- [Singleton](examples/design/singleton.md)
- [State](examples/design/state.rs)

## Sequencing

- [Double Buffer](examples/sequence/double-buffer.rs)
- [Game Loop](examples/sequence/game-loop.rs)
- [Update](examples/sequence/update.rs)
