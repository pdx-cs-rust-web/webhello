struct Point {
    x: i64,
    y: i64,
}

fn main() {
    let p = Point { x: 0, y: 0 };
    println!("{} {}", p.x, p.y);

    for i in (1u8 ..= 10).rev() {
        let x: u16 = i.into();
        println!("{}", x);
    }
}
