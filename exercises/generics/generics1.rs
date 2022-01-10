// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` for hints!

// 

// 思考： 
// 1. 泛型类型推断 
// 2. 两种指定类型的方法: 显式指定 和  turbofish 操作符（::<&str>）

fn main() {
    // let mut shopping_list: Vec<_> = Vec::new();
    // let mut shopping_list = Vec::new();
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");

}
