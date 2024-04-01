fn main() {
    let mut x = 5;
    x += 1;
    let x = if x == 5 {
        "five"
    } else {
        "?"
    };
    println!("{}", x);
}
