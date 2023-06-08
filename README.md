# Klang Language- its rust.. but dynamic!

Klang is a dynamic programming language interpreted using Rust.

## Language Overview

- Klang starts running from the first line of code.
- Indentations are not mandatory, but it is strongly recommended to adhere to formal indentation rules for clean and readable code.
- Klang has four simple types: bool, number, string, and vector.
- Variable declaration: Use `let identifier = value` syntax, variables do not require an initial value.
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
- While loop: `while bool { code }`
- Range: `int..int` (exclusive) `int..int..int3` (exclusive, with step size `int3`)
- Scoping: Klang uses `{}` for scoping.
- I/O: Klang uses `print()` for output and `std::read()` for input.
- klang allows custom format inside strings `"hi {1+2}"` would be `hi 3`, but dont allow recursive formatting (formatting inside formatting).
- meaning you can print anything you want using 1 print statement! for example: `print("3 pi is: {3 * std::pi()}");`
- Error handling: Klang does not feature explicit error handling. Errors are handled by the parser, scanner, and compiler, and reported to the developer in the terminal.
- Functions: All functions in Klang are public.
- the way you declare a function is: `fn name(arg1, arg2) {`
- you can then use return value; or return; to quit the function and return a value.
- Example:
```klang
fn add(int1, int2) {
    return int1 + int2;
}
print("3 + 5 = {add(3, 5)}");
```
- klang offers a veriety of native functions, each runs in rust! here are the native functions klang offers:
- Math Functions: `sin` `cos` `tan` `sqrt` `pow` `ln` `log` `round` `abs` `min` `max` `pi`
- Random Functions: `random` `range` `randbool`
- Time Functions: `time` `sleep`
- File I/O Functions: `readFile` `writeFile` `read`
- use them by doing `std::` and add the function name
