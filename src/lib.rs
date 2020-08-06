pub mod food;
pub mod healthcare;
pub mod housing;
pub mod income;
pub mod leisure;
pub mod misc;
pub mod saving;
pub mod taxes;
pub mod transport;

pub struct Budget {
    pub income: f64,
    pub age: i32,
    pub food_params: food::FoodParameters,
    pub healthcare_params: healthcare::HealthcareParameters,
    pub housing_params: housing::HousingParameters,
}

impl Budget {
    pub fn new (income: f64, age: i32) -> Budget {
        let food_params = food::FoodParameters::new();
        let healthcare_params = healthcare::HealthcareParameters::new();
        let housing_params = housing::HousingParameters::new();
        Budget {
            income,
            age,
            food_params,
            healthcare_params,
            housing_params,
        }
    }
}
