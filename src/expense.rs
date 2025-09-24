pub struct Expense {
    name: String,
    amount: i32
}

impl Expense {
    fn new(name: String, amount: i32) -> Self {
        Self { name, amount }
    }
}

pub fn total_expense(expense: &Vec<Expense>) -> i32 {
    let mut tmp = 0;

    for exp in expense {
        tmp += exp.amount;
    }

    tmp
}