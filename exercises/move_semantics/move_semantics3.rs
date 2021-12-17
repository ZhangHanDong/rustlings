// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` for hints :)

// 

fn main() {
    let mut vec0 = Vec::new();

    let mut vec1 = fill_vec(&mut vec0);

    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // vec1.push(88);

    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// 思考： 将参数改为可变引用？
// 新手： `mut vec`  vs  `&mut Vec`
// fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
fn fill_vec(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    // let mut vec = vec;
    vec.push(22); // Vec::push(&mut Vec<i32>)
    vec.push(44);
    vec.push(66);

    vec
}
