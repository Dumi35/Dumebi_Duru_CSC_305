use std::mem;
const AGES: [i32; 2] = [20, 30];
const MY_TUPLE: (i32, f64, &str, &str, i64) = (42, 3.14, "hello", "friend",89);
const MY_TUPLE2: (i32, i32) = (42, 46);

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

pub fn run(){
    AGES.iter().for_each(|&number| {
        println!("{}", number);
    });

    /* for element in MY_TUPLE.0..= MY_TUPLE.4{
        println!("Element: {:#?}", element);
    } */

    

    
    println!("{:?}",AGES);
    println!("{:?}",MY_TUPLE);
    
    let array: Box<[i32]> = Box::new([1, 2, 3,4,5,9,10,205]);

    let slice: &[i32] = &array[0..6];
    println!("{:?}",slice);

    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0.
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1 .. 4]);

    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

}
