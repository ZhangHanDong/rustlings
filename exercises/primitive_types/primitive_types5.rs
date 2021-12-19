// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

// 

// 思考： 
// 1. 什么是模式匹配？解构？ 
// 2. 如果cat是三个元素，要取前两个
fn main() {
    let cat = ("Furry McFurson", 3.5, 5,6,8);
    let (name, age, _, _,_) = cat; // rust 有 `*_` 语法吗？

    // let name = cat.0;
    // let age = cat.1;

    println!("{} is {} years old.", name, age);
}
