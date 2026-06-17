use mini_vault::Vault;
use mini_vault::vault::VaultError;
use mini_vault::math::fees;

fn main() {
    let mut vault = Vault::new(String::from("zach"));

    println!("fee on 50_000 lamports: {}", fees::fee_amount(50_000));

    match vault.deposit(50_000) {
        Ok(balance) => println!("deposited, balance is now {balance}"),
        Err(e) => println!("deposit failed: {e:?}"),
    }

    match vault.withdraw(1_000_000) {
        Ok(balance) => println!("withdrew, balance is now {balance}"),
        Err(VaultError::InsufficientFunds) => println!("nice try — insufficient funds"),
        Err(e) => println!("withdraw failed: {e:?}"),
    }

    println!("final balance: {}", vault.balance());
}
