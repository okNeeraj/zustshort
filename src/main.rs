fn main() {
    println!("Hello, world! hh");
    println!();

    let b = "Hello Rust";
    println!("{}", b);

    let calculate = cn();
    println!("{}", calculate);
}

fn cn() -> i32 {
    5 + 5
}
