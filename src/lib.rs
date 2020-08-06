pub struct Budget {
    pub income: f64,
    pub age: i32,
}

impl Budget {
    pub fn new (income: f64, age: i32) -> Budget {
        Budget {
            income,
            age,
        }
    }
}
