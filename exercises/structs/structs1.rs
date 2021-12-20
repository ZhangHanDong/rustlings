// structs1.rs
// Address all the TODOs to make the tests pass!

//

struct ColorClassicStruct {
    // TODO: Something goes here
    name: &'static str, 
    hex: &'static str, 
}

// 面向 C 过程式

fn new(name: &'static str, hex: &'static str) -> ColorClassicStruct {
    ColorClassicStruct{name, hex}
}

// --- 重构 ： 面向对象风格

impl ColorClassicStruct{
    fn new(name: &'static str, hex: &'static str) -> Self{
        Self{name, hex }
    }

    fn other(){
        // ..
    }
}

struct ColorTupleStruct<'a>(&'a str, &'a str);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct::new("green","#00FF00");

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct("green","#00FF00");
        let green2 = ColorTupleStruct("green","#00FF00");

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        let unit_struct = UnitStruct;
        let unit_struct2 = UnitStruct;

        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}


// 思考： 

// 1. 元组结构体 vs 具名结构体： https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=fba56cbe824be089d820771bc1dc0493

// 2. 为什么需要单元结构体？

// 3. 两个单元结构体的实例是否为同一个？