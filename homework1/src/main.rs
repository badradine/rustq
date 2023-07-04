fn main() {
    for result in 1..=100 {
        if result % 15 == 0 {
            println!("FizzBuzz");
        } else if result % 3 ==0 {
            println!("Fizz");
        } else if result % 5 ==0 {
            println!("Buzz");
        } else {
            println!("{}", result);
        }
    }
}


