// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

// Execute `rustlings hint generics2` for hints!

// 

// struct Wrapper {
//     value: u32,
// }

// impl Wrapper {
//     pub fn new(value: u32) -> Self {
//         Wrapper { value }
//     }
// }
use std::fmt::Debug;

// 类型系统术语：？  
//  T ，泛型 代表 所有的具体类型

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

impl<T: Debug+Eq> Wrapper<T> {

    pub fn eq(&self, v: T) -> bool {
        self.value == v
    }
}

// impl Trait（存在类型） 代表 满足 Eq 约束的 某一个具体类型 
// fn eq2(v1: impl Eq, v2: impl Eq) -> bool {
//     v1 == v2
// }

// fn eq2<T: Eq>(v1: T, v2: T) -> bool {
//     v1 == v2
// }

// 思考：泛型函数有缺陷
struct NotEq;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        let w = Wrapper::new(42);
        w.eq(42);
        // assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        let w = Wrapper::new("Foo");
        w.eq("Foo");
        // assert_eq!(Wrapper::new("Foo").value, "Foo");
    }

    // #[test]
    // fn store_str_in_wrapper() {
    //     assert_eq!(Wrapper::new(NotEq).value, NotEq);
    // }
}
