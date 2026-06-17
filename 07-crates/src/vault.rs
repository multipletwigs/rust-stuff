use crate::math::fees;

#[derive(Debug, PartialEq)]
pub enum VaultError {
    ZeroAmount,
    InsufficientFunds,
}

pub struct Vault {
    pub owner: String,
    balance: u64,
}

impl Vault {
    pub fn new(owner: String) -> Vault {
        Vault { owner, balance: 0 }
    }

    pub fn deposit(&mut self, amount: u64) -> Result<u64, VaultError> {
        if amount == 0 {
            return Err(VaultError::ZeroAmount);
        }
        let net = amount - fees::fee_amount(amount);
        self.balance += net;
        Ok(self.balance)
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<u64, VaultError> {
        if amount == 0 {
            return Err(VaultError::ZeroAmount);
        }
        if amount > self.balance {
            return Err(VaultError::InsufficientFunds);
        }
        self.balance -= amount;
        Ok(self.balance)
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_takes_fee() {
        let mut v = Vault::new(String::from("zach"));
        assert_eq!(v.deposit(10_000), Ok(9_970));
    }

    #[test]
    fn cannot_overdraw() {
        let mut v = Vault::new(String::from("zach"));
        v.deposit(10_000).unwrap();
        assert_eq!(v.withdraw(999_999), Err(VaultError::InsufficientFunds));
    }
}
