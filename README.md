# Alice language specification
## Introduction
Alice is an interpreted statically typed, concatenating, stack based language with a variable table.

## Specification
```ebnf
(* note that whitespaces have been left out of this definition *)
(* for the sake of simplicity. All statements have to be separated *)
(* with either a whitespace of a separator like '(', ',', '"' ... *)
(* or any operator. The latter two groups of tokens don't have to be separated *)
(* except to distinguish between two multiplications "* *" and pow operation "**" *)

(* an alice program *)
program = { statement | block };
block = "{", program, "}";
statement = phrase | literal | ident | keyword | op |,
op = "+" | "-" | "*" | "/" | "%";
eq = "=";
type = ident (* identical rules *)
phrase = let | fun |;
let = "let", ident, [(* literal definition *) eq, literal]
fun = "fun", ident, [":"], [type, ident], {",", type, ident}, ["->", type], block;
```
