use generics::lifetimes::{Vault, VaultView, first_word, richer};
use generics::math::largest;
use generics::pairs::Pair;

fn main() {
    // 1. GENERIC FUNCTION: same fn, two different element types
    let balances = vec![40_000u64, 9_970, 120_000, 3_300];
    println!("largest balance: {}", largest(&balances));

    let fees = vec![0.30f64, 0.05, 0.25];
    println!("largest fee:     {}", largest(&fees)); // T = f64 now

    // 2. GENERIC STRUCT + bounded method
    let pair = Pair { a: 50_000u64, b: 49_850 };
    println!("larger of pair:  {}", pair.larger());

    // 3. LIFETIMES: return a reference picked from the inputs
    let alice = Vault::new("alice", 120_000);
    let bob = Vault::new("bob", 49_850);
    let winner = richer(&alice, &bob);
    println!("richer vault:    {} ({})", winner.owner, winner.balance);

    // 4. a struct that holds a borrow
    let view = VaultView { owner: &alice.owner };
    println!("{}", view.label());

    // 5. ZERO-COPY: the returned &str points INTO `sentence`, nothing allocated
    let sentence = String::from("hello world");
    println!("first word:      {}", first_word(&sentence));
}
