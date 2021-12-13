// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)

// 

fn main() {
    let x : i32 = 10; // 思考： 为神马 Rust 编译器不允许未初始化？ 出于内存安全的考虑
    if x == 10 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}
