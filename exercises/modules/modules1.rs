// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)

//

// 可见性 以模块为界

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }
    
    // pub
    // pub(crate)
    pub(super)
    fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    // path
    sausage_factory::make_sausage();
}

// 思考： 
// 1. mod vs use


// crate -> lib.rs （出口/入口）top level -> API (Pub use )
//            ｜—— btree.rs (mod) -> API (Pub )
//                   |_ map (mod) ->  API (private) // use crate::btree::set;
//                   |_ set (mod) ->  API (private) // use super::map;


// use crate::xxx; / use super::xxx; use self::xxx;
// use std::xxx;
// use nom::xxx;

// pub 
// pub(crate)
// pub(super)


// 模块的定义方式：
// 1. 文件就是一个模块 或者 mod{  }
// 2. 多个文件定义为一个模块，有两种方式：
//     1. btree.rs + btree/
//     2. btree/ mod.rs 


// 一般把测试单独放到一个文件里，加速编译
// map.rs + map/tests.rs

// https://toml.io/cn/v1.0.0