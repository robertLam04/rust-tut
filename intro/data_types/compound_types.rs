fn main() {

    /*
    Tuple type:
        - general way of grouping values with variety of types into one
        - fixed length
        - acceess with . or pattern matching
        - can be mutable
    */
    let tup: (i32, f64, u8) = (500, 6.2, 2);

    let (x,y,z) = tup;

    let z = tup.2;

    println!("y: {y}");

    /*
    Array type:
        - every element must have same type
        - have a fixed length
        - stored on stack instead of heap
        - syntax: [x; y] declares an array with y copies of value x
    */
    let a = [1,2,3,4,5];
    let first = a[0];

    println!("first: {first}");

}