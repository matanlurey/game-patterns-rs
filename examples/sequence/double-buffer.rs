//! Double Buffer.
//!
//! Our program renders the pixels one at a time, but we want the display driver to see them all.
//!
//! ```bash
//! cargo run --example sequence-double-buffer
//! ```

use std::mem;

fn main() {
    let mut face = Scene::<char>::new(6, 6);

    face.draw(1, 1, '▓');
    face.draw(4, 1, '▓');
    face.draw(1, 3, '▓');
    face.draw(2, 4, '▓');
    face.draw(3, 4, '▓');
    face.draw(4, 3, '▓');

    fn print_scene(scene: &Scene<char>) {
        for row in scene.pixels() {
            for col in row {
                let col = {
                    if col == &Default::default() {
                        ' '
                    } else {
                        *col
                    }
                };
                print!("{}", col);
            }
            println!();
        }
    }

    // Noop.
    print_scene(&face);

    // Actually draws the face.
    face.swap();
    print_scene(&face);

    // Back to a no-op (empty face).
    face.swap();
    print_scene(&face);
}

pub struct FrameBuffer<T> {
    pixels: Vec<T>,
    width: usize,
}

impl<T> FrameBuffer<T> {
    /// Returns the width of the buffer.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the buffer.
    pub fn height(&self) -> usize {
        self.pixels.len() / self.width
    }
}

impl<T> FrameBuffer<T>
where
    T: Clone + Default,
{
    /// Creates a new frame buffer with the given width and height.
    ///
    /// # Panics
    ///
    /// If width or height is zero.
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0);
        assert!(height > 0);
        Self {
            pixels: vec![T::default(); width * height],
            width,
        }
    }

    /// Draws (writes to a cell) of the buffer.
    pub fn draw(&mut self, x: usize, y: usize, pixel: T) {
        self.pixels[y * self.width + x] = pixel;
    }

    /// Clears the buffer.
    pub fn clear(&mut self) {
        for pixel in &mut self.pixels {
            *pixel = T::default();
        }
    }

    /// Returns the pixels of the buffer as vector of row slices.
    pub fn pixels(&self) -> Vec<&[T]> {
        self.pixels.chunks(self.width).collect()
    }
}

pub struct Scene<T> {
    display: FrameBuffer<T>,
    drawing: FrameBuffer<T>,
}

impl<T> Scene<T> {
    /// Returns the width of the scene.
    pub fn width(&self) -> usize {
        self.display.width()
    }

    /// Returns the height of the scene.
    pub fn height(&self) -> usize {
        self.display.height()
    }
}

impl<T> Scene<T>
where
    T: Clone + Default,
{
    /// Creates a new scene with the given width and height.
    ///
    /// # Panics
    ///
    /// If width or height is zero.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            display: FrameBuffer::new(width, height),
            drawing: FrameBuffer::new(width, height),
        }
    }

    /// Draws (writes to a cell) of the scene.
    pub fn draw(&mut self, x: usize, y: usize, pixel: T) {
        self.drawing.draw(x, y, pixel);
    }

    /// Clears the scene.
    pub fn clear(&mut self) {
        self.drawing.clear();
    }

    /// Returns the pixels of the scene as vector of row slices.
    pub fn pixels(&self) -> Vec<&[T]> {
        self.display.pixels()
    }

    /// Swaps the display and drawing buffers.
    pub fn swap(&mut self) {
        mem::swap(&mut self.display, &mut self.drawing);
    }
}
