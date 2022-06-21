fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let y = 7;

    println!("Floor division: {} / {} = {}", x, y, x / y);
    println!("Remainder: {} % {} = {}", x, y, x % y);

    // Tuples
    let tup = (123, 12.3, "steve");
    println!("Tuple: {:?}", tup);
    println!("Tuple first value: {}", tup.0);

    // Initialising array with the same value
    let arr = [3; 5];
    println!("Array with same values: {:?}",  arr);
    println!("Array with same values (pretty): {:#?}",  arr);
}
