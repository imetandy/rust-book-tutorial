fn main() {
    // will cause program to panic. 
    // panic!("Crash and burn!");

    let v = vec![1,2,3];
    v[99];
    // error here will be a panic: 
    // index out of bounds: the len is 3 but the index is 99
}
