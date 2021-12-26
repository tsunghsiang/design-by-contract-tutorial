use crate::debit_card::DebitCard;
use contracts::ensures;
use contracts::invariant;
use contracts::requires;

pub struct ATM {
    bank_id: String,
    cash_reserves: i32,
}

#[invariant(self.cash_reserves >= 0)]
#[invariant(self.bank_id.len() == 3)]
#[invariant((0..10).contains(&self.bank_id.chars().nth(0).unwrap().to_digit(10).unwrap()))]
#[invariant((0..10).contains(&self.bank_id.chars().nth(1).unwrap().to_digit(10).unwrap()))]
#[invariant((1..10).contains(&self.bank_id.chars().nth(2).unwrap().to_digit(10).unwrap()))]
impl ATM {
    #[requires(id.len() == 3)]
    #[requires((0..10).contains(&id.chars().nth(0).unwrap().to_digit(10).unwrap()))]
    #[requires((0..10).contains(&id.chars().nth(1).unwrap().to_digit(10).unwrap()))]
    #[requires((1..10).contains(&id.chars().nth(2).unwrap().to_digit(10).unwrap()))]
    #[requires(reserves >= &0)]
    #[ensures(id == ret.get_bank_id())]
    #[ensures(reserves == ret.get_cash_reserve())]
    pub fn init(id: &str, reserves: &i32) -> Self {
        Self {
            bank_id: id.to_string(),
            cash_reserves: reserves.clone(),
        }
    }
}

impl ATM {
    #[requires(!self.bank_id.is_empty(), "bank_id should be a 3-digit number")]
    fn get_bank_id(&self) -> &str {
        &self.bank_id
    }

    #[requires(self.cash_reserves >= 0, "cash_reserves should be a non-negative number")]
    fn get_cash_reserve(&self) -> &i32 {
        &self.cash_reserves
    }

    #[requires(cash > 0, "deposit should be a positive value")]
    #[ensures(self.cash_reserves > old(self.cash_reserves), "cash reserves should be greater than its previous value after execution")]
    pub fn deposit(&mut self, card: &mut DebitCard, cash: i32) {
        if &self.bank_id == card.get_bank_id() {
            self.cash_reserves += cash.clone();
            card.deposit(cash.clone())
        } else {
            self.cash_reserves += cash.clone() + 15;
            card.deposit(cash.clone() - 15)
        }
    }

    #[requires(cash > 0, "withdraw should be a positive value")]
    #[requires(cash <= self.cash_reserves, "withdrawn values should be less than/equal to current cash reserves of the ATM")]
    #[ensures(self.cash_reserves < old(self.cash_reserves), "cash reserves should be less than its previous value after execution")]
    pub fn withdraw(&mut self, card: &mut DebitCard, cash: i32) {
        if &self.bank_id == card.get_bank_id() {
            self.cash_reserves -= cash.clone();
            card.withdraw(cash.clone());
        } else {
            self.cash_reserves -= cash.clone() + 15;
            card.withdraw(cash.clone() - 15)
        }
    }
}
