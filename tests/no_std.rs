#![no_std]

use enum_dispatch::enum_dispatch;
use core::convert::TryInto;

struct MyImplementorA {
}

impl MyBehavior for MyImplementorA {
    fn my_trait_method(&self) {
    }
}

struct MyImplementorB {
}

impl MyBehavior for MyImplementorB {
    fn my_trait_method(&self) {
    }
}

#[enum_dispatch]
enum MyBehaviorEnum {
    MyImplementorA,
    MyImplementorB,
}

#[enum_dispatch(MyBehaviorEnum)]
trait MyBehavior {
    fn my_trait_method(&self);
}

#[test]
fn main() {
    let a: MyBehaviorEnum = MyImplementorA {}.into();
    a.my_trait_method();

    let a: MyImplementorA = a.try_into().unwrap();
}
