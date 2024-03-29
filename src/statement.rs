use crate::flow::*;
use crate::object::*;
use crate::runtime::*;
use crate::type_check::*;

pub trait Statement {
    fn in_pattern(&self) -> StackPattern {
        StackPattern(Vec::new())
    }
    /// for stack operator type check
    fn custom_type_check(&self, _stack: &mut TypeStack) -> Result<(), TypeCheckError> {
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

/// adds the two top most stack elements
pub struct AddStatement;

/// subtracts the two top most stack elements in "reading" order:
/// a b - = a - b
pub struct SubStatement;

/// multiplies the two top most stack elements
pub struct MulStatement;

/// divides the two top most stack elements in "reading" order:
/// a b / = a / b
pub struct DivStatement;

/// raises the second element to the power of the first
/// a b ** = a^b
pub struct PowStatement;

/// "modulos" the two top most stack elements in "reading" order:
/// a b % = a % b
pub struct ModStatement;

/// clears the stack
pub struct ClearStatement;

/// binds a variable
pub struct LetStatement {
    pub ident: String,
    pub ty: u32,
    pub literal: Option<AliceVal>,
}

/// copies a variable's value from the table onto the stack
pub struct PushFromTableStatement(pub String);

/// binds a function
pub struct FunStatement {
    pub ident: String,
    pub fun: AliceFun,
}

/// executes a function from the table
pub struct ExecuteFunStatement(pub String);

/// if statement
pub struct IfStatement(pub IfContainer);

/// if-else statement
pub struct IfElseStatement(pub IfElseContainer);

/// reads a single line of input from the command line
pub struct ReadInputStatement;

/// compares the two topmost values for equality
pub struct EqsStatement;
///
/// compares the two topmost values for equality
pub struct NotEqsStatement;

/// compares the two topmost (number) values in a > fashion
pub struct GtStatement;

/// compares the two topmost (number) values in a >= fashion
pub struct GtEqsStatement;
///
/// compares the two topmost (number) values in a > fashion
pub struct LtStatement;

/// compares the two topmost (number) values in a <= fashion
pub struct LtEqsStatement;

/// negates the topmost bool
pub struct NotStatement;

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
    fn execute(&self, _stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        std::process::exit(0);
    }
}

impl Statement for DropStatement {
    fn in_pattern(&self) -> StackPattern {
        StackPattern::any(1)
    }

    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        let _ = stack.pop();
        // type checker promises that stack operations can never fail
        Ok(())
    }
}

impl Statement for SwapStatement {
    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        // need to dynamically generate type check patterns
        stack.required_size(2)?;
        let second = stack.vals.remove(stack.vals.len() - 2);
        stack.vals.push(second);
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
        stack
            .vals
            .push(*stack.vals.get(stack.vals.len() - 1).unwrap()); // unwrapping safe due to previous check
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
        stack
            .vals
            .push(*stack.vals.get(stack.vals.len() - 2).unwrap()); // unwrapping safe due to previoud check
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
        let third = stack.vals.remove(stack.vals.len() - 3);
        stack.vals.push(third);
        Ok(())
    }
    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        let third = stack.remove(2);
        stack.push(third);
        Ok(())
    }
}

// works on number + number
// and string + string
impl Statement for AddStatement {
    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        stack.required_size(2)?;
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.vals.push(match (a, b) {
            (INT, INT) => INT,
            (FLOAT, FLOAT) => FLOAT,
            (INT, FLOAT) | (FLOAT, INT) => FLOAT,
            (STRING, STRING) => STRING,
            _ => {
                return Err(TypeCheckError(
                    "+ only works on numbers and string+string concat".into(),
                ))
            }
        });
        Ok(())
    }

    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        let b = stack.pop()?;
        let a = stack.pop()?;
        // all unwrapping is safe due to type checker
        match (a, b) {
            (AliceVal::Int(a), AliceVal::Int(b)) => {
                stack.push(AliceVal::Int(Some(a.unwrap() + b.unwrap())))
            }
            (AliceVal::Float(a), AliceVal::Float(b)) => {
                stack.push(AliceVal::Float(Some(a.unwrap() + b.unwrap())))
            }
            (AliceVal::Int(a), AliceVal::Float(b)) => {
                stack.push(AliceVal::Float(Some(a.unwrap() as f64 + b.unwrap())))
            }
            (AliceVal::Float(a), AliceVal::Int(b)) => {
                stack.push(AliceVal::Float(Some(a.unwrap() + b.unwrap() as f64)))
            }
            (AliceVal::String(a), AliceVal::String(b)) => stack.push(AliceVal::String(Some({
                let mut s = a.unwrap().clone();
                s.push_str(b.unwrap().as_str());
                s
            }))),
            _ => (),
        }
        Ok(())
    }
}

// works on number - number
impl Statement for SubStatement {
    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        stack.required_size(2)?;
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.vals.push(match (a, b) {
            (INT, INT) => INT,
            (FLOAT, FLOAT) => FLOAT,
            (INT, FLOAT) | (FLOAT, INT) => FLOAT,
            _ => return Err(TypeCheckError("- only works on numbers".into())),
        });
        Ok(())
    }

    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        let b = stack.pop()?;
        let a = stack.pop()?;
        // all unwrapping is safe due to type checker
        match (a, b) {
            (AliceVal::Int(a), AliceVal::Int(b)) => {
                stack.push(AliceVal::Int(Some(a.unwrap() - b.unwrap())))
            }
            (AliceVal::Float(a), AliceVal::Float(b)) => {
                stack.push(AliceVal::Float(Some(a.unwrap() - b.unwrap())))
            }
            (AliceVal::Int(a), AliceVal::Float(b)) => {
                stack.push(AliceVal::Float(Some(a.unwrap() as f64 - b.unwrap())))
            }
            (AliceVal::Float(a), AliceVal::Int(b)) => {
                stack.push(AliceVal::Float(Some(a.unwrap() - b.unwrap() as f64)))
            }
            _ => (),
        }
        Ok(())
    }
}

// works on number - number
impl Statement for MulStatement {
    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        stack.required_size(2)?;
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.vals.push(match (a, b) {
            (INT, INT) => INT,
            (FLOAT, FLOAT) => FLOAT,
            (INT, FLOAT) | (FLOAT, INT) => FLOAT,
            _ => return Err(TypeCheckError("* only works on numbers".into())),
        });
        Ok(())
    }

    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        let b = stack.pop()?;
        let a = stack.pop()?;
        // all unwrapping is safe due to type checker
        match (a, b) {
            (AliceVal::Int(a), AliceVal::Int(b)) => {
                stack.push(AliceVal::Int(Some(a.unwrap() * b.unwrap())))
            }
            (AliceVal::Float(a), AliceVal::Float(b)) => {
                stack.push(AliceVal::Float(Some(a.unwrap() * b.unwrap())))
            }
            (AliceVal::Int(a), AliceVal::Float(b)) => {
                stack.push(AliceVal::Float(Some(a.unwrap() as f64 * b.unwrap())))
            }
            (AliceVal::Float(a), AliceVal::Int(b)) => {
                stack.push(AliceVal::Float(Some(a.unwrap() * b.unwrap() as f64)))
            }
            _ => (),
        }
        Ok(())
    }
}

// works on number - number
impl Statement for DivStatement {
    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        stack.required_size(2)?;
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.vals.push(match (a, b) {
            (INT, INT) => INT,
            (FLOAT, FLOAT) => FLOAT,
            (INT, FLOAT) | (FLOAT, INT) => FLOAT,
            _ => return Err(TypeCheckError("/ only works on numbers".into())),
        });
        Ok(())
    }

    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        let b = stack.pop()?;
        let a = stack.pop()?;
        // all unwrapping is safe due to type checker
        match (a, b) {
            (AliceVal::Int(a), AliceVal::Int(b)) => {
                stack.push(AliceVal::Int(Some(a.unwrap() / b.unwrap())))
            }
            (AliceVal::Float(a), AliceVal::Float(b)) => {
                stack.push(AliceVal::Float(Some(a.unwrap() / b.unwrap())))
            }
            (AliceVal::Int(a), AliceVal::Float(b)) => {
                stack.push(AliceVal::Float(Some(a.unwrap() as f64 / b.unwrap())))
            }
            (AliceVal::Float(a), AliceVal::Int(b)) => {
                stack.push(AliceVal::Float(Some(a.unwrap() / b.unwrap() as f64)))
            }
            _ => (),
        }
        Ok(())
    }
}

// works on number - number
impl Statement for PowStatement {
    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        stack.required_size(2)?;
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.vals.push(match (a, b) {
            (INT, INT) => INT,
            (FLOAT, FLOAT) => FLOAT,
            (FLOAT, INT) => FLOAT,
            (INT, FLOAT) => {
                return Err(TypeCheckError(
                    "cannot raise an int to the power of a float".into(),
                ))
            }
            _ => return Err(TypeCheckError("** only works on numbers".into())),
        });
        Ok(())
    }

    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        let b = stack.pop()?;
        let a = stack.pop()?;
        // all unwrapping is safe due to type checker
        match (a, b) {
            (AliceVal::Int(a), AliceVal::Int(b)) => stack.push(AliceVal::Int(Some(
                a.unwrap().pow(b.unwrap().try_into().unwrap()),
            ))),
            (AliceVal::Float(a), AliceVal::Float(b)) => {
                stack.push(AliceVal::Float(Some(a.unwrap().powf(b.unwrap()))))
            }
            (AliceVal::Float(a), AliceVal::Int(b)) => stack.push(AliceVal::Float(Some(
                a.unwrap().powi(b.unwrap().try_into().unwrap()),
            ))),
            _ => (),
        }
        Ok(())
    }
}

// works on number - number
impl Statement for ModStatement {
    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        stack.required_size(2)?;
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        stack.vals.push(match (a, b) {
            (INT, INT) => INT,
            (FLOAT, FLOAT) => FLOAT,
            (INT, FLOAT) | (FLOAT, INT) => FLOAT,
            _ => return Err(TypeCheckError("** only works on numbers".into())),
        });
        Ok(())
    }

    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        let b = stack.pop()?;
        let a = stack.pop()?;
        // all unwrapping is safe due to type checker
        match (a, b) {
            (AliceVal::Int(a), AliceVal::Int(b)) => {
                stack.push(AliceVal::Int(Some(a.unwrap() % b.unwrap())))
            }
            (AliceVal::Float(a), AliceVal::Float(b)) => {
                stack.push(AliceVal::Float(Some(a.unwrap() % b.unwrap())))
            }
            (AliceVal::Int(a), AliceVal::Float(b)) => {
                stack.push(AliceVal::Float(Some(a.unwrap() as f64 % b.unwrap())))
            }
            (AliceVal::Float(a), AliceVal::Int(b)) => {
                stack.push(AliceVal::Float(Some(a.unwrap() % b.unwrap() as f64)))
            }
            _ => (),
        }
        Ok(())
    }
}

impl Statement for ClearStatement {
    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        stack.required_size(0)?;
        stack.vals.clear();
        Ok(())
    }

    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        stack.stack.clear();
        Ok(())
    }
}

impl Statement for LetStatement {
    fn in_pattern(&self) -> StackPattern {
        StackPattern::single(self.ty)
    }

    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        stack.vars.insert(self.ident.clone(), self.ty);
        Ok(())
    }

    fn execute(&self, stack: &mut AliceStack, table: &mut AliceTable) -> Result<(), String> {
        table.put(
            self.ident.clone(),
            if self.literal.is_some() {
                self.literal.as_ref().unwrap().clone()
            } else {
                stack.pop().unwrap()
            },
        );
        Ok(())
    }
}

impl Statement for PushFromTableStatement {
    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        if let Some(ty) = stack.vars.get(&self.0) {
            stack.vals.push(*ty);
            Ok(())
        } else {
            Err(TypeCheckError(format!(
                "variable binding {} doesn't exist when this executes",
                self.0
            )))
        }
    }

    fn execute(&self, stack: &mut AliceStack, table: &mut AliceTable) -> Result<(), String> {
        // unwrapping safe due to type checker
        stack.push(table.get(&self.0).unwrap().clone());
        Ok(())
    }
}

impl Statement for ExecuteFunStatement {
    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        let sig = stack.funs.remove(&self.0);
        if sig.is_some() {
            let sig_clone = sig.as_ref().unwrap().clone();
            stack.funs.insert(self.0.clone(), sig.unwrap());
            let sig = sig_clone;
            sig.0.type_check(stack)?;
            if sig.1 != 0 {
                stack.vals.push(sig.1);
            }
            Ok(())
        } else {
            Err(TypeCheckError(format!(
                "function '{}' doesn't exist when this executes!",
                self.0
            )))
        }
    }

    fn execute(&self, stack: &mut AliceStack, table: &mut AliceTable) -> Result<(), String> {
        let fun = table.take(&self.0);
        if fun.is_none() {
            panic!("fix your type checker, dumbass!")
        }
        let fun = fun.unwrap();
        let fun_clone = fun.clone();
        table.put(self.0.clone(), fun);
        if let AliceVal::Function(Some(f)) = fun_clone {
            f.execute(stack, table)
        } else {
            panic!("fix your type checker, dumbass")
        }
    }
}

impl Statement for FunStatement {
    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        stack.funs.insert(
            self.ident.clone(),
            (self.fun.args.clone(), self.fun.return_type),
        );
        Ok(())
    }

    fn execute(&self, _stack: &mut AliceStack, table: &mut AliceTable) -> Result<(), String> {
        table.put(
            self.ident.clone(),
            AliceVal::Function(Some(self.fun.clone())),
        );
        Ok(())
    }
}

impl Statement for IfStatement {
    fn in_pattern(&self) -> StackPattern {
        StackPattern::single(BOOL)
    }

    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        let stack_size_0 = stack.vals.len();
        check_rc(stack, &self.0.body)?;
        if stack.vals.len() != stack_size_0 {
            Err(TypeCheckError(
                "if without else part is not allowed to modify stack".into(),
            ))
        } else {
            Ok(())
        }
    }

    fn execute(&self, stack: &mut AliceStack, table: &mut AliceTable) -> Result<(), String> {
        if let Ok(AliceVal::Bool(Some(b))) = stack.pop() {
            if b {
                for s in &self.0.body {
                    s.execute(stack, table)?;
                }
            }
        } else {
            panic!("fix your type checker!")
        }
        Ok(())
    }
}

impl Statement for IfElseStatement {
    fn in_pattern(&self) -> StackPattern {
        StackPattern::single(BOOL)
    }

    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        let mut stack_clone = stack.clone();
        check_rc(stack, &self.0.if_body)?;
        check_rc(&mut stack_clone, &self.0.else_body)?;
        if stack_clone == stack {
            Ok(())
        } else {
            Err(TypeCheckError(
                "if and else body don't have equal affect on stack".into(),
            ))
        }
    }

    fn execute(&self, stack: &mut AliceStack, table: &mut AliceTable) -> Result<(), String> {
        if let Ok(AliceVal::Bool(Some(b))) = stack.pop() {
            for s in if b {
                &self.0.if_body
            } else {
                &self.0.else_body
            } {
                s.execute(stack, table)?;
            }
        } else {
            panic!("fix your type checker!")
        }
        Ok(())
    }
}

impl Statement for ReadInputStatement {
    fn out_pattern(&self) -> StackPattern {
        StackPattern::single(STRING)
    }
    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        use std::io;
        let mut s = String::new();
        io::stdin().read_line(&mut s).map_err(|e| e.to_string())?;
        stack.push(AliceVal::String(Some(if s.ends_with("\n") {
            s[..s.len() - 1].into()
        } else {
            s
        })));
        Ok(())
    }
}

impl Statement for EqsStatement {
    fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
        stack.required_size(2)?;
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        if a != b {
            Err(TypeCheckError("cannot == compare values of different types".into()))
        } else {
            stack.vals.push(BOOL);
            Ok(())
        }
    }

    fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
        let a = stack.pop();
        let b = stack.pop();
        stack.push(AliceVal::Bool(Some(a == b)));
        Ok(())
    }
}

// generate <, <=, >, >=
macro_rules! cmp_statement {
    ($name:ident, $op:tt) => {
        impl Statement for $name {
            fn custom_type_check(&self, stack: &mut TypeStack) -> Result<(), TypeCheckError> {
                stack.required_size(2)?;
                // unwrapping safe due to above check
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                if !(a == FLOAT || a == INT && a == b) {
                    Err(TypeCheckError("can only arithmetically compare int to int and float to float!".into()))
                } else {
                    stack.vals.push(BOOL);
                    Ok(())
                }
            }

            fn execute(&self, stack: &mut AliceStack, _table: &mut AliceTable) -> Result<(), String> {
                // unwrapping safe due to type checker
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(AliceVal::Bool(Some(
                    match (a, b) {
                        (AliceVal::Float(f1), AliceVal::Float(f2)) => f1 $op f2,
                        (AliceVal::Int(n1), AliceVal::Int(n2)) => n1 $op n2,
                        _ => panic!("fix your type checker, dumbass"),
                    }
                )));
                Ok(())
            }
        }
    }
}

cmp_statement![GtStatement, >];
cmp_statement![GtEqsStatement, >=];
cmp_statement![LtStatement, <];
cmp_statement![LtEqsStatement, <=];

