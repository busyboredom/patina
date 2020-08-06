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
    pub food_params: food::FoodParameters,
    pub healthcare_params: healthcare::HealthcareParameters,
    pub housing_params: housing::HousingParameters,
    pub misc_params: misc::MiscParameters,
    pub tax_params: taxes::TaxParameters,
    pub transport_params: transport::TransportParameters,
    pub income_params: income::IncomeParameters,
}

impl Budget {
    pub fn new () -> Budget {
        let food_params = food::FoodParameters::new();
        let healthcare_params = healthcare::HealthcareParameters::new();
        let housing_params = housing::HousingParameters::new();
        let misc_params = misc::MiscParameters::new();
        let tax_params = taxes::TaxParameters::new();
        let transport_params = transport::TransportParameters::new();
        let income_params = income::IncomeParameters::new();
        
        Budget {
            food_params,
            healthcare_params,
            housing_params,
            misc_params,
            tax_params,
            transport_params,
            income_params,
        }
    }
}

impl Default for Budget {
    fn default() -> Self {
        Budget::new()
    }
}
