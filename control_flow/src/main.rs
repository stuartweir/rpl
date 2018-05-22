fn main() {
    let number = 6;

    // must evaluate to a bool
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let num = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", num);

    // different types of loops
    loop {
        println!("again!");
        break;
    }

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!");

    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
    // vs...
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // using a range, reversed
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!");
}
