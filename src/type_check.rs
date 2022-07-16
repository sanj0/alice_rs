use crate::runtime::AliceVal;
use crate::statement::Statement;

pub const STRING: u32 = 1;
pub const BOOL: u32 = 2;
pub const INT: u32 = 4;
pub const FLOAT: u32 = 8;
pub const ANY: u32 = 0b1111;

pub struct TypeStack(pub Vec<u32>);
pub struct StackPattern(pub Vec<u32>);

pub fn check(statements: &Vec<Box<dyn Statement>>) -> Result<(), TypeCheckError> {
    let mut stack = TypeStack(Vec::new());
    for s in statements {
        s.in_pattern().type_check(&mut stack)?;
        s.custom_type_check(&mut stack)?;
        s.out_pattern().push(&mut stack);
    }
    if stack.0.is_empty() {
        Ok(())
    } else {
        Err(TypeCheckError(format!(
            "{} excess values on the stack!",
            stack.0.len()
        )))
    }
}

pub fn check_interactive(
    stack: &mut TypeStack,
    statements: &Vec<Box<dyn Statement>>,
) -> Result<(), TypeCheckError> {
    for s in statements {
        s.in_pattern().type_check(stack)?;
        s.custom_type_check(stack)?;
        s.out_pattern().push(stack);
    }
    Ok(())
}

impl StackPattern {
    pub fn single(ty: u32) -> Self {
        Self(vec![ty])
    }

    pub fn any(n: usize) -> Self {
        let mut vec = Vec::with_capacity(n);
        for i in 0..n {
            vec.push(ANY)
        }
        Self(vec)
    }

    pub fn type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        for t in &self.0 {
            if let Some(actual) = stack.pop() {
                if actual & t != actual {
                    return Err(TypeCheckError(format!(
                        "wrong type on stack when this executes"
                    ))); // todo descriptive error msg
                }
            } else {
                return Err(TypeCheckError(
                    "too few values on stack when this executes".into(),
                ));
            }
        }
        Ok(())
    }

    pub fn push(&self, stack: &mut TypeStack) {
        for t in &self.0 {
            stack.0.push(*t);
        }
    }
}

impl TypeStack {
    pub fn pop(&mut self) -> Option<u32> {
        self.0.pop()
    }

    pub fn required_size(&self, size: usize) -> Result<(), TypeCheckError> {
        if self.0.len() < size {
            Err(TypeCheckError(
                "too few elements on stack when this executes".into(),
            ))
        } else {
            Ok(())
        }
    }
}

pub struct TypeCheckError(pub String);

impl From<TypeCheckError> for String {
    fn from(err: TypeCheckError) -> String {
        err.0.clone()
    }
}

pub fn type_bit(val: &AliceVal) -> u32 {
    match val {
        AliceVal::String(_) => STRING,
        AliceVal::Bool(_) => BOOL,
        AliceVal::Int(_) => INT,
        AliceVal::Float(_) => FLOAT,
    }
}
