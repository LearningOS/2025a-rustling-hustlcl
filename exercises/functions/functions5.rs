// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let num: i32 = 3;
    let answer = square(num);
    println!("The square of 3 is {answer}");
}

fn square(num: i32) -> i32 {
    num * num
}
