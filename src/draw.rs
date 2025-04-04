use crate::canvas::Canvas;

pub trait Draw {
    fn draw(&mut self, canvas: &mut Canvas, x: isize, y: isize);
}
