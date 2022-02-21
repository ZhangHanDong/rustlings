// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

// 思考： 
// 当确定不存在 None 的情况，该如何使用 unwrap 或 expect？
// let file = File::open("some.toml").unwrap();
// let file = File::open("some.toml").expect("config file must be supply！");

fn main() {
    let optional_word = Some(String::from("rustlings"));
    // let optional_word = optional_word.unwrap();
    // TODO: Make this an if let statement whose value is "Some" type
    if let Some(word) = optional_word {
        println!("The word is: {}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }


    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let Some(integer) = optional_integers_vec.pop() {
        println!("current value: {:?}", integer);
    }
}

// 默认约定