// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

// I AM NOT DONE

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}
// 思考： 分号的意义？ 查询一下 OCmal 里面分号语法的意义，Rust 借鉴了它。
// https://ocamlverse.github.io/content/faq_if_semicolon.html
fn square(num: i32) -> i32 {
    num * num
}
