fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // still immutable, but 'shadowing'
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // we are annotating the type, since there could be potentially many options here
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value(s) of the tup are: {}, {}, {}", x, y, z);
    println!("Printing tup with dot notation: {}, {}, {}", tup.0, tup.1, tup.2);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("The months of the year are: {:#?}", months);
}
