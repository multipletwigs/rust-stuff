fn main() {
    let mut empty_str = String::new();
    empty_str.push('a');
    println!("Address of empty_str, {:p}", empty_str.as_ptr());

    let mut s = String::new();
    for c in "hello world, this is a test".chars() {
        s.push(c);
        println!(
            "len = {:2}, cap = {:2}, ptr = {:p}",
            s.len(),
            s.capacity(),
            s.as_ptr()
        );
    }

    let mut v = vec![(44, "a"), (55, "b")];
    println!("before: len = {}, cap = {}", v.len(), v.capacity());
    v.push((66, "c"));
    println!("after:  len = {}, cap = {}", v.len(), v.capacity());

    use std::mem::size_of;

    #[repr(C)] // repr(C) = "keep my declaration order", like C does
    struct InOrder {
        a: u8,
        b: u32,
        c: u8,
    }

    struct Shuffled {
        a: u8,
        b: u32,
        c: u8,
    } // default repr(Rust): compiler may reorder

    println!("declaration order: {} bytes", size_of::<InOrder>());
    println!("compiler's order:  {} bytes", size_of::<Shuffled>());
}
