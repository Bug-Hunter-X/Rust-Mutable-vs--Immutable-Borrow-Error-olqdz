fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x;  // z is an immutable reference to x

    *y += 1; // Modifying x through y
    println!("x = {}", x); // x is now 6

    // This line will cause a compile-time error because z is an immutable reference and we are trying to modify x.
    //*z += 1; 
    println!("x = {}", x);
}