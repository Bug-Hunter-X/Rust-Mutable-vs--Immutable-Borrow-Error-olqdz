fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y += 1; // Modifying x through y
    println!("x = {}", x); // x is now 6

    // Correct approach: create a new immutable reference after the modification
    let z = &x;
    println!("x = {}", *z);
}