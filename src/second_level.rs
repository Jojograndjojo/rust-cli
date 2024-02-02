use mockall::*;

#[automock]
pub trait SecondLevelTrait {
    fn second_level_method(&self, second_level_flag: Option<String>) -> Result<(), anyhow::Error>;
}

pub struct SecondLevel {}

impl SecondLevelTrait for SecondLevel {
    fn second_level_method(&self, second_level_flag: Option<String>) -> Result<(), anyhow::Error> {
        match second_level_flag {
            Some(second_level_flag) => {
                println!("Here is the second flag value: {}", second_level_flag);
                Ok(())
            }
            None => anyhow::bail!("second level flag is empty"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second_level_method() {
        let second_level = SecondLevel {};
        let second_level_flag = Some("ola".to_string());
        assert!(second_level.second_level_method(second_level_flag).is_ok());
    }

    #[test]
    fn test_second_level_method_with_empty_flag() {
        let second_level = SecondLevel {};
        let second_level_flag = None;
        assert!(second_level.second_level_method(second_level_flag).is_err());
    }
}