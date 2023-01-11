//! The observer pattern is a software design pattern in which an object, called the subject,
//! maintains a list of its dependents, called observers, and notifies them automatically of any
//! state changes, usually by calling one of their methods.
//!
//! ```bash
//! cargo run --example design-observer
//! ```

// cSpell: ignore: Aragorn Legolas

fn main() {
    #[derive(Clone)]
    pub enum Event {
        Fired,
        Jumped,
    }

    pub struct Hero {
        name: String,
    }

    let aragorn = Hero {
        name: "Aragorn".to_string(),
    };

    let legolas = Hero {
        name: "Legolas".to_string(),
    };

    let observer = |event: Event, source: &Hero| match event {
        Event::Fired => println!("{} fired!", source.name),
        Event::Jumped => println!("{} jumped!", source.name),
    };

    let mut subject = Subject::<Event, &Hero>::new();

    // Will print to console.
    subject.attach(observer);
    subject.notify(Event::Jumped, &aragorn);
    subject.notify(Event::Fired, &legolas);

    // Will do nothing, since removed.
    subject.detach(observer);
    subject.notify(Event::Jumped, &aragorn);
    subject.notify(Event::Fired, &legolas);
}

/// An example. Could also have been a trait called Observer with an associated method "notify":
///
/// ```no_run
/// trait Observer<E, S> {
///     fn notify(&self, event: E, source: S);
/// }
/// # struct Event;
/// # struct Source;
/// struct MyObserver;
///
/// impl Observer<Event, &Source> for MyObserver {
///     fn notify(&self, event: Event, source: &Source) {
///         // Do something with the event and source.
///     }
/// }
/// ```
pub type Observer<E, S> = fn(event: E, source: S);

pub struct Subject<E, S>
where
    E: Clone,
    S: Clone,
{
    observers: Vec<Observer<E, S>>,
}

impl<E, S> Subject<E, S>
where
    E: Clone,
    S: Clone,
{
    pub fn new() -> Self {
        Subject {
            observers: Vec::new(),
        }
    }

    pub fn attach(&mut self, observer: Observer<E, S>) {
        self.observers.push(observer);
    }

    pub fn detach(&mut self, observer: Observer<E, S>) {
        self.observers.retain(|o| *o != observer);
    }

    pub fn notify(&self, event: E, source: S) {
        for observer in &self.observers {
            observer(event.clone(), source.clone());
        }
    }
}

impl<E, S> Default for Subject<E, S>
where
    E: Clone,
    S: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
