fn main() {
    let x = 5;
    let mut counter = 0;

    //an example of an infinite loop
    loop {
        println!("again!");
        counter = counter + 1;
        if counter == x {
            break;
        }
    }

    //an example of a while loop
    while counter != 10 {
        println!("more!");
        counter = counter + 1;
    }

    //an example of iterating through the items in an array
    let a = [1, 2, 3, 4, 5];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    //looping through the numbers within a range
    //the lower bound is inclusive. the upper bound is not.
    for number in (1..4).rev() {
        println!("{}", number);
    }
}
