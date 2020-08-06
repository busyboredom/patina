pub struct FoodParameters {

}

impl FoodParameters {
    pub fn new() -> FoodParameters {
        FoodParameters {}
    }
}

impl Default for FoodParameters {
    fn default() -> Self {
        FoodParameters::new()
    }
}