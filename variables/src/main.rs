fn main() {
    let x = 5;
    println!("Hello {x}");
    // x = 6;

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("Hello {THREE_HOURS_IN_SECONDS}");

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_a, _b, _c) = _tup;

    println!("{}", _tup.1);

    let _ar: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:#?}", _ar);
}
