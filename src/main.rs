mod atm;
mod debit_card;

use debit_card::DebitCard;
use atm::ATM;

fn main() {
    // interaction between debit card of TC Bank & ATM of CITI Bank 
    let mut citi_bank_atm = ATM::init("812", &1000000);
    let mut tc_bank_card = DebitCard::init("123456783946", &200000, "006");
    citi_bank_atm.deposit(&mut tc_bank_card, 1000);
    assert_eq!(&1001015, citi_bank_atm.get_cash_reserve());
    assert_eq!(&200985, tc_bank_card.get_balance());
    citi_bank_atm.withdraw(&mut tc_bank_card, 5000);
    assert_eq!(&996030, citi_bank_atm.get_cash_reserve());
    assert_eq!(&195970, tc_bank_card.get_balance());    
    // interaction between debit card of TC Bank & ATM of TC Bank
    let mut tc_bank_atm = ATM::init("006", &1000000);
    tc_bank_atm.deposit(&mut tc_bank_card, 40000);
    assert_eq!(&1040000, tc_bank_atm.get_cash_reserve());
    assert_eq!(&235970, tc_bank_card.get_balance());
    tc_bank_atm.withdraw(&mut tc_bank_card, 30000);
    assert_eq!(&1010000, tc_bank_atm.get_cash_reserve());
    assert_eq!(&205970, tc_bank_card.get_balance());
}
