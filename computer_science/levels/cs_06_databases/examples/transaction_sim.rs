use std::collections::HashMap;

#[derive(Debug)]
struct Accounts {
    balances: HashMap<&'static str, i64>,
}

impl Accounts {
    fn transfer(
        &mut self,
        from: &'static str,
        to: &'static str,
        amount: i64,
    ) -> Result<(), String> {
        let from_balance = *self
            .balances
            .get(from)
            .ok_or_else(|| format!("missing account: {from}"))?;
        let to_balance = *self
            .balances
            .get(to)
            .ok_or_else(|| format!("missing account: {to}"))?;

        if from_balance < amount {
            return Err("insufficient funds".to_string());
        }

        self.balances.insert(from, from_balance - amount);
        self.balances.insert(to, to_balance + amount);
        Ok(())
    }
}

fn main() {
    let mut accounts = Accounts {
        balances: HashMap::from([("alice", 100), ("bob", 50)]),
    };

    println!("before: {:?}", accounts);
    println!("transfer: {:?}", accounts.transfer("alice", "bob", 30));
    println!("after: {:?}", accounts);
    println!(
        "failed transfer: {:?}",
        accounts.transfer("alice", "carol", 10)
    );
    println!("after failed transfer: {:?}", accounts);
}
