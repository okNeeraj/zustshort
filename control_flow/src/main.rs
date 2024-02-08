fn main() {
    let n = 3;

    if n < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    /**
     * Using if in a let Statement
     * Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable,
     * as in Listing 3-2.
     */
    let condition = true;
    let num = if condition { 5 } else { 6 };

    println!("The value of number is: {num}");
}
