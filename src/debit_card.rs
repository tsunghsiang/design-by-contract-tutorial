use contracts::ensures;
use contracts::invariant;
use contracts::requires;

pub struct DebitCard {
    account: String,
    balance: i32,
    bank_id: String,
}

#[invariant(self.balance >= 0)]
#[invariant(self.bank_id.len() == 3)]
#[invariant((0..10).contains(&self.bank_id.chars().nth(0).unwrap().to_digit(10).unwrap()))]
#[invariant((0..10).contains(&self.bank_id.chars().nth(1).unwrap().to_digit(10).unwrap()))]
#[invariant((1..10).contains(&self.bank_id.chars().nth(2).unwrap().to_digit(10).unwrap()))]
#[invariant(self.account.len() == 12)]
#[invariant((0..10).contains(&self.account.chars().nth(0).unwrap().to_digit(10).unwrap()))]
#[invariant((0..10).contains(&self.account.chars().nth(1).unwrap().to_digit(10).unwrap()))]
#[invariant((0..10).contains(&self.account.chars().nth(2).unwrap().to_digit(10).unwrap()))]
#[invariant((0..10).contains(&self.account.chars().nth(3).unwrap().to_digit(10).unwrap()))]
#[invariant((0..10).contains(&self.account.chars().nth(4).unwrap().to_digit(10).unwrap()))]
#[invariant((0..10).contains(&self.account.chars().nth(5).unwrap().to_digit(10).unwrap()))]
#[invariant((0..10).contains(&self.account.chars().nth(6).unwrap().to_digit(10).unwrap()))]
#[invariant((0..10).contains(&self.account.chars().nth(7).unwrap().to_digit(10).unwrap()))]
#[invariant((0..10).contains(&self.account.chars().nth(8).unwrap().to_digit(10).unwrap()))]
#[invariant((0..10).contains(&self.account.chars().nth(9).unwrap().to_digit(10).unwrap()))]
#[invariant((0..10).contains(&self.account.chars().nth(10).unwrap().to_digit(10).unwrap()))]
#[invariant((0..10).contains(&self.account.chars().nth(11).unwrap().to_digit(10).unwrap()))]
impl DebitCard {
    #[requires(account.len() == 12)]
    #[requires((0..10).contains(&account.chars().nth(0).unwrap().to_digit(10).unwrap()))]
    #[requires((0..10).contains(&account.chars().nth(1).unwrap().to_digit(10).unwrap()))]
    #[requires((0..10).contains(&account.chars().nth(2).unwrap().to_digit(10).unwrap()))]
    #[requires((0..10).contains(&account.chars().nth(3).unwrap().to_digit(10).unwrap()))]
    #[requires((0..10).contains(&account.chars().nth(4).unwrap().to_digit(10).unwrap()))]
    #[requires((0..10).contains(&account.chars().nth(5).unwrap().to_digit(10).unwrap()))]
    #[requires((0..10).contains(&account.chars().nth(6).unwrap().to_digit(10).unwrap()))]
    #[requires((0..10).contains(&account.chars().nth(7).unwrap().to_digit(10).unwrap()))]
    #[requires((0..10).contains(&account.chars().nth(8).unwrap().to_digit(10).unwrap()))]
    #[requires((0..10).contains(&account.chars().nth(9).unwrap().to_digit(10).unwrap()))]
    #[requires((0..10).contains(&account.chars().nth(10).unwrap().to_digit(10).unwrap()))]
    #[requires((0..10).contains(&account.chars().nth(11).unwrap().to_digit(10).unwrap()))]
    #[requires(balance >= &0)]
    #[requires(bank_id.len() == 3)]
    #[requires((0..10).contains(&bank_id.chars().nth(0).unwrap().to_digit(10).unwrap()))]
    #[requires((0..10).contains(&bank_id.chars().nth(1).unwrap().to_digit(10).unwrap()))]
    #[requires((1..10).contains(&bank_id.chars().nth(2).unwrap().to_digit(10).unwrap()))]
    pub fn init(account: &str, balance: &i32, bank_id: &str) -> Self {
        Self {
            account: account.to_string(),
            balance: balance.clone(),
            bank_id: bank_id.to_string(),
        }
    }
}

impl DebitCard {
    #[requires(cash > 0, "deposit should be greater than 0")]
    #[ensures(self.balance == old(self.balance) + cash, "deposit result equals cash pluses previous balance")]
    pub fn deposit(&mut self, cash: i32) {
        self.balance += cash;
    }

    #[requires(cash > 0, "withdraw should be greater than 0")]
    #[ensures(self.balance == old(self.balance) - cash, "withdraw result equals previous balance minuses balance")]
    pub fn withdraw(&mut self, cash: i32) {
        self.balance -= cash;
    }

    #[requires(!self.bank_id.is_empty(), "bank_id should be a 3-digit number")]
    pub fn get_bank_id(&self) -> &str {
        &self.bank_id
    }

    #[requires(!self.account.is_empty(), "account should be a 3-digit number")]
    pub fn get_account(&self) -> &str {
        &self.account
    }

    #[requires(self.balance >= 0, "balance should be a non-negative number")]
    pub fn get_balance(&self) -> &i32 {
        &self.balance
    }
}

#[cfg(test)]
mod tests {
    use crate::debit_card::DebitCard;
    #[test]
    fn given_legal_fields_provided_when_debit_card_init_then_should_not_panic() {
        let (account, balance, bank_id) = (String::from("123456789012"), 0, String::from("812"));
        let card = DebitCard::init(&account, &balance, &bank_id);
        assert_eq!("123456789012", card.get_account());
        assert_eq!(&0, card.get_balance());
        assert_eq!("812", card.get_bank_id());
    }

    #[test]
    #[should_panic]
    fn given_illegal_account_provided_when_debit_card_init_then_should_panic() {
        let (account, balance, bank_id) = (String::from("k23456789012"), 0, String::from("812"));
        let _card = DebitCard::init(&account, &balance, &bank_id);
    }

    #[test]
    #[should_panic]
    fn given_illegal_balance_provided_when_debit_card_init_then_should_panic() {
        let (account, balance, bank_id) = (String::from("523456789012"), -1, String::from("812"));
        let _card = DebitCard::init(&account, &balance, &bank_id);
    }

    #[test]
    #[should_panic]
    fn given_illegal_bank_id_provided_when_debit_card_init_then_should_panic() {
        let (account, balance, bank_id) = (String::from("k23456789012"), 0, String::from("000"));
        let _card = DebitCard::init(&account, &balance, &bank_id);
    }
}
