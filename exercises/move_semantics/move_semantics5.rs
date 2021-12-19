// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

// 

// ｜ 100 ｜ ｜ ｜y ｜ z｜ 
//     ^        |    | &mut x
//     |________|____|


// 思考： 为神马改完顺序就可以了？
// 思考： reborrowing ？ 后面再说
fn main() {
    // x 是 owned type
    let mut x = 100i32; // 4字节 安全的在栈上存储的类型，简单值类型

    // 借用检查器 只检查 借用/引用 &mut x ，为了防止悬垂指针或其他指针类问题
    let y     = &mut x;  // 'y lifetime start
    *y += 100;  // 'y lifetime end

    let z = &mut x; // 'z lifetime start
    *z += 1000; // x == 200 + 1000 // // 'z lifetime end

    assert_eq!(x, 1200);
}

// Move vs Copy
// 思考： 为什么 x 不等于 1200？
// 为什么有了 move 还需要 Copy ?
// fn main() {
//     let mut x = 100i32; // 4字节 安全的在栈上存储的类型，简单值类型 
//     // x 实现了 Copy trait (按位复制)
//     let mut y = x; // x 被 copy 给了y
//     let mut z = x;
//     y += 100i32; 
//     z += 1000;
//     assert_eq!(x, 1200); // x == 100
// }
