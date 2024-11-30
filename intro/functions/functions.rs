/*

    - Must declare the types of their params
    - A new scope block created with {} is an expression (and a syntactic scope)

*/

fn main() {
    let y = {
        let x = 3;
        //Note no semi colon, expressions do not include ending semicolons
        x + 1
    };
    println!("y: {y}");

    let x = five();
    println!("x: {x}");

}

/*

    - Return values, do not name but declare type w/ ->
    - Can return with return keyword, or just return last expression implicitly

*/
fn five() -> i32 {
    5
}