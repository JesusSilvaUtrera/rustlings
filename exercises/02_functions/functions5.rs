fn square(num: i32) -> i32 {
    num * num // No return statement required, just remove the ;
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
