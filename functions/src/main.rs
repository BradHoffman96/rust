fn main() {
    another_function(3);

    let x = five();
    println!("{}", x);
}

//parameter types MUST be declared within each function
//This is an example of a "statement" function, in which no values are returned
fn another_function(x: i32) {
    println!("The value of x is: {}", x);

    //This is an example of a "statement" containing an "expression"
    let y = {
        let x = 3;

        //expressions (seen below) do not have a semi-colon after them
        //doing so would turn them into a statemewnt
        x + 1
    };

    println!("The value of y is: {}", y);
}

//An example of the ending expression used to signal the return of a function
//The "return" keyword can be used to end a function early
fn five() -> i32 {
    5
}

