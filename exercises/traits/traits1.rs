// traits1.rs
// Time to implement some traits!
//
// Your task is to implement the trait
// `AppendBar' for the type `String'.
//
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.

//

// 思考： Rust 里有很多 trait，是否需要记忆（背下来）？
// 学习技巧：你要善于分类。

// trait：行为 / 指定类型的行为。

// 作用：
// - marker： Copy/ Size / Sync/ Send 
// - 泛型限定 => T : TraitBound 静态
// - trait 对象 => 对象（Object），动态行为  object.method
// - 接口 （架构），依赖倒置

// let traitobject = Box<dyn Trait>;
// traitobject.method(fn pointer) 
// 对象安全（Object Safe） / 动态安全 (dyn safe)

// 思考：
// 什么时候 方法是 impl A，什么时候需要 impl Trait for A ？

// struct A;

// impl A {

// }

// impl Trait for A {

// }


trait AppendBar {
    fn append_bar(self) -> Self;
    // fn append_bar(&mut self);
}


impl AppendBar for String {
    //Add your code here
    fn append_bar(mut self) -> Self {
        self.push_str("Bar");
        self
    }

    // fn append_bar(&mut self)  {
    //     self.push_str("Bar");
    // }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
