use core::f32;

use crate::{general::Vector2D, widgets::{Direction, Widget}};

#[derive(Debug)]
pub struct NextPositionCalculator {
    next_x: f32,
    next_y: f32,
    size_x: f32,
    size_y: f32,
    root_x: f32,
    root_y: f32,
    direction: Direction,
    limit_x: f32,
    limit_y: f32,
}

impl NextPositionCalculator {
    pub fn new(direction: Direction) -> Self {
        Self {
            next_x: Default::default(),
            next_y: Default::default(),
            size_x: Default::default(),
            size_y: Default::default(),
            root_x: Default::default(),
            root_y: Default::default(),
            direction: direction,
            limit_x: f32::INFINITY,
            limit_y: f32::INFINITY,
        }
    }
    pub fn next(&mut self, widget: &mut Box<dyn Widget>) -> Vector2D<f32> {
        let position = (self.root_x + self.next_x, self.root_y + self.next_y).into();
        let widget_size = widget.size();
        if self.direction == Direction::Horizontal {
            self.next_x += widget_size.x();
            if self.next_x > self.limit_x {
                self.next_x = 0.0;
                self.next_y += self.size_y;
            }
            if self.next_x > self.size_x {
                self.size_x = self.next_x;
            }
            if widget_size.y() > self.size_y {
                self.size_y = widget_size.y();
            }
        } else {
            self.next_y += widget_size.y();
            if self.next_y > self.limit_y {
                self.next_y = 0.0;
                self.next_x += self.size_x;
            }
            if self.next_y > self.size_y {
                self.size_y = self.next_y;
            }
            if widget_size.x() > self.size_x {
                self.size_x = widget_size.x();
            }
        }
        return position;
    }
    pub fn size(&self) -> Vector2D<f32> {
        Vector2D::new(self.size_x, self.size_y)
    }
    pub fn reset(&mut self) {
        self.next_x = Default::default();
        self.next_y = Default::default();
        self.size_x = Default::default();
        self.size_y = Default::default();
    }
    pub fn set_root_position(&mut self, position: &Vector2D<f32>) {
        self.root_x = position.x();
        self.root_y = position.y();
    }
}
