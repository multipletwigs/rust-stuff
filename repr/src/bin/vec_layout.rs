use std::mem::size_of;

fn main() {
    let mut v = vec![(44, "a"), (55, "b")];
    v.push((66, "c")); // <-- HERE (len = 3, cap = 4, so every field is distinguishable)

    println!("handle size: {} bytes", size_of::<Vec<(i32, &str)>>());
    println!("as_ptr() = {:p}, len = {}, cap = {}\n", v.as_ptr(), v.len(), v.capacity());

    // Read the 24-byte handle as three raw usize words, in memory order.
    let words: [usize; 3] = unsafe { std::mem::transmute_copy(&v) };

    for (i, w) in words.iter().enumerate() {
        let label = if *w == v.as_ptr() as usize {
            "<- this word is ptr"
        } else if *w == v.len() {
            "<- this word is len"
        } else if *w == v.capacity() {
            "<- this word is cap"
        } else {
            "<- ???"
        };
        println!("offset {:2}: {:#018x} {}", i * 8, w, label);
    }
}
