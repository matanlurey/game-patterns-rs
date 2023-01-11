//! A lightweight object that can be referenced many times.
//!
//! "If you find yourself creating an enum and doing lots of switches on it, consider this pattern."
//!
//! ```bash
//! cargo run --example design-flyweight
//! ```

use std::fmt::Display;

use rand::Rng;

fn main() {
    // Example, using references (could be local, static, or reference counted).
    let mut grid = Grid::<&TerrainData>::new(14, 6);
    let mut rng = rand::thread_rng();

    // Terrain types
    let (grass, hill, river) = (
        TerrainData { display_as: '.' },
        TerrainData { display_as: '^' },
        TerrainData { display_as: '~' },
    );

    // Fill the ground with grass.
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            // Sprinkle in some hills.
            if rng.gen_ratio(1, 10) {
                grid.set(x, y, &hill);
            } else {
                grid.set(x, y, &grass);
            }
        }
    }

    // Lay a river
    let x = rng.gen_range(0..grid.width());
    for y in 0..grid.height() {
        grid.set(x, y, &river);
    }

    // Print the grid.
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            print!("{}", grid.get(x, y));
        }
        println!();
    }
}

struct Grid<T> {
    cells: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    /// Create a new grid with the given width and height.
    ///
    /// # Panics
    ///
    /// If width or height is zero.
    fn new(width: usize, height: usize) -> Self
    where
        T: Clone + Default,
    {
        assert!(width > 0);
        assert!(height > 0);
        Grid {
            cells: vec![T::default(); width * height],
            width,
        }
    }

    /// Returns the cell at the given coordinates.
    fn get(&self, x: usize, y: usize) -> &T {
        &self.cells[y * self.width + x]
    }

    /// Sets the cell at the given coordinates.
    fn set(&mut self, x: usize, y: usize, value: T) {
        self.cells[y * self.width + x] = value;
    }

    /// Returns the width of the grid.
    fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid.
    fn height(&self) -> usize {
        self.cells.len() / self.width
    }
}

struct TerrainData {
    display_as: char,
}

impl TerrainData {
    const EMPTY: TerrainData = TerrainData { display_as: ' ' };
}

impl Default for &TerrainData {
    fn default() -> Self {
        &TerrainData::EMPTY
    }
}

impl Display for TerrainData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_as)
    }
}
