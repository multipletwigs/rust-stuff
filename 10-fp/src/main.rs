use fp::shirt::{Inventory, ShirtColor};

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user1_pref = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user1_pref);
    println!(
        "The user with preference {:?} gets {:?}",
        user1_pref, giveaway1
    );
}
