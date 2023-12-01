///Functions and Closures
//We have been using functions already, including the main() which is the program entry point
//In this section, we are particularly highlighting the fact that functions
//have a type unto themselves and variables of a given function type
//can be declared and passed to another function.
//So, we can have a series of function calls, the output of one becoming the input of
//the next. Herein lies the concept of higher order functions

//As already mentioned, in Rust, functions have their own types.
//Below is an illustration

///Function to add to two signed integers. Returns a signed integer
fn add(a: i32, b: i32) -> i32 {
    a + b
}
//The function type embodied in the above is fn(i32, i32) -> i32.
//Function type is defined by the keyword fn followed 
//by the optional expected parameter types
//and then the optional expected return type.

///Here we define a function name apply that is expected to receive the function type
/// above name f here, along with two other unsigned interger parameters named x and y
/// respectively
fn apply_add(f: fn(i32, i32) -> i32, x: i32, y: i32) -> i32 {
    f(x, y) //a call to the function passed, which in its turn is passed two other parameters
}