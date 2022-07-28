use crate::statement::Statement;

use std::rc::Rc;

pub fn box_to_rc(b: Box<dyn Statement>) -> Rc<dyn Statement> {
    Rc::from(b)
}
