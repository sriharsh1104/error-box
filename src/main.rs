// use std::env;

// fn main() {
//     let u = match env::var_os("USER") {
//         Some(v) => v.into_string().unwrap(),
//         None => panic!("$USER is not set")
//     };
//     println!("Got username: {}", u);
//     println!("{:?}",verbose());
//     println!("{:?}",short());

// }
// fn verbose() {
//     let name = "USER";
//     match env::var(name) {
//         Ok(v) => println!("{}: {}", name, v),
//         Err(e) => panic!("${} is not set ({})", name, e)
//     }
// }

// fn short() {
//     let v = env::var("USER").expect("$USER is not set");
// }

// use std::fmt;

// type Result<T> = std::result::Result<T, DoubleError>;

// #[derive(Debug, Clone)]
// struct DoubleError;

// impl fmt::Display for DoubleError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "invalid first item to double")
//     }
// }

// fn double_first(vec: Vec<&str>) -> Result<i32> {
//     vec.first()
//         .ok_or(DoubleError)
//         .and_then(|s| {
//             s.parse::<i32>()
//                 .map_err(|_| DoubleError)
//                 .map(|i| 2 * i)
//         })
// }

// fn print(result: Result<i32>) {
//     match result {
//         Ok(n) => println!("The first doubled is {}", n),
//         Err(e) => println!("Error: {}", e),
//     }
// }

// fn main() {
//     let numbers = vec!["42", "93", "18"];
//     let empty = vec![];
//     let strings = vec!["tofu", "93", "18"];

//     print(double_first(numbers));
//     print(double_first(empty));
//     print(double_first(strings));
// }

use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into())
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into()) 
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
