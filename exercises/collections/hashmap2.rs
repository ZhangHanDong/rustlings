// hashmap2.rs

// A basket of fruits in the form of a hash map is given. The key
// represents the name of the fruit and the value represents how many
// of that particular fruit is in the basket. You have to put *MORE
// THAN 11* fruits in the basket. Three types of fruits - Apple (4),
// Mango (2) and Lychee (5) are already given in the basket. You are
// not allowed to insert any more of these fruits!
//
// Make me pass the tests!
//
// Execute the command `rustlings hint hashmap2` if you need
// hints.

// 

use std::collections::HashMap;

// 注意这三个trait
#[derive(Hash, PartialEq, Eq)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = vec![
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        // TODO: Put new fruits if not already present. Note that you
        // are not allowed to put any type of fruit that's already
        // present!

        match fruit {
            Fruit::Apple => {
                // match *basket.get(&Fruit::Apple){
                //     Some(v) => { }
                //     None => 
                // }

                // if let None = *basket.get(&Fruit::Apple) {
                //     basket.insert(Fruit::Apple, 4);
                // }
                // 思考： 操作符优先级，这里需要加括号
                // https://doc.rust-lang.org/stable/reference/expressions.html#expression-precedence
                let v = (*basket).get(&Fruit::Apple);
                if v.is_none() {
                    basket.insert(Fruit::Apple, 4);
                }
            }
            Fruit::Banana => {
                let v = (*basket).get(&Fruit::Banana);
                if v.is_none() {
                    basket.insert(Fruit::Banana, 2);
                }
            }
            Fruit::Mango => {
                let v = (*basket).get(&Fruit::Mango);
                if v.is_none() {
                    basket.insert(Fruit::Mango, 2);
                }
            }
            Fruit::Lychee => {
                let v = (*basket).get(&Fruit::Lychee);
                if v.is_none() {
                    basket.insert(Fruit::Lychee, 5);
                }
            }
            Fruit::Pineapple => {
                let v = (*basket).get(&Fruit::Pineapple);
                if v.is_none() {
                    basket.insert(Fruit::Pineapple, 2);
                }
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        // turbofish ::<> 
        let mut basket = HashMap::<Fruit, u32>::new();
        basket.insert(Fruit::Apple, 4);
        basket.insert(Fruit::Mango, 2);
        basket.insert(Fruit::Lychee, 5);

        basket
    }

    // 思考：unwrap() 能不能随便用？
    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        // 思维：为什么这里不加括号？*basket
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4); 
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }
}
