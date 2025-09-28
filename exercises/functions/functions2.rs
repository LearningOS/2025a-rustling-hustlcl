// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let num: i32 = 3;
    call_me(num);
}

fn call_me(num: i32) {
    for i in 0..num {
        let print_num = i + 1;
        println!("Ring! Call number {print_num}");
    }
}
