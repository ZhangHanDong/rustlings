// variables3.rs
// Make me compile! Execute the command `rustlings hint variables3` if you want a hint :)

// 

// 思考： 
// 1. 为什么不允许二次赋值 
// 2. 为什么不允许你修改第11行，如何修改第11行可以编译通过？
// 3. 继承式可变 和 普通的mut可变 有什么区别？

fn main() {
    let mut x = 3; // 默认不可变
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}

// 继承式可变
// fn main() {
//     let mut x = 3; // 默认不可变
//     println!("Number {}", x);
//     let x = 5; // don't change this line
//     println!("Number {}", x);
// }
// 内部可变性