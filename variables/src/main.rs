fn main() {

    //x is mutable once the 'mut' keyword is added
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //y is immutable, but using let will allow us to reassign and then have the value be immutable
    //this is referred to as 'shadowing'
    let y = 12;
    println!("The value of y is: {}", y);
    let y = y * 2;
    println!("The value of y is: {}", y);

    //declaration of a tuple
    let tup : (i32, f64, u8) = (500, 6.4, 1);
    //example of destructuring a tuple, to access its values
    let (x, y, z) = tup;
    println!("The value of z is: {}", z);
    //you can also access a tuple's value through a period (.)
    println!("The value at index 0 of tup is: {}", tup.0);

    //declaring an array
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("The first value in the array is: {}", first);
}