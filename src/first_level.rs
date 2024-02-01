use mockall::*;

#[automock]
pub trait FirstLevelTrait {
    fn first_level_method(&self, first_level_flag: String) -> Result<(), anyhow::Error>;
}

pub struct FirstLevel {}

impl FirstLevelTrait for FirstLevel {
    fn first_level_method(&self, _first_level_flag: String) -> Result<(), anyhow::Error> {
        println!("Here is the first flag value: {}", _first_level_flag);
        Ok(())
    }
}
