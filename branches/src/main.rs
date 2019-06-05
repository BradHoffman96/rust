fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition is true.");
    } else if number % 4 == 0 {
        println!("Number is not divisable by 4.");
    } else {
        println!("No conditions were met");
    }


    //an example of using conditions within a let statement
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);
}
