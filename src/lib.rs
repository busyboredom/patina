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
    pub income_params: income::IncomeParameters,
    pub leisure_params: leisure::LeisureParameters,
    pub misc_params: misc::MiscParameters,
    pub saving_params: saving::SavingParameters,
    pub tax_params: taxes::TaxParameters,
    pub transport_params: transport::TransportParameters,
}

impl Budget {
    pub fn new() -> Budget {
        Budget {
            food_params: food::FoodParameters::new(),
            healthcare_params: healthcare::HealthcareParameters::new(),
            housing_params: housing::HousingParameters::new(),
            income_params: income::IncomeParameters::new(),
            leisure_params: leisure::LeisureParameters::new(),
            misc_params: misc::MiscParameters::new(),
            saving_params: saving::SavingParameters::new(),
            tax_params: taxes::TaxParameters::new(),
            transport_params: transport::TransportParameters::new(),
        }
    }
}

impl Default for Budget {
    fn default() -> Self {
        Budget::new()
    }
}
