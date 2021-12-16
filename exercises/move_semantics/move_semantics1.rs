// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)

// 

// 思考： 栈 vs 堆 ？

// 栈 -> 编译期识别类型的大小，默认分配的空间
// 堆 -> 运行时动态分配

// 思考： 
// 1. 了解到move语义是什么之后，进一步思考，为什么要这么设计？

fn main() {
    let vec0 = Vec::new(); // 动态增长 - 堆
    let vec1 = &vec0;

    let mut vec1 = fill_vec(vec0);


    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);// Vec::push(&mut vec1, 88)

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // println!("{} has  content `{:?}`", "vec0", vec0); // move  思考一下： move以后为什么不让用？ 结合 Rust 语言原则是内存安全
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22); // Vec::push(&mut vec, 22)
    vec.push(44);
    vec.push(66);

    vec
}
