// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

//

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

// 思考： Option 也算是一种错误处理的类型
// 暂时的不能像 Result 那样传播。未来引入 Try 关键字可以：https://doc.rust-lang.org/stable/std/ops/trait.Try.html

fn main() {
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers: [Option<u16>; 5] = [None; 5];

    for iter in 0..5 {
        let number_to_add: u16 = {
            ((iter * 1235) + 2) / (4 * 16)
        };

        numbers[iter as usize] = Some(number_to_add);
    }
}
