use std::io;

pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut y = x;
        let mut z = 0;
        while y > 0 {
                z *= 10;
                z += y % 10;
                y /= 10;
                    }
        z == x
}

fn main(){
    let mut input_line = String::new();
    io::stdin() // the rough equivalent of `std::cin`
    .read_line(&mut input_line) // actually read the line
    .expect("Failed to read line"); // which can fail, however
let x: i32 = input_line
    .trim() // ignore whitespace around input
    .parse() // convert to integers
    .expect("Input not an integer"); // which, again, can fail

    println!("{}", is_palindrome(x));
}