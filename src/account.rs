
use std::fmt::Display;
use std::ops::{AddAssign, SubAssign};

type STRINGT = String;
type FLOATT = f64;

pub trait AccountTrait<T, U, V>{
    fn deposit(self: &mut Self, amount: FLOATT) -> Result<STRINGT, STRINGT>;
    fn check_balance(self: &Self) -> Result<Option<V>, STRINGT>;
    fn widthrew(self: &mut Self, amount: FLOATT) -> Result<STRINGT, STRINGT>;
}
pub struct Account<T, U, V>{
    pub id: T,
    pub name: U,
    pub balance: V
}
impl<T, U, V> Account<T, U, V> 
where 
    T: Display + Copy, 
    U: Display, 
    V: Display + Copy
{
    pub fn new(id: T, name: U, balance: V) -> Self {
        println!("account creation was successful.");
        Self{id, name, balance}
    }
}
impl<T, U, V> AccountTrait<T, U, V> for Account<T, U, V> 
where 
T: Display + Copy, 
U: Display, 
V: Display + Copy + AddAssign + SubAssign + PartialOrd + From<f64>
{
    fn deposit(self: &mut Self, amount: FLOATT) -> Result<STRINGT, STRINGT>{
        let x: Result<STRINGT, STRINGT> = if amount < 1f64{
            Err("minimum deposit amount is $1.".to_string())
        } else {
            self.balance += V::from(amount);
            Ok(format!("${amount} deposited succesfully. new balance is ${}.", self.balance))
        };
        x
    }
    fn check_balance(self: &Self) -> Result<Option<V>, STRINGT>{
        let converted = self.balance;
        let y = if converted < V::from(1f64){
            Err("Account is empty".to_string())
        } else {
            Ok(Some(converted))
        };
        y
    }
    fn widthrew(self: &mut Self, amount: FLOATT) -> Result<STRINGT, STRINGT>{
        let converted = self.balance;
        let z = if converted < V::from(amount) {
            Err("Insufficient funds.".to_string())
        } else {
            self.balance -= V::from(amount);
            Ok(format!("${} has been withdrawn succesfully. new balance is ${}.", amount, self.balance))
        };
        z
    }
}

pub fn match_bal(bal: &Result<Option<f64>, String>){
    let x = match bal{
        Ok(val) => println!("account balance: ${}", val.unwrap()),
        Err(err) => eprintln!("{}", err)
    };
    x
}