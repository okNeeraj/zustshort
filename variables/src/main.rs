fn main() {
    /**
     * Variable declaration
     * By default, variables in Rust are immutable,
     * meaning their values cannot be changed after they've been initialized.
     * You can make variables mutable by using the mut keyword.
     */
    let x: i32 = 5;
    println!("Value of X is - {}", x);

    let mut y: i32 = 10;
    println!("Initial value of y is - {}", y);

    y = 20;
    println!("Value of Y after override is - {}", y);

    /**
     * Variable Shadowing
     * Rust allows you to shadow variables,
     * which means you can declare a new variable with the same name as an existing variable.
     * This is useful for reusing variable names without having to introduce new names.
     */
    let z = 5;

    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }

    println!("The value of z is: {z}");

    /**
     * Constants
     * In addition to variables, Rust also supports constants,
     * which are similar to variables but their values cannot be changed.
     * Constants are declared using the const keyword and must have a specified type.
     */

    const MAX_POINTS: u32 = 10000; // Declares a constant named 'MAX_POINTS' of type u32 (unsigned 32-bit integer) with the value 10000

    /**
     * Data Type
     * Rust provides a variety of built-in data types,
     * including integers, floating-point numbers, booleans, characters, tuples, arrays, and more.
     * Each data type has its own set of properties and behaviors.
     */
    let integer: i32 = 42; // 32-bit integer
    let float: f64 = 3.14; // 64-bit floating-point number
    let boolean: bool = true; // Boolean
    let character: char = 'a'; // Character
    let tuple: (i32, f64, char) = (42, 3.14, 'a'); // Tuple
    let array: [i32; 3] = [1, 2, 3]; // Array of 3 integers
}
