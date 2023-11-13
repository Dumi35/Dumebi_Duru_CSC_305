use crate::data_types::derived;

struct Person{
        name: String,
        age: u32    
}
union Student{
        matricNo: [char; 8],
        age: u32  
}

enum month{
    january, february
}

pub fn run(){
    let red = Person{
        name: String::from("Red"),
        age: 19
    };

    //#[derived(Debug)];

    /* let blue = Student{
        matricNo: ['a','a','a','a','a','a','a','a'],
        age: 19
    }; */

    let one = month::january;
    let two = month::february;

    println!("Name: {}", red.name)
}