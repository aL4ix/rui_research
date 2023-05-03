use std::rc::Rc;

use glyph_brush::ab_glyph::FontArc;

use crate::general::Color;
use crate::widgets::Primitive;
use crate::widgets::primitives::{Shape, Text};

pub enum PrimitiveOrTwoRc<T1, T2> {
    Ref1(Rc<T1>),
    Ref2(Rc<T2>),
    Pri(Box<dyn Primitive>)
}
pub fn theme_for_button(text: &str, font_size: f32, font: FontArc, color_t: Color, color_s: Color)
                        -> (Vec<PrimitiveOrOneRc<Text>>, Rc<Text>) {
    let text = Text::new(0, text, font_size, font, color_t);
    let size = text.size().clone();
    let shape = Shape::new_square(0, size, 0, color_s);
    let rc = Rc::new(text);
    (vec![PrimitiveOrOneRc::Pri(Box::new(shape)), PrimitiveOrOneRc::Rc(rc.clone())], rc)
}
pub enum PrimitiveOrOneRc<T> {
    Rc(Rc<T>),
    Pri(Box<dyn Primitive>)
}
pub fn theme_for_textbox(text: &str, font: FontArc, font_size: f32, color: Color)
                         -> (Vec<PrimitiveOrOneRc<Text>>, Rc<Text>) {
    let text_pri = Text::new(1, text, font_size, font, color);
    let rc = Rc::new(text_pri);
    (vec![PrimitiveOrOneRc::Rc(rc.clone())], rc)
}