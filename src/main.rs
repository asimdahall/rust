fn main() {
    let x: u32 = 100;
    println!("The value of X is {}", x);

    // @info: this is called shadowing, you can user the same variable with the let keyword.
    let x: i32 = -20;
    println!("The value of X is {}", x);

    let y: bool = false;
    println!("The value of Y is {}", y);
}
