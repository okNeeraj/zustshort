fn main() {
    println!("Hello, world!");
    another_function();
    function_params(10);
    print_labeled_measurement(5, 'h');
}

fn another_function() {
    println!("Another function.");
}

fn function_params(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/**
 * Statements and Expressions
 * Function bodies are made up of a series of statements optionally ending in an expression.
 * So far, the functions we’ve covered haven’t included an ending expression, 
 * but you have seen an expression as part of a statement. 
 * Because Rust is an expression-based language, this is an important distinction to understand. 
 * Other languages don’t have the same distinctions, 
 * so let’s look at what statements and expressions are and how their differences affect the bodies of functions.
 * 
 * Statements are instructions that perform some action and do not return a value.
 * Expressions evaluate to a resultant value. Let’s look at some examples.
 * 
 * We’ve actually already used statements and expressions. 
 * Creating a variable and assigning a value to it with the let keyword is a statement.
 * In Listing 3-1, let y = 6; is a statement.
 */






