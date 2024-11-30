/*
    Source: https://rust-book.cs.brown.edu/ch04-03-fixing-ownership-errors.html

    - Safety is the absence of undefined behaviour
    - Ownership is mostly just heap management
    - Stack holds data associated with a specific function
    - Heap holds data that can outlive a function
    - Helpful to try and think about what the borrow checker is doing, think in terms of permissions
    Boxes:
        - Construct called Box for data on heap
        - Does not permit manual memory management
        - Automatically deallocates a box's heap memory after the function
        - if a variable owns a box, when rust deallocates the variables frame,
          rust deallocates the box's heap memor
        - when a is bound to Box::new([0; 10000]) we say a owns the box
        - if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move
            - use .clone() to get around this
    - Pointers:
        - allows for aliasing, accessing the same data through different vars

    - References (borrowing):
        - Kind of pointer
        - denoted by &var or &mut var 
        - dereferencing (*)
        - rust implicitly inserts derefs and ref in some cases
        - Note:
            - Resizing vectors will deallocate the prev array and allocate a new bigger one
            - References pointing to the prev array may be left pointing to invalid memory

    - Pointer safety principle:
        - Data should never be aliased and mutated at the same time
        - Data must outlive any references to it
    
    - Rules of thumb:
        - Do not pass mutable references as inputs to functions unless it would be expected 
        - It is very rare for Rust functions to take ownership of heap-owning data structures like Vec and String
        - If a value does not own heap data, it can be copied without moving ownership (except mutable references)

    - Slices:
        - Special kind of references that are 'fat' pointers, or pointers with metadata
        - Metadata is the length of the slice
        - eg let hello: &str = &s[0..5]; (0: start index, 5: one more than last position in slice)

*/

fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} {}", m1, m2);


    let name = vec![String::from("John"), String::from("Doe")];
    //Gives ownership of name to the function
    let full = stringify_name_with_title(name);
    println!("{full}");
    // println!("{:?}", name); // This would result in a compile-time error!
}

// g1, g2 points to m1, m2. Then g1 and g2 are deallocated after greet, not m1 and m2
// m1 owns "Hello" Since g1 doesnt own anything, so after greet ends no heap data is deallocated
// Only the stack frame for greet diappears
fn greet(g1: &String, g2: &String) { // note the ampersands
    println!("{} {}!", g1, g2);
}

//Takes complete ownership
//In rust, try and reach a careful balance of asking for the right level of permissions
fn stringify_name_with_title(mut name: Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

//Correct solution
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}

//Or, slightly more efficiently
fn stringify_name_with_title(name: &Vec<String>) -> String {
    //name.join copies data in name into full
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

// This will not compile because dst loses write permissions after being borrowed by largest
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = 
      dst.iter().max_by_key(|s| s.len()).unwrap();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}

// The following solutions all have one thing in common: 
// shortening the lifetime of borrows on dst to not overlap with a mutation to dst

//Clone
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}

// Perform length comparisons first then mutate dst
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
    let to_add: Vec<String> = 
        src.iter().filter(|s| s.len() > largest.len()).cloned().collect();
    dst.extend(to_add);
}

// Copy out just the largest length, not the actual string
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

// Using slice exmaple:
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}





