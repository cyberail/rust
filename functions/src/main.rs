fn main() {
    println!("Hello, world!");
    my_func(23);

    println!("return value {}", returnable());

    println!("return value {}", plus_one(3));
}

fn my_func(x: i32) {
    println!("My func {x}");
}

fn returnable() -> i32 {
    let f = 32;
    f
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
