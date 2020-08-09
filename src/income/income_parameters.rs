pub struct IncomeParameters {
    // Known Parameters
    pub jobs: Vec<Job>,

    // Calculated Parameters
    pub c_gross_income: Result<f64, &'static str>,
}

impl IncomeParameters {
    pub fn new() -> IncomeParameters {
        IncomeParameters {
            // Known Parameters
            jobs: vec![Job {
                pay: Pay::Salary(0.0),
                weekly_hours: 0.0,
                overtime_multiplier: 0.0,
                overtime_after: 0.0,
                doubletime_after: 0.0,
                day_roadwork_differential: 0.0,
                overnight_roadwork_differential: 0.0,
                per_diem: 0.0,
                overnight_per_diem: 0.0,
                percent_day_roadwork: 0.0,
                percent_overnight_roadwork: 0.0,
                paid_days_off: 0,
                average_yearly_bonus: 0.0,
            }],

            // Calculated Parameters
            c_gross_income: Err("Gross income has not been calculated"),
        }
    }
}

impl Default for IncomeParameters {
    fn default() -> Self {
        IncomeParameters::new()
    }
}

pub struct Job {
    pub pay: Pay,
    pub weekly_hours: f32,
    pub overtime_multiplier: f32,
    pub overtime_after: f32,
    pub doubletime_after: f32,
    pub day_roadwork_differential: f32,
    pub overnight_roadwork_differential: f32,
    pub per_diem: f32,
    pub overnight_per_diem: f32,
    pub percent_day_roadwork: f32,
    pub percent_overnight_roadwork: f32,
    pub paid_days_off: i32,
    pub average_yearly_bonus: f64,
}

#[derive(Debug)]
pub enum Pay {
    Salary(f64),
    Hourly(f64),
}
