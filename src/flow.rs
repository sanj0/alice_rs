use std::rc::Rc;
use crate::statement::Statement;

// flow control.
// 1. if
// 2. if-else
// 3. match
// 4. for
// 5. while

/// 1. if
#[derive(Clone)]
pub struct IfContainer {
    pub body: Vec<Rc<dyn Statement>>,
}

// 2. if-else
pub struct IfElseContainer {
    pub if_body: Vec<Rc<dyn Statement>>,
    pub else_body: Vec<Rc<dyn Statement>>,
}
