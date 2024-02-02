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
            }
            None => anyhow::bail!("first level flag is empty"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_level_method() {
        let first_level = FirstLevel {};
        let first_level_flag = Some("hello".to_string());
        assert!(first_level.first_level_method(first_level_flag).is_ok());
    }

    #[test]
    fn test_first_level_method_with_empty_flag() {
        let first_level = FirstLevel {};
        let first_level_flag = None;
        assert!(first_level.first_level_method(first_level_flag).is_err());
    }
}
