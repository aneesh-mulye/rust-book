fn square(x: i32) -> i32 {
    x * x
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("{}", add(square(3), square(4)));
}
