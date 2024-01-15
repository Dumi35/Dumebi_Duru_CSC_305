///associate greetings module with this crate
mod greetings;
mod data_types;

extern crate hello_world_lib;


use std::string;

use greetings::{japanese, english, spanish, french};
use data_types::primitive::{scalar::scalar,compound};
use data_types::derived::user_defined;


fn multiplier(numbers: &[i64])-> i64{
    let mut product: i64 = 1;
    numbers.iter().for_each(|&number| {
        product *= number;  
    });
    product
}

#[derive(Debug)]
struct Dum{
    name: String,
}

fn add<T: std::fmt::Debug>(a:T){
    println!("{:?}",a);
}

#[cfg(test)]
#[test]
fn Funtime(){
        println!("woohoo");
}

fn main() {
    println!("Hello, world!");
    println!("{}", english::default_greeting());
    println!("{}", japanese::default_greeting());
    println!("{}", spanish::default_greeting());
    println!("{}", french::default_greeting());
    println!("{}", hello_world_lib::greeting_from_lib()); 
    
    scalar::run();
    compound::run();
    user_defined::run2();
    println!("The product is {}", multiplier(&[4,5,-6,9]));
    
    println!("{},{}", scalar::boolean().0, scalar::boolean().1);
    println!("{}", scalar::strings());
    println!("{},{},{},{}", scalar::signed_integers().0, scalar::signed_integers().1,scalar::signed_integers().2, scalar::signed_integers().3);
    println!("{},{},{},{}", scalar::unsigned_integers().0, scalar::unsigned_integers().1,scalar::unsigned_integers().2, scalar::unsigned_integers().3);
    println!("{},{}", scalar::floats().0, scalar::floats().1); 

struct test<T>{
    one:i32,
    two:lol,
    three:T

}
    
struct lol{
    yellow:i32
}
    

let lex= test{one:1,three:5,two:lol{yellow:5}};
let lex1= test{one:1,three:5.2,two:lol{yellow:5}};

let x:Vec<i32> = vec![0,1,2,3,4,5];
let mut x_iter = x.iter();
x_iter.next();

let nums = vec![1, 2, 3];
let doubled:Vec<i32> = nums.iter().map(|n| n * 2).collect();
println!("{:?}",doubled);

//println!("{:?}",add(Dum{name:"red".to_owned()}));
/* add(Dum{name:"red".to_owned()});
add(12);
add("fat");
add("fat".to_owned());
add(true);
add(16.5);
add(("true",true,false)); */


}