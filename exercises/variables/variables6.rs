// variables6.rs
// Make me compile! Execute the command `rustlings hint variables6` if you want a hint :)

// 

// 思考：为什么常量需要指定类型？而变量可以推导。
// 参考： https://github.com/rust-lang/rfcs/issues/1349
// https://users.rust-lang.org/t/why-is-type-declaration-necessary-in-constants/14200/3

const NUMBER : i32 = 3;

fn main() {
    println!("Number {}", NUMBER);
}
