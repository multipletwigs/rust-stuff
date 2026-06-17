use super::BPS_DENOMINATOR;

pub const DEPOSIT_FEE_BPS: u64 = 30;

pub fn fee_amount(amount: u64) -> u64 {
    amount * DEPOSIT_FEE_BPS / BPS_DENOMINATOR
}
