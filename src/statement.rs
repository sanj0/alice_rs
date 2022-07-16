use crate::runtime::{AliceStack, AliceTable, AliceVal};
use crate::type_check::*;

pub trait Statement {
    fn in_pattern(&self) -> StackPattern {
        StackPattern(Vec::new())
    }
    /// for stack operator type check
    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        Ok(())
    }
    fn out_pattern(&self) -> StackPattern {
        StackPattern(Vec::new())
    }
    fn execute(&self, stack: &mut AliceStack, table: &mut AliceTable) -> Result<(), String>;
}

/// clones a literal onto the stack
pub struct PushStatement(pub AliceVal);

/// pops a value and prints it
pub struct PrintlnStatement;

/// pops a value and prints it; flushes stdio
pub struct PrintStatement;

/// prints the full stack for debug purposes
pub struct PrintStackStatement;

/// exits the program with the exit code at the stack head
pub struct ExitStatement;

/// exits the program with an ok (0) exit code
pub struct OkExitStatement;

/// drop the head of the stack
pub struct DropStatement;

/// swaps the two top most stack elements
pub struct SwapStatement;

/// duplicates the stacks head on top of itself
pub struct DupStatement;

/// copies the second element on the stack on top
/// a b over -> a b a'
pub struct OverStatement;

/// rotates the third stack item on top
/// a b c rot -> b c a
pub struct RotStatement;

impl Statement for PushStatement {
    fn out_pattern(&self) -> StackPattern {
        StackPattern::single(type_bit(&self.0))
    }

    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        stack.push(self.0.clone());
        Ok(())
    }
}

impl Statement for PrintlnStatement {
    fn in_pattern(&self) -> StackPattern {
        StackPattern::any(1)
    }

    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        let val = stack.pop()?;
        println!("{val}");
        Ok(())
    }
}

impl Statement for PrintStatement {
    fn in_pattern(&self) -> StackPattern {
        StackPattern::any(1)
    }

    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        let val = stack.pop()?;
        print!("{val}");
        Ok(())
    }
}

impl Statement for PrintStackStatement {
    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        for val in &stack.stack {
            println!("{val}");
        }
        Ok(())
    }
}

impl Statement for ExitStatement {
    fn in_pattern(&self) -> StackPattern {
        StackPattern::single(INT)
    }
    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        // todo: redundant with type checker
        match stack.pop_typed(&AliceVal::int()) {
            Ok(Some(val)) => std::process::exit(val.unchecked_int() as i32),
            Ok(None) => panic!("implement a type checker!"),
            Err(_) => panic!("implement a type checker!"),
        }
    }
}

impl Statement for OkExitStatement {
    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        std::process::exit(0);
    }
}

impl Statement for DropStatement {
    fn in_pattern(&self) -> StackPattern {
        StackPattern::any(1)
    }

    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        stack.pop();
        // type checker promises that stack operations can never fail
        Ok(())
    }
}

impl Statement for SwapStatement {
    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        // need to dynamically generate type check patterns
        stack.required_size(2)?;
        let second = stack.0.remove(stack.0.len() - 2);
        stack.0.push(second);
        Ok(())
    }

    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        let second = stack.remove(1);
        stack.push(second);
        Ok(())
    }
}

impl Statement for DupStatement {
    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        stack.required_size(1)?;
        stack.0.push(*stack.0.get(stack.0.len() - 1).unwrap()); // unwrapping safe due to previous check
        Ok(())
    }

    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        stack.push(stack.get(0).unwrap().clone()); // unuwrapping safe due to type checker
        Ok(())
    }
}

impl Statement for OverStatement {
    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        stack.required_size(2)?;
        stack.0.push(*stack.0.get(stack.0.len() - 2).unwrap()); // unwrapping safe due to previoud check
        Ok(())
    }

    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        stack.push(stack.get(1).unwrap().clone()); // unuwrapping safe due to type checker
        Ok(())
    }
}

impl Statement for RotStatement {
    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        stack.required_size(3)?;
        let third = stack.0.remove(stack.0.len() - 3);
        stack.0.push(third);
        Ok(())
    }
    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        let third = stack.remove(2);
        stack.push(third);
        Ok(())
    }
}

