// functions4.rs
// Make me compile! Execute `rustlings hint functions4` for hints :)

// This store is having a sale where if the price is an even number, you get
// 10 Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.

// 

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

// 思考：
// 1. 表达式 和 分号
// 2. 为什么 Rust 没有 三元操作符？
// 3. 是否可以 let a = if { ... } 
fn sale_price(price: i32) -> i32  {
    let value = if is_even(price) { price - 10 } else { price - 3 };
    value
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
