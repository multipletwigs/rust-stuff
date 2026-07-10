use std::fmt::Display;

pub trait Priced {
    fn price(&self) -> u32;
}

pub struct Grocery {
    pub name: String,
    price: u32,
}

impl Priced for Grocery {
    fn price(&self) -> u32 {
        self.price
    }
}

impl Display for Grocery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (${})", self.name, self.price)
    }
}

impl PartialOrd for Grocery {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.price.partial_cmp(&other.price)
    }
}

impl PartialEq for Grocery {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

/**
 *
1. why does largest require T: Copy but cheapest does not?

I'm gusssing largest need copy just by looking at type we're returning the value
so T needs to impl Copy trait for the return type to make sense? also largest
doesn't need explicit lifelines because the function doessn't need to know how long
list lives?

2. what does the shared 'a on &'a [T] and &'a T do?
It tells the compiler that the return reference to T lives as long as 'a which is tagged
to the input [T] but in this case it feels implicit and not needed because there's only one
return so implicitly the return lives as long as the first argument

3. what would the compiler say if cheapest returned T instead of &T?
To return T means u need to implement Copy on T. and you can't write the type as items: [T] because T can be anything and not known at compile time. T is just something that implements Priced.

 *
 *
*/

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list {
        if item > max {
            max = item;
        }
    }
    max
}

fn cheapest<T: Priced>(items: &[T]) -> &T {
    let mut best = &items[0];
    for item in items {
        if item.price() < best.price() {
            best = item;
        }
    }
    best
}

pub fn exercise_2() {
    let priced_items = vec![
        Grocery {
            name: String::from("Apple"),
            price: 1090,
        },
        Grocery {
            name: String::from("Banana"),
            price: 100,
        },
    ];

    let prices = vec![123, 345];

    let cheapest_item = cheapest(&priced_items);
    let most_expensive = largest(&prices);

    println!("The cheapest item is {}", cheapest_item);
    println!("The largest number is {}", most_expensive);
}
