// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    //let vec0 = Vec::new(); // #1
    let mut vec0 = Vec::new(); // #2

    //let mut vec1 = fill_vec(vec0.clone()); // #1
    //let mut vec1 = fill_vec(&mut vec0); // #2
    fill_vec(&mut vec0); // #3

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // vec1.push(88); // #1-2
    vec0.push(88); //#3

    //println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1); // #1-2
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0); // #3
}

//fn fill_vec(vec: Vec<i32>) -> Vec<i32> { // #1
//fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {// #2
fn fill_vec(vec: &mut Vec<i32>) -> () {
    // #3
    //let mut vec = vec; // #1
    //let vec = vec; // #2

    vec.push(22);
    vec.push(44);
    vec.push(66);

    //vec.to_vec() // #1-2
}
