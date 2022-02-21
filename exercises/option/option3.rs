// option3.rs
// Make me compile! Execute `rustlings hint option3` for hints

// 

// 思考：Rust 编译器只有一个 move语义 
// Copy 是 core 库加的 trait ，行为
// move 本质就是 让变量变为 未初始化 状态
// Copy 实际上就是 让 move 以后的变量 初始化为原来的值。
// 编译器 release 模式可能对 按位复制 有一些优化：copy 省略 ，在汇编里看不到 memcpy 指令

// #[derive(Copy,Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}   
