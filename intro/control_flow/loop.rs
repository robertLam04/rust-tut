/*

    - Loops can be used to retry operations that may fail. Add value you
    want to return after the break expression used to stop the loop
    - Can label loops for readibility: (eg 'counting_up: loop {})
    - 

*/

fn main() {

    // Return from a loop
    let mut counter = 0;

    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter;
        }
    };

    println!("result: {result}");

    // for loop:
    let a = [10,20,30,40,50];

    for element in a {
        println!("{element}");
    }

}