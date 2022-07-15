use crate::runtime::{AliceStack, AliceTable, AliceVal};

pub trait Statement {
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

impl Statement for PushStatement {
    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        stack.push(self.0.clone());
        Ok(())
    }
}

impl Statement for PrintlnStatement {
    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        let val = stack.pop()?;
        println!("{val}");
        Ok(())
    }
}

impl Statement for PrintStatement {
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
