#//this is a comment- obviously. the language will start running from the first line.
#//klang is staticly typed and is interpreted using rust.
#//In Klang, although indentations are not mandatory, it is strongly recommended to adhere to formal indentation rules to maintain clean and readable code.
#//klang has 4 simple types: bool, number, string, vector. chars will just be stored inside strings. deal with it.
#// var declaration:
#//let identifier = value(?);
#let x = 3;
#//vec would be defined as let x = [1, 2, 3];
#// control flow:
#//klang uses <, >, <=, >=, ==, &&, || for control flow. and will use the basic +, -, *, /, % for arithmatic operations
#//if bool && bool || bool {
#//    expr;
#//} else {
#//    expr;
#//}
#if x > 1 {
#    x = 2;
#} else {
#    x = -2;
#}
#// loops:
#// for identifier in iterable { expr; }
#// while bool { expr; }
#// range: 
#// int..int exclusive, int..int..int3 exclusive, step size is int3.
#for i in 1..7..2 {
#    x = i; // x = 1, 3, 5
#}
#while x > 2 {
#    x = -3;
#}
#// klang will use {} for scoping
#// I/O: klang will use print() for output and std::read() for input
#let y: string = read();
#//print("text {identifier} text");
#print("this is how you use a print to output {y}");
#//klang will not feature any error handling. every error will be handled by the parser, scanner, compiler and reported to the developer in the terminal
#//functions: all functions are public. deal with it.
#//fn name(arg1: type, arg2: type) {
#//    expr;
#//    return value;
#//}
#fn add(int1: int, int2: int) {
#    return int1 + int2;
#}
#print("3 + 5 = {add(3, 5)}");

//moudles and imports: MAN IDFK
//ideas:
//switch, break, continue, std_lib, structs
//std lib will contain: advanced types, advanced math functions, random functions, time functions, also read and maybe more I/O.
