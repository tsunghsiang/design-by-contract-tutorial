use crate::debit_card::DebitCard;
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
    pub fn get_cash_reserve(&self) -> &i32 {
        &self.cash_reserves
    }

    #[requires(cash > 0, "deposit should be a positive value")]
    #[ensures(self.cash_reserves > old(self.cash_reserves), "cash reserves should be greater than its previous value after execution")]
    pub fn deposit(&mut self, card: &mut DebitCard, cash: i32) {
        if &self.bank_id == card.get_bank_id() {
            self.cash_reserves += cash.clone();
            card.deposit(cash.clone());
        } else {
            self.cash_reserves += cash.clone() + 15;
            card.deposit(cash.clone() - 15);
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
            self.cash_reserves -= cash.clone();
            self.cash_reserves += 15;
            card.withdraw(cash.clone() + 15);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::atm::ATM;
    use crate::debit_card::DebitCard;

    #[test]
    #[should_panic]
    fn given_bank_id_with_invalid_length_when_atm_init_then_should_panic() {
        let (bank_id, reserves) = ("1234", &0);
        let _atm = ATM::init(bank_id, reserves);
    }

    #[test]
    #[should_panic]
    fn given_bank_id_with_invalid_digit_when_atm_init_then_should_panic() {
        let (bank_id, reserves) = ("000", &0);
        let _atm = ATM::init(bank_id, reserves);
    }

    #[test]
    fn given_legal_bank_id_when_atm_init_then_should_not_panic() {
        let (bank_id, reserves) = ("006", &10);
        let atm = ATM::init(bank_id, reserves);
        assert_eq!("006", atm.get_bank_id());
        assert_eq!(&10, atm.get_cash_reserve());
    }

    #[test]
    #[should_panic]
    fn given_invalid_reserves_when_atm_init_then_should_panic() {
        let (bank_id, reserves) = ("812", &-20);
        let _atm = ATM::init(bank_id, reserves);
    }

    #[test]
    #[should_panic]
    fn given_cash_less_than_zero_when_deposit_then_should_panic(){
        let mut card = DebitCard::init("111111111111", &0, "812");
        let mut atm = ATM::init("812", &200000);
        atm.deposit(&mut card, -1000);
    }

    #[test]
    #[should_panic]
    fn given_cash_equals_zero_when_deposit_then_should_panic(){
        let mut card = DebitCard::init("111111111111", &0, "812");
        let mut atm = ATM::init("812", &200000);
        atm.deposit(&mut card, 0);
    }

    #[test]
    fn given_cash_greater_than_zero_when_deposit_then_should_not_panic(){
        let mut card = DebitCard::init("111111111111", &0, "812");
        let mut atm = ATM::init("812", &200000);
        atm.deposit(&mut card, 2000);
    }

    #[test]
    #[should_panic]
    fn given_cash_less_than_zero_when_withdraw_then_should_panic(){
        let mut card = DebitCard::init("111111111111", &0, "812");
        let mut atm = ATM::init("812", &200000);
        atm.withdraw(&mut card, -2000);
    }

    #[test]
    #[should_panic]
    fn given_cash_equals_zero_when_withdraw_then_should_panic(){
        let mut card = DebitCard::init("111111111111", &0, "812");
        let mut atm = ATM::init("812", &200000);
        atm.withdraw(&mut card, 0);
    }
    
    #[test]
    #[should_panic]
    fn given_reserves_less_than_withdrawn_cash_when_withdraw_then_should_panic(){
        let mut card = DebitCard::init("111111111111", &10000, "812");
        let mut atm = ATM::init("812", &1000);
        atm.withdraw(&mut card, 2000);
    }

    #[test]
    fn given_sufficient_reserves_when_withdraw_then_should_not_panic(){
        let mut card = DebitCard::init("111111111111", &10000, "812");
        let mut atm = ATM::init("812", &10000);
        atm.withdraw(&mut card, 2000);
        assert_eq!(&8000, card.get_balance());
        assert_eq!(&8000, atm.get_cash_reserve());
    }  
}
