use crate::canvas::Canvas;

pub trait Draw {
    fn draw(&self, canvas: &mut Canvas, x: usize, y: usize);
}
