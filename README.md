# Klang Language

Klang is a statically-typed programming language interpreted using Rust.

## Language Overview

- Klang starts running from the first line of code.
- Indentations are not mandatory, but it is strongly recommended to adhere to formal indentation rules for clean and readable code.
- Klang has four simple types: bool, number, string, and vector. Characters are stored inside strings.
- Variable declaration: Use `let identifier = value(?)` syntax.
- Control flow: Klang uses `<`, `>`, `<=`, `>=`, `==`, `&&`, `||` for control flow.
- Arithmetic operations: Klang uses `+`, `-`, `*`, `/`, `%` for basic arithmetic operations.
- If statement:
`if expression {
    code
} else {
    code
}`
- Loops:
- For loop: `for identifier in iterable { code }`
- While loop: `while bool { code; }`
- Range:
- `int..int` (exclusive)
- `int..int..int3` (exclusive, with step size `int3`)
- Scoping: Klang uses `{}` for scoping.
- I/O: Klang uses `print()` for output and `std::read()` for input.
- Error handling: Klang does not feature explicit error handling. Errors are handled by the parser, scanner, and compiler, and reported to the developer in the terminal.
- Functions: All functions in Klang are public.
fn name(arg1: type, arg2: type) {
    code
    return value; (or return;)
}
- Example:
```klang
fn add(int1: int, int2: int) {
    return int1 + int2;
}
print("3 + 5 = {add(3, 5)}");
