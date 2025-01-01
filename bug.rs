fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let ptr = vec.as_ptr();

    // Danger zone: the vector's memory is freed here!
    drop(vec);

    // Reading from ptr is now undefined behavior!
    println!("Value at ptr: {}", unsafe { *ptr });
}