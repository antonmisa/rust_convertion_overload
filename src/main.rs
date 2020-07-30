use std::fmt::Display;
use core::convert::Into;

fn my_func1<T: Into<f64> + Copy + Display>(p: T) {
	println!("value is {}", p.into());
}

trait Trait<T> {
    fn foo(&self, x: T);
}

struct Bar;

impl Trait<i32> for Bar {
    fn foo(&self, v: i32) {
        println!("value {} is i32", v);
    }
}

impl Trait<u32> for Bar {
    fn foo(&self, v: u32) {
        println!("value {} is u32", v);
    }
}

impl Trait<f32> for Bar {
    fn foo(&self, v: f32) {
        println!("value {} is f32", v);
    }
}

impl Trait<&str> for Bar {
    fn foo(&self, v: &str) {
        println!("value {} is str", v);
    }
}

fn main() {
    let x = 5;
	my_func1(x);
	
	let x = 5.5;
	my_func1(x);	
	
	Bar.foo(5);
	Bar.foo(5.5);
	Bar.foo("6");
}
