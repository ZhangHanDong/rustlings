// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

// 

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}
// 思考： 分号的意义？ 查询一下 OCmal 里面分号语法的意义，Rust 借鉴了它。
//  OCmal:  expr_a ; expr_b ;; (一个分号叫 序列运算符，两个分号叫终止符)
// Rust : expr_a ; expr_b;  (一个分号叫 序列运算符，如果分号后面什么都没有，会补上单元值`()`)
fn square(num: i32) -> i32 {
    // return num * num;
    num*num
}
