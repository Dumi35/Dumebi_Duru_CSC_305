#![allow(dead_code)]
//use crate::data_types::derived;

const PI_VALUE: f32 = std::f32::consts::PI;
/* struct Person{
        name: String,
        age: u32    
} */
union Student{
        matric_no: [char; 8],
        age: u32  
}

enum Month{
    January, February
}

pub fn run(){
    /* let red = Person{
        name: String::from("Red"),
        age: 19
    }; */

    //#[derived(Debug)];

    /* let blue = Student{
        matricNo: ['a','a','a','a','a','a','a','a'],
        age: 19
    }; */

    /* let one = Month::January;
    let two = Month::February; */

    
}

//exercise: similar implementation for circle and triangle. needs perimeter, comparison of perimeter

  //This suppresses warnings when a given declared function is  not used.

use core::cmp::Ordering; //Used dor comparison of value sizes 

pub enum Comp { //Enumerate Comparison
    LessThan,
    GreaterThan,
    Equal,
}

pub enum Gender { //Enumerate Gender
    Male,
    Female,
}
// Hello
#[derive(Debug)] //Decorate the struct Person. Debug is an inbuilt trait. This statement will provoke impl Debug for Person; Metaprogramming
struct Person {
    name: String,
    age: u8,
} 
struct Unit;
// A unit struct
//Have no state of their own but useful for
//implementing some trait.
//E.g. struct Global is a unit struct that can implement traits like Allocator
//std::fmt::Error is a unit struct that implements
//traits like Error

//A tuple struct
struct Pair(i32, f32); //No named fields but has fields

//A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct. Below Point is used as datatype in Rectangle
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}


pub fn run1() {

    //declare a variable of type Person and assign values.
    let person = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("{:#?}", person); //{:#?} implies pretty debug print person. :? is for debug print and :#? is for pretty debug print

    // Instantiate a unit struct
    let _unit = Unit;//As simple as that. If Unit implements some trait, then _unit will demand those implementations

    //declare a Point
    let point = Point { x: 10.3, y: 0.4 };

    //Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };
    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a let binding.
    let Point {
        x: left_edge,//left_edge here will be declared. If you use x:f32 only, x will be declared.
        y: top_edge,//top_edge here will be declared. If you use y:f32 only, y will be declared.
    } = point;
    dbg!(&left_edge,&top_edge);


    let _rectangle = Rectangle { //I used _ with rectangle to silence warning knowing that the variable is not used.
        //struct instantiation is an expression too as used below in Point
        top_left: Point {
            x: left_edge,//left_edge is available, thanks to the destructuring above
            y: top_edge,//top_edge is available, thanks to the destructuring above
        },
        bottom_right,
    };

    //Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    //Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    //Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}

//Let's work on user-defined traits. Traits enable us achieve polymorphism.
//We are designing Shape below for the purpose of 
//specifying all expected functions and methods in any struct that implements Shape.
trait Shape {
    type ConcreteShape; // associated type declaration that will allow each shape have diff. parameters when instatntiating
    fn area(&self) -> f32;
    fn set_name(&mut self, name: &'static str);
    fn get_name(&self) -> &str;
    fn perimeter(&self)-> f32;
    fn new(shape_data: Self::ConcreteShape) -> Self; //the concrete type that Shape trait is associated with
}
//The use of 'static lifetime above ensures that our
//compiler is clear about the availability of those values, as they are borrowed.
//static will be available throughout the lifetime of the program.

///Use Default to specify the availability of default instance creation without values passed for property
#[derive(Default, Debug, Clone)]
struct Rect {
    length: f32,
    width: f32,
    name: &'static str,
}

impl Rect {  //define default methods that will be available for only rectangles
    //default default() function. Will override derived default if any. 
    fn default() -> Self {
        Rect {
            length: 1.0,
            width: 1.0,
            name: "default_rect_name",
        }
    }

    fn set_length(&mut self, length: f32) {
        self.length = length;
    }

    fn get_length(&self) -> f32 {
        self.length
    }

    fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    fn get_width(&self) -> f32 {
        self.width
    }
}

impl Shape for Rect {
    //Associated function used to create a new Shape
    type ConcreteShape = (f32, f32, & 'static str); //will be used to define that rect has length, breadth, and name

    fn new(dimensions: (f32, f32,& 'static str)) -> Self { //used to instantiate a rect with unique values other than the default
        Rect{
            length: dimensions.0,
            width: dimensions.1,
            name: dimensions.2,
        }
    }

    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn perimeter(&self)-> f32 {
        2.0 * (self.length + self.width)
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }

}

//implement Partial Eq
impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }
    // Provided methods
    //fn lt(&self, other: &Rhs) -> bool { ... }
    //fn le(&self, other: &Rhs) -> bool { ... }
    //fn gt(&self, other: &Rhs) -> bool { ... }
    //fn ge(&self, other: &Rhs) -> bool { ... }
}


//A conversion implementation into String
//Expects a string slice with length, width, name, separated by commas
impl From<&'static str> for Rect {
    fn from(s: &'static str) -> Rect {
        let mut parts = s.split(',');
        let length = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let width = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };

        Rect { length, width, name: &name}
    }
}


//Exercise
/*
I need similar implementation for Circle and Triangle
Besides Area, I need Perimeter and comparison on the basis of Perimeter
In your submission, I need a comment against every line of code about what it is mearnt to achieve
 */

#[derive(Debug)]
struct Circle { //a struct of type circle wuth 2 fields: radius and name
    radius: f32,
    name: &'static str,
}

impl Circle {
    //default default() function. Will override derived default if any. 
    fn default() -> Self {
        Circle {
            radius: 1.0,
            name: "default_circle_name",
        }
    }

    fn set_radius(&mut self, radius:f32) {
        self.radius = radius;
    }
    fn get_radius(&self)-> f32 {
        self.radius
    }
}


impl Shape for Circle {
    //Associated function used to create a new Shape
    type ConcreteShape = (f32, & 'static str);  //will be used to define that circle has radius and name

    fn new(dimensions: (f32, & 'static str)) -> Self { //used to instantiate a circle with unique values other than the default
        Circle { radius:dimensions.0, name:dimensions.1 }
    }

    fn area(&self) -> f32 {
        (self.radius.powf(2.0) * PI_VALUE).round() as f32  //rounds the final area answer and displays as a floating point number
    }

    fn perimeter(&self)-> f32 {
        (2.0 * self.radius * PI_VALUE).round() as f32  //rounds the final perimeter answer and displays as a floating point number
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }

    
} 


//implement Partial Eq
impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool { //eq for equals to
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool { //ne for not equals to
        !self.eq(other)
    }
}

impl PartialOrd for Circle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }
    // Provided methods
    //fn lt(&self, other: &Rhs) -> bool { ... }
    //fn le(&self, other: &Rhs) -> bool { ... }
    //fn gt(&self, other: &Rhs) -> bool { ... }
    //fn ge(&self, other: &Rhs) -> bool { ... }
}

//A conversion implementation into String
//Expects a string slice with radius and name, separated by commas
impl From<&'static str> for Circle {
    fn from(s: &'static str) -> Circle {
        let mut parts = s.split(',');
        let radius = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };

        Circle { radius, name: &name}
    }
}


#[derive(Default, Debug, Clone)]
struct Triangle { //a struct of type triangle with 2 fields: height and base
    height: f32,
    base: f32,
    name: &'static str,
}

impl Triangle {  //define default methods that will be available for only Triangles
    //default default() function. Will override derived default if any. 
    fn default() -> Self {
        Triangle {
            height: 1.0,
            base: 1.0,
            name: "default_Triangle_name",
        }
    }

    fn set_height(&mut self, height: f32) {
        self.height = height;
    }

    fn get_height(&self) -> f32 {
        self.height
    }

    fn set_base(&mut self, base: f32) {
        self.base = base;
    }

    fn get_base(&self) -> f32 {
        self.base
    }
}

impl Shape for Triangle {
    //Associated function used to create a new Shape
    type ConcreteShape = (f32, f32, & 'static str); //will be used to define that Triangle has height, breadth, and name

    fn new(dimensions: (f32, f32,& 'static str)) -> Self { //used to instantiate a Triangle with unique values other than the default
        Triangle{
            height: dimensions.0,
            base: dimensions.1,
            name: dimensions.2,
        }
    }

    fn area(&self) -> f32 {
        self.height * self.base
    }

    fn perimeter(&self)-> f32 {
        2.0 * (self.height + self.base)
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }

}

//implement Partial Eq
impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Triangle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }
    // Provided methods
    //fn lt(&self, other: &Rhs) -> bool { ... }
    //fn le(&self, other: &Rhs) -> bool { ... }
    //fn gt(&self, other: &Rhs) -> bool { ... }
    //fn ge(&self, other: &Rhs) -> bool { ... }
}


//A conversion implementation into String
//Expects a string slice with height, base, name, separated by commas
impl From<&'static str> for Triangle {
    fn from(s: &'static str) -> Triangle {
        let mut parts = s.split(',');
        let height = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let base = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };

        Triangle { height, base, name: &name}
    }
}

pub fn run2() {
    let rectangle1 = Rect::default(); //use the default values of Rect
    let circle1 = Circle::default(); //use the default values of Rect
    let triangle1 = Triangle::default(); //use the default values of Rect
    
    println!("Length of rectangle1: {}", rectangle1.length);
    println!("Width of the rectangle1:{}", rectangle1.width);
    println!("Name of the rectangle1:{}", rectangle1.name);

    let rectangle2 = Rect::new((1.0, 3.0, "Rectangle2")); //declare new rects with unique values
    let rectangle3 = Rect::from("4,5,Rectangle3,0");

    //Compare using PartialOrd
    let result1 = rectangle1.partial_cmp(&rectangle2);
    println!(" result1 = {:?}", result1);

    let result2 = rectangle1.le(&rectangle2);
    println!("result2 = {:?}", result2);

    //Compare using PartialEq
    let result3 = rectangle2.eq(&rectangle3);
    println!("result3 = {:?}", result3);

    let result4 = rectangle2.ne(&rectangle3);
    println!("result4 = {:?}", result4);
}