#![allow(dead_code)]
mod keyword;
mod lexer;
mod loc;
mod parser;
mod runtime;
mod statement;
mod type_check;
mod object;
mod utils;
mod flow;

use crate::lexer::AliceLexer;
use crate::parser::AliceParser;
use clap::Parser;
use std::time::{ Instant, Duration };
use std::collections::HashMap;

fn main() -> Result<(), String> {
    let args = AliceArgs::parse();
    if args.path.is_none() {
        launch_interactive();
    }

    let bench = args.bench.unwrap_or(false);
    let mut total = Duration::from_millis(0);
    let file = args.path.unwrap(); // unwrapping here is safe due to previous check
    let t0 = Instant::now();
    let tokens = AliceLexer::new(load_src(&file)?, file.clone()).tokenize();
    if bench {
        let elapsed = t0.elapsed();
        total += elapsed;
        println!("[bench] reading, tokenizing:\t{}", display_duration(&elapsed));
    }
    if let Ok(tokens) = tokens {
        let mut stack = crate::runtime::AliceStack::new(64);
        let mut table = crate::runtime::AliceTable::new(32);
        let t0 = Instant::now();
        let statements = AliceParser::new(tokens).parse(None);
        if bench {
            let elapsed = t0.elapsed();
            total += elapsed;
            println!("[bench] parsing, type checking:\t{}", display_duration(&elapsed));
        }
        if let Err(msg) = statements {
            return Err(format!("Error parsing {file}: {msg}"));
        }

        let statements = statements.unwrap();
        let t0 = Instant::now();
        for s in statements {
            if let Err(e) = s.execute(&mut stack, &mut table) {
                return Err(format!("Error executing {file}: {e}"));
            }
        }
        if bench {
            let elapsed = t0.elapsed();
            total += elapsed;
            println!("[bench] executing program:\t{}", display_duration(&elapsed));
            println!("[bench] total elapsed:\t\t{}", display_duration(&total));
        }
    } else {
        return Err(format!("Error tokenizing {file} {:?}: ", tokens.err()));
    }
    return Ok(());
}

#[derive(Parser, Debug)]
#[clap(author = "Malte Dostal <malted@duck.com>")]
#[clap(name = "alicelang")]
#[clap(about = "alicelang cli")]
struct AliceArgs {
    #[clap(short, long, value_parser)]
    /// Enable emitting of intermediate representation.
    /// Possible value(s): java
    emit: Option<String>,
    #[clap(short, long)]
    /// enables benchmark output
    bench: Option<bool>,
    #[clap(value_parser)]
    /// Path to the alice file.
    /// Empty for interactive mode
    path: Option<String>,
}

fn launch_interactive() {
    let mut stack = crate::runtime::AliceStack::new(64);
    let mut type_stack = crate::type_check::TypeStack::new();
    let mut table = crate::runtime::AliceTable::new(64);

    use std::io::Write;
    println!("interactive alice");
    let mut input = String::new();
    loop {
        print!("alice>>");
        std::io::stdout().flush().expect("flushing stdout failed");
        std::io::stdin()
            .read_line(&mut input)
            .expect("reading stdin failed");
        let s: String = input.trim().into();
        input.clear();
        let tokens = AliceLexer::new(s, "<interactive>".into()).tokenize();
        if let Ok(tokens) = tokens {
            // println!("{tokens:?}");
            let parser = AliceParser::new(tokens);
            let statements = parser.parse(Some(&mut type_stack));
            if let Err(msg) = statements {
                eprintln!("{}", format!("error parsing input: {msg}"));
                // have to redo type stack
                type_stack.vals.clear();
                for val in &stack.stack {
                    type_stack.vals.push(crate::type_check::type_bit(val));
                }
            } else {
                let statements = statements.ok().unwrap();
                for s in statements {
                    if let Err(e) = s.execute(&mut stack, &mut table) {
                        eprintln!("error: {e}");
                    }
                }
            }
        } else {
            eprintln!("Error: {:?}", tokens.err().unwrap());
        }
    }
}

fn load_src(path: &String) -> Result<String, String> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    if std::path::Path::new(path).exists() {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let mut buf_reader = BufReader::new(file);
        let mut src = String::new();
        buf_reader
            .read_to_string(&mut src)
            .map_err(|e| e.to_string())?;
        Ok(src)
    } else {
        Err("file {path} doesn't exist!".into())
    }
}

fn display_duration(dur: &Duration) -> String {
    let ms = dur.as_micros() as f64 / 1000f64;
    /*let m = ms / 1000 / 60;
    let s = ms / 1000 % 60;
    let ms = (ms % 1000) as f64 + ((dur.as_micros() % 1000) as f64 / 1000f64);*/
    format!("{} ms", ms)
}
