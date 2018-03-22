//
// compile : rustc error.rs 
//
use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, MyError>;

#[derive(Debug, Clone)]
// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
struct MyError;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for MyError {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

fn main() {
    let x : i32 = 0;
    let numbers = vec!["tofu", "93", "18"];
    numbers.first().ok_or(MyError); 
}