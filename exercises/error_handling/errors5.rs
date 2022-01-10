// errors5.rs

// This program uses a completed version of the code from errors4.
// It won't compile right now! Why?
// Execute `rustlings hint errors5` for hints!

// 

use std::error;
use std::fmt;
use std::num::{ParseIntError, IntErrorKind};

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// 思考： 使用 From 如何转换？

// impl Into<ParseIntError> for CreationError {
//     fn into(self) -> ParseIntError {
//         match self{
//             CreationError::Negative => {
//                 ParseIntError{kind: IntErrorKind::NegOverflow} 
//             }
//             CreationError::Zero =>  { ParseIntError{kind: IntErrorKind::Zero} }
//         }
//     }
// }

// impl From<CreationError> for ParseIntError {
//     fn from(e: CreationError) -> ParseIntError {
//         match e{
//             CreationError::Negative => {
//                 ParseIntError{kind: IntErrorKind::NegOverflow} 
//             }
//             CreationError::Zero =>  { ParseIntError{kind: IntErrorKind::Zero} }
//         }
//     }
// }

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64))
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

// 去看标准库 Error
impl error::Error for CreationError {}
