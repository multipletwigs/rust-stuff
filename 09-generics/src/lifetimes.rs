//! LIFETIMES: labels the borrow checker uses to prove a reference never
//! outlives the data it points at. They change nothing at runtime.

pub struct Vault {
    pub owner: String, // OWNED -> Vault needs no lifetime
    pub balance: u64,
}

impl Vault {
    pub fn new(owner: &str, balance: u64) -> Vault {
        Vault { owner: owner.to_string(), balance }
    }
}

// Returns a reference chosen from the inputs, so the result shares lifetime
// `'a` with BOTH inputs: it is valid only as long as the shorter-lived input.
// Delete every `'a` and this stops compiling.
pub fn richer<'a>(a: &'a Vault, b: &'a Vault) -> &'a Vault {
    if a.balance >= b.balance { a } else { b }
}

// A struct that BORROWS a name it does not own -> it must carry a lifetime:
// a `VaultView<'a>` may not outlive the `&str` it points at.
pub struct VaultView<'a> {
    pub owner: &'a str,
}

impl<'a> VaultView<'a> {
    pub fn label(&self) -> String {
        format!("view of {}'s vault", self.owner)
    }
}

// ZERO-COPY: returns a slice that points INTO `s` — no new String is allocated.
// A &str is just (pointer, length); `&s[..i]` is a new (pointer, length) onto the
// SAME bytes. You wrote no lifetime, but elision rule #2 (one input reference ->
// its lifetime flows to the output) makes the real signature:
//     fn first_word<'a>(s: &'a str) -> &'a str
// so the returned slice can never outlive the string it borrows from.
pub fn first_word(s: &str) -> &str {
    for (i, &b) in s.as_bytes().iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    s
}

// ─────────────────────────────────────────────────────────────────────────────
// DANGLING REFERENCE — uncomment `dangle` and run `cargo build`.
// You'll get error[E0597]: `sentence` does not live long enough.
// This is the borrow checker proving zero-copy is SAFE: it won't let `word`
// outlive the data it points into.
//
pub fn dangle() {
    let word;
    {
        let sentence = String::from("hello world");
        word = first_word(&sentence); // word borrows sentence's bytes
    } // <- sentence dropped here, its bytes are freed
    println!("{word}"); // word would point at freed memory
}
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn richer_returns_bigger_balance() {
        let a = Vault::new("a", 100);
        let b = Vault::new("b", 200);
        assert_eq!(richer(&a, &b).owner, "b");
    }

    #[test]
    fn first_word_is_a_slice_into_input() {
        let s = String::from("hello world");
        assert_eq!(first_word(&s), "hello");
        assert_eq!(first_word("solana"), "solana"); // no space -> whole string
    }
}
