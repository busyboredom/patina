pub struct IncomeParameters {
    pub salary: f64,
}

impl IncomeParameters {
    pub fn new() -> IncomeParameters {
        IncomeParameters { salary: 34_000.00 }
    }
}

impl Default for IncomeParameters {
    fn default() -> Self {
        IncomeParameters::new()
    }
}
