use mockall::*;

#[automock]
pub trait SecondLevelTrait {
    fn second_level_method(
        &self,
        second_level_flag: String,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct SecondLevel {}

impl SecondLevelTrait for SecondLevel {
    fn second_level_method(
        &self,
        _second_level_flag: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
