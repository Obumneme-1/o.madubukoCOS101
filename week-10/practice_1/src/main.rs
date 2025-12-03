fn main() {
    let v = vec![101, 250, 330, 400];
    // vector v owns the object in heap

    // only a single variable hold the given heap memory at a given time
    let v2 = v;
    // here two variable owns the heap value,
    // two pointers to the same content is not allowed in rust
    // rust is very samrt in terms of memory access, so it detects a race condition
    // as two variables point to the same heap

    println!("{:?}",v2);

}
