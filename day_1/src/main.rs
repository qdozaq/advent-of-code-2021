fn main() {
    let c = add(1,2);
    println!("Hello, world! {}", c);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}