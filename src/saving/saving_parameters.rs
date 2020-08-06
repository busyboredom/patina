pub struct SavingParameters {

}

impl SavingParameters {
    pub fn new() -> SavingParameters {
        SavingParameters {}
    }
}

impl Default for SavingParameters { 
    fn default() -> Self{
        SavingParameters::new()
    }
}