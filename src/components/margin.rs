use crate::draw::Draw;
use crate::size::Size;
use crate::Drawable;

use alloc::boxed::Box;

#[derive(Default, Debug)]
pub struct MarginValue {
    pub left: usize,
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
}

pub struct Margin<T: Drawable> {
    pub(crate) element: T,
    pub(crate) margin: MarginValue,
}

impl<T: Drawable> Draw for Margin<T> {
    fn draw(&mut self, canvas: &mut crate::canvas::Canvas, x: usize, y: usize) {
        self.element
            .draw(canvas, x + self.margin.left, y + self.margin.top);
    }
}

impl<T: Drawable> Size for Margin<T> {
    fn set_size(&mut self, _x: usize, _y: usize) {
        unimplemented!()
    }

    fn size(&self) -> (usize, usize) {
        let (w, h) = self.element.size();

        (
            w + self.margin.left + self.margin.right,
            h + self.margin.top + self.margin.bottom,
        )
    }
}

impl<T: 'static + Drawable> Drawable for Margin<T> {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}

impl<T: 'static + Drawable> Margin<T> {
    pub fn with(element: T, margin: MarginValue) -> Self {
        Self { element, margin }
    }

    pub fn left_and_right(element: T, left: usize, right: usize) -> Self {
        Self {
            element,
            margin: MarginValue {
                left,
                top: 0,
                right,
                bottom: 0,
            },
        }
    }

    pub fn top_and_bottom(element: T, top: usize, bottom: usize) -> Self {
        Self {
            element,
            margin: MarginValue {
                left: 0,
                top,
                right: 0,
                bottom,
            },
        }
    }

    pub fn like_args(element: T, left: usize, top: usize, right: usize, bottom: usize) -> Self {
        Self {
            element,
            margin: MarginValue {
                left,
                top,
                right,
                bottom,
            }
        }
    }

    pub fn element_position(&self) -> (usize, usize) {
        (self.margin.left, self.margin.top)
    }

    pub fn element(&self) -> &T {
        &self.element
    }

    pub fn element_mut(&mut self) -> &mut T {
        &mut self.element
    }

	/*
    pub fn passthrough<T: Drawable + 'static>(f: fn(&mut T, usize, usize))
    -> impl FnMut(&mut dyn Drawable, usize, usize) {
        move |element, x, y| {
            let el: &mut Margin = element.as_any_mut().downcast_mut::<Margin>().unwrap();
            
            let pos = el.element_position();

            let el: &mut T = el.element_mut().as_any_mut().downcast_mut::<T>().unwrap();

            (f)(el, x.saturating_sub(pos.0), y.saturating_sub(pos.1));
        }
    }
    */
}
