// Calculations included directly in patina::income.

use super::income_parameters::Pay;

pub fn calc_gross_income(budget: &mut crate::Budget) -> f64 {
    let mut income = 0.0;
    for job in &budget.income_params.jobs {
        match job.pay {
            Pay::Hourly(rate) => {}
            Pay::Salary(rate) => {}
        }
    }
    income
}
