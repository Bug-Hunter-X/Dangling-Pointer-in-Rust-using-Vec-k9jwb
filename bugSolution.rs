fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Correct approach: keep the vector in scope!
    let ptr = vec.as_ptr();

    unsafe {
        println!("Value at ptr: {}", *ptr);
    }
}