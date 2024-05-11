fn main(){
    println!("We only getting started rn!");

    // constansts
    const MAX_POINTS: u32 = 10000; // this is a constant
    
    // creating a variable


    let x = 5; // this is immutable by default
    println!("The value of x is: {}", x);
    
    

    // This is how a mutable variable is declared
    let mut y = 10;
    println!("The value of y is: {}", y); // this will give a warning because mutable variable y is not used / changed
    y += 4;
    println!("The value of y is: {}", y); 

    /// setting a specific type for a variable
    let z: u32 = 100; // this is an unsigned 32-bit integer
    // all data types
    // i8, i16, i32, i64, i128, isize - signed integers
    // u8, u16, u32, u64, u128, usize - unsigned integers
    // f32, f64 - floating point numbers
    // bool - boolean
    // char - character

    //tuple
    let t:(i32,f64,char) = (42,6.12,'j');
    // destructuring a tuple
    let (a,b,c) = t;
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    println!("The value of c is: {}", c);

    // variable shadowing
    let n = 10; // initial value of n is 10
    let n = n + 5; // but here it is overshadowed by a new value of 15
    {
        // but here the value of n is only 30 while it is within the scope, outside the scope the value of n is 15 again
        let n = n * 2;
        println!("The value of n is {n}");
    }
    println!("The value of n is {n}");

    let mut spaces = "     ";
    // spaces = spaces.len(); -> this throws error as we cant change type of a variable
    let len = spaces.len();
    // println!(spaces);

}
