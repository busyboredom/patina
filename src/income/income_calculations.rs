// Calculations included directly in patina::income.

use super::income_parameters::{Job, Pay};

pub fn calc_gross_income(budget: &mut crate::Budget) -> f64 {
    let mut income = 0.0;
    let params = &budget.income_params;
    for job in &params.jobs {
        match job.pay {
            Pay::Hourly(rate) => {
                income += calc_weekly_from_hourly(job) * 52.0; // 52 weeks in a year.
            }
            Pay::Salary(rate) => {
                income += rate;
            }
        }
    }
    budget.income_params.c_gross_income = Ok(income);
    income
}

fn calc_weekly_from_hourly(job: &Job) -> f64 {
    let mut income = 0.0;
    if let Pay::Hourly(rate) = job.pay {
        // Calc base pay.
        income += f64::from(job.weekly_hours) * rate;

        // Add overtime adjustment.
        if let Some(overtime_start) = job.overtime_after {
            income += f64::from(
                (job.weekly_hours - overtime_start).max(0.0) * (job.overtime_multiplier - 1.0),
            ) * rate;
        }

        // Add doubletime adjustment.
        if let Some(doubletime_start) = job.doubletime_after {
            // If overtime exists:
            if let Some(overtime_start) = job.overtime_after {
                income += f64::from(
                    (job.weekly_hours - overtime_start - doubletime_start)
                        * (2.0 - job.overtime_multiplier),
                ) * rate;
            } else {
                // If there wasn't overtime:
                income += f64::from((job.weekly_hours - doubletime_start) * (2.0 - 1.0)) * rate;
            }
        }
    } else {
        panic!("Expected hourly wage, got salary.");
    }
    income
}
