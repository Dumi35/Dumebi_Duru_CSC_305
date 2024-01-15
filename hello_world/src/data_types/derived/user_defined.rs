#![allow(dead_code)]
//use crate::data_types::derived;
//Dumebi Duru CSC 305

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

    /* let blue = Student{
        matricNo: ['a','a','a','a','a','a','a','a'],
        age: 19
    }; */

    
}

fn estimate_2_dp(number:f32)->f32 {  //rounds numbers to 2 decimal places
    let answer = (number * 100.0).round() / 100.0;
    answer
}



//This suppresses warnings when a given declared function is  not used.

use core::cmp::Ordering;
//use std::f32::consts::PI; //Used dor comparison of value sizes 

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

trait IntoRect {  //would be used in converting other shapes to rectangles
    fn into_rect(&self) -> Rect;
}
trait IntoCircle {  //would be used in converting other shapes to circles
    fn into_circle(&self) -> Circle;
}

trait IntoTriangle {  //would be used in converting other shapes to triangles
    fn into_triangle(&self) -> Triangle;
}

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
    fn eq(&self, other: &Self) -> bool { //check area and perimeter equalties for rectangle
        self.area() == other.area() && self.perimeter() == other.perimeter()
    }

    fn ne(&self, other: &Self) -> bool { // checks if 2 rectangles are not equals to each other
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



impl IntoCircle for Rect {  //convert the rectangle into a circle by comparing the areas and making r subject of formula i.e lh = PI*r^2
    fn into_circle(&self) -> Circle {
        Circle {
            radius: ((self.length* self.width)/PI_VALUE).powf(0.5),
            name: self.name
        }
    }
}

impl IntoTriangle for Rect {  //convert the rectangle into a triangle by comparing the areas and making r subject of formula i.e lh = (S*(S-a)*(S-b)*(S-c))^0.5
    fn into_triangle(&self) -> Triangle {
        Triangle {
            a: ((16.0 *(self.length*self.width).powf(2.0))/3.0).powf(0.25),
            b: ((16.0 *(self.length*self.width).powf(2.0))/3.0).powf(0.25),
            c: ((16.0 *(self.length*self.width).powf(2.0))/3.0).powf(0.25),
            name: self.name
        }
    }
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
    fn eq(&self, other: &Self) -> bool { //check area and perimeter equalities for circle
        self.area() == other.area() && self.perimeter() == other.perimeter()
    }

    fn ne(&self, other: &Self) -> bool { // checks if 2 circles are not equals to each other
        !self.eq(other)
    }
}

impl PartialOrd for Circle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }
}

impl IntoRect for Circle{ //convert circle into a rectangle
    fn into_rect(&self) -> Rect {
        Rect { 
            length: (PI_VALUE*self.radius.powf(2.0)).powf(0.5), 
            width: (PI_VALUE*self.radius.powf(2.0)).powf(0.5),
            name: self.name,
        }
    }
}

impl IntoTriangle for Circle {  //convert the circle into a triangle by comparing the areas and making r subject of formula i.e lh = (S*S-a*S-b*S-c)^0.5
    fn into_triangle(&self) -> Triangle {
        Triangle {
            a: ((16.0*PI_VALUE.powf(2.0)*self.radius.powf(4.0))/3.0).powf(0.25),
            b: ((16.0*PI_VALUE.powf(2.0)*self.radius.powf(4.0))/3.0).powf(0.25),
            c: ((16.0*PI_VALUE.powf(2.0)*self.radius.powf(4.0))/3.0).powf(0.25),
            name: self.name
        }
    }
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
    a: f32, //first side
    b: f32, //second side
    c:f32, //third side
    name: &'static str,
}

impl Triangle {  //define default methods that will be available for only Triangles
    //default default() function. Will override derived default if any. 
    fn default() -> Self {
        Triangle {
            a: 1.0,
            b: 1.0,
            c:1.0,
            name: "default_Triangle_name",
        }
    }

    fn set_a(&mut self, a: f32) {
        self.a = a;
    }

    fn get_a(&self) -> f32 {
        self.a
    }

    fn set_b(&mut self, b: f32) {
        self.b = b;
    }

    fn get_b(&self) -> f32 {
        self.b
    }
    fn set_c(&mut self, c: f32) {
        self.c= c;
    }

    fn get_c(&self) -> f32 {
        self.c
    }
}

impl Shape for Triangle {
    //Associated function used to create a new Shape
    type ConcreteShape = (f32, f32, f32, & 'static str); //will be used to define that Triangle has height, breadth, and name

    fn new(dimensions: (f32, f32, f32,& 'static str)) -> Self { //used to instantiate a Triangle with unique values other than the default
        Triangle{
            a: dimensions.0,
            b: dimensions.1,
            c: dimensions.2,
            name: dimensions.3,
        }
    }

    fn perimeter(&self)-> f32 {
        self.a + self.b + self.c
    }
    
    fn area(&self) -> f32 { //use Hero's formula for area
        let p:f32 = self.perimeter()/2.0; 
        estimate_2_dp((p *  (p -self.a) * (p -self.b) * (p -self.c)).powf(0.5)) //round the answer to 2 decimal places
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
    fn eq(&self, other: &Self) -> bool { //check area and perimeter equalties for triangle
        self.area() == other.area() && self.perimeter() == other.perimeter()
    }

    fn ne(&self, other: &Self) -> bool { // checks if 2 triangles are not equals to each other
        !self.eq(other)
    }
}

impl PartialOrd for Triangle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }
}

impl IntoRect for Triangle{ //convert triangle to rectangle by making l and w subjects of formula by comparing the areas
    fn into_rect(&self) -> Rect {
        Rect { 
            length: ((3.0 * self.a.powf(4.0))/16.0).powf(0.25), 
            width: ((3.0 * self.a.powf(4.0))/16.0).powf(0.25), 
            name: self.name,
        }
    }
}
impl IntoCircle for Triangle{ //convert triangle to rectangle by making radius subject of formula by comparing the areas
    fn into_circle(&self) -> Circle {
        Circle { 
            radius: ((3.0*self.a.powf(4.0))/(16.0*PI_VALUE)).powf(0.25),
            name: self.name,
        }
    }
}


//A conversion implementation into String
//Expects a string slice with a, b, c, name, separated by commas
impl From<&'static str> for Triangle {
    fn from(s: &'static str) -> Triangle {
        let mut parts = s.split(',');
        let a = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let b = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let c = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };

        Triangle { a, b, c, name: &name}
    }
}

pub fn run2() {
    let rectangle1 = Rect::default(); //use the default values of Rect
    let circle1 = Circle::default(); //use the default values of Rect
    let triangle1 = Triangle::default(); //use the default values of Rect
    
    println!("Length of rectangle1: {}", rectangle1.length);
    println!("Width of the rectangle1:{}", rectangle1.width);
    println!("Name of the rectangle1:{}", rectangle1.name);

    println!("\nRadius of circle1: {}", circle1.radius);
    println!("Name of the circle1:{}", circle1.name);

    println!("\nSide 1 of triangle1: {}", triangle1.a);
    println!("Side 2 of the triangle1:{}", triangle1.b);
    println!("Side 3 of the triangle1:{}", triangle1.c);
    println!("Name of the triangle1:{}", triangle1.name);


    let rectangle2 = Rect::new((1.5, 0.5, "Rectangle2")); //declare new rects with unique values
    let rectangle3 = Rect::from("4,5,Rectangle3");

    let circle2 = Circle::new((3.5, "Circle2")); //declare new circles with unique values
    let circle3 = Circle::from("13,Circle3");

    let triangle2 = Triangle::new((3.5, 4.22, 2.0, "triangle2")); //declare new triangles with unique values
    let triangle3 = Triangle::from("12,13,14,triangle3");

    println!("\nArea of triangle2: {}", triangle2.area());

    //Compare rectangles using PartialOrd
    println!("\nComparing rectangles");
    let result1 = rectangle1.partial_cmp(&rectangle2);
    println!("result1 = {:?}", result1);

    let result2 = rectangle1.le(&rectangle2);
    println!("result2 = {:?}", result2);

    //Compare using PartialEq
    let result3 = rectangle1.eq(&rectangle2);
    println!("result3 = {:?}", result3);

    let result4 = rectangle2.ne(&rectangle3);
    println!("result4 = {:?}", result4);

    //comparing circles
    println!("\nComparing circles");
    let result5 = circle1.partial_cmp(&circle2);
    println!("result5 = {:?}", result5);

    let result6 = circle1.ge(&circle2); //greater than or equals to
    println!("result6 = {:?}", result6);

    //Compare using PartialEq
    let result7 = circle1.eq(&circle2);
    println!("result7 = {:?}", result7);

    let result8 = circle2.ne(&circle3);
    println!("result8= {:?}", result8);

    //comparing triangles
    println!("\nComparing triangles");
    let result9 = triangle1.partial_cmp(&triangle2);
    println!("result9 = {:?}", result9);

    let result10 = triangle1.gt(&triangle2);
    println!("result10 = {:?}", result10);

    //Compare using PartialEq
    let result11 = triangle2.eq(&triangle2);
    println!("result11 = {:?}", result11);

    let result12 = triangle2.ne(&triangle3);
    println!("result12= {:?}", result12);

    //convert each shape to the others by equating area
    println!("\ntriangle2 to circle = {:?}", triangle2.into_circle());
    println!("triangle2 to rectangle = {:?}", triangle2.into_rect());

    println!("\ncircle2 to triangle = {:?}", circle2.into_triangle());
    println!("circle2 to rectangle = {:?}", circle2.into_rect());

    println!("\nrectangle2 to triangle = {:?}", rectangle2.into_triangle());
    println!("rectangle2 to circle = {:?}", rectangle2.into_circle());
}