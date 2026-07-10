struct Book {
    title: String,
    pages: u32,
}

/** QUESTIONS -- Thanks Meowy

1. is .iter() correct or .into_iter()?
We can investigate this by taking a look into the signature of the functions.

   impl<T> [T]
   pub const fn iter(&self) -> Iter<'_, T>
   `T` = `Book`
   Returns an iterator over the slice.
   The iterator yields all items from start to end.

So it takes in &self which is an immutable reference to itself, then returns a struct of
Iter<'_, T>. We can then go into Iter struct to check out its Iterator impl.

iterator! {struct Iter -> *const T, &'a T, const, {/* no mut */}, as_ref, each_ref, {

Then you get a macro which is a template for Iter struct's Iterator implementation, the type
is clear there, some *const T, &'a T, const ... yaddi yada but the important thing is it kinda
explains why the map is piped with &Book.

impl<T, A: Allocator> IntoIterator for Vec<T, A> {

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            let me = ManuallyDrop::new(self);
            let alloc = ManuallyDrop::new(ptr::read(me.allocator()));

If you check the into_iter type you can see it moves library into the function, then drop it
Idk what it exactly does but it feels like  bad choice because u still want the library var.

And since we wanna reuse the library var, if you need to use it you should borrow it instead.
So we should use iter.

2. what type would this return .max_by_key?

Straight forward answer is Option<&Book>, still trying to get the lifeline part of the type.
But looking at the type inside, you have Iter<'a, Book> struct but the Item is &T, so Option<Self::Item> shoud be Option<&Book>

impl Iterator for Iter<'a, T>    { type Item = &'a T; ... }

fn max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
where
    Self: Sized,
    F: FnMut(&Self::Item) -> B,

Interesting thing is the closure itself is |b: &&Book| still trying to wrap my head around
this but the F in the type is FnMut(&Self::Item) So &&Book, apparently closure generator but
maybe i will come across more concrete examples

3. is unwrap correct?
Yeah it should be, max by key returns none when the iterator itself is empty. First next() is None.
*/
pub fn exercise_1() {
    // counting heap-allocations
    // 1. instantiating the vec is one heap allocation
    // 2. then you have each item inside the vec, so it's really three more allocations for the
    //    string since it can grow anywhere?
    let library = vec![
        Book {
            title: String::from("Rust in Action"),
            pages: 400,
        },
        Book {
            title: String::from("The Book"),
            pages: 560,
        },
        Book {
            title: String::from("Programming Rust"),
            pages: 700,
        },
    ];

    let total: u32 = library.iter().map(|b| b.pages).sum();
    let longest = library.iter().max_by_key(|b| b.pages).unwrap();

    println!("Total pages: {}", total);
    println!("Longest book: {}", longest.title);
}
