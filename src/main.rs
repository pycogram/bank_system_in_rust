use bank_system::account::{
    Account, 
    AccountTrait, 
    match_bal
};

fn main() {
    let mut acc: Account<i32, String, f64> = Account::new(101, "daniel".to_string(), 12.0);
    
    let bal: Result<Option<f64>, String> = acc.check_balance();
    match_bal(&bal);

    let deposit = acc.deposit(20.00); 
    println!("{:?}", deposit.unwrap());

    let withdraw = acc.widthrew(9.00);
    println!("{:?}", withdraw.unwrap());

    let bal = acc.check_balance();
    match_bal(&bal);
}

