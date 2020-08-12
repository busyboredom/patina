// Calculations included directly in patina::income.

use super::income_parameters::Pay;

pub fn calc_gross_income(budget: &mut crate::Budget) -> f64 {
    let mut income = 0.0;
    let params = &budget.income_params;
    for job in &params.jobs {
        match job.pay {
            Pay::Hourly(rate) => {
                income += f64::from(job.weekly_hours.max(job.overtime_after)) * rate;
            }
            Pay::Salary(rate) => {}
        }
    }
    budget.income_params.c_gross_income = Ok(income);
    income
}
