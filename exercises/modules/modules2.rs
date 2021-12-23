// modules2.rs
// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.
// Make me compile! Execute `rustlings hint modules2` for hints :)

// 

mod delicious_snacks {

    // TODO: Fix these use statements
    // pub use self::fruits::PEAR as fruit;
    // pub use self::veggies::CUCUMBER as veggie;

    // 思考：如果不用 pub use 怎么打印？

    // pub
    // pub(crate)
    pub(super)
    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }
    // pub
    // pub(crate)
    pub(super)
    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    // println!(
    //     "favorite snacks: {} and {}",
    //     delicious_snacks::fruit,
    //     delicious_snacks::veggie
    // );

    // println!(
    //     "favorite snacks: {} and {}",
    //     delicious_snacks::fruits::APPLE, // error[E0603]: module `fruits` is private
    //     delicious_snacks::veggies::CARROT,
    // );

    println!(
            "favorite snacks: {} and {}",
            delicious_snacks::fruits::APPLE, 
            delicious_snacks::veggies::CARROT,
        );
}
