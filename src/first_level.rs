use mockall::*;

#[automock]
pub trait FirstLevelTrait {
    fn first_level_method(&self, first_level_flag: Option<String>) -> Result<(), anyhow::Error>;
}

pub struct FirstLevel {}

impl FirstLevelTrait for FirstLevel {
    fn first_level_method(&self, first_level_flag: Option<String>) -> Result<(), anyhow::Error> {
        match first_level_flag {
            Some(first_level_flag) => {
                println!("Here is the first flag value: {}", first_level_flag);
                Ok(())
            },
            None => anyhow::bail!("first level flag is empty"),
        }
    }
}
