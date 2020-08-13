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
            jobs: vec![],

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
    pub weekly_workdays: f32,
    pub overtime_multiplier: f32,
    pub overtime_after: Option<f32>,
    pub doubletime_after: Option<f32>,
    pub day_roadwork_differential: f32,
    pub overnight_roadwork_differential: f32,
    pub daytrip_per_diem: f32,
    pub overnight_per_diem: f32,
    pub percent_day_roadwork: f32,
    pub percent_overnight_roadwork: f32,
    pub paid_holidays: f32,
    pub anticipated_missed_days: f32,
    pub average_yearly_bonus: f64,
    pub employer_401k_match: f32,
    pub employer_401k_match_limit: f32,
}

impl Job {
    pub fn new() -> Job {
        Job {
            pay: Pay::Hourly(0.0),
            weekly_hours: 40.0,
            weekly_workdays: 5.0,
            overtime_multiplier: 1.5,
            overtime_after: None,
            doubletime_after: None,
            day_roadwork_differential: 0.0,
            overnight_roadwork_differential: 0.0,
            daytrip_per_diem: 0.0,
            overnight_per_diem: 0.0,
            percent_day_roadwork: 0.0,
            percent_overnight_roadwork: 0.0,
            paid_holidays: 0.0,
            anticipated_missed_days: 0.0,
            average_yearly_bonus: 0.0,
            employer_401k_match: 0.0,
            employer_401k_match_limit: 0.0,
        }
    }
}

impl Default for Job {
    fn default() -> Self {
        Job::new()
    }
}

pub enum Pay {
    Salary(f64),
    Hourly(f64),
}

impl std::fmt::Display for Pay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pay::Hourly(rate) => write!(f, "${:.2}/hour", rate),
            Pay::Salary(rate) => write!(f, "${:.2}/year", rate),
        }
    }
}
