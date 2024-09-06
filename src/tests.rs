#[cfg(test)]
mod tests {
    use crate::pwd_count;

    #[test]
    fn validate_default_passwords_returned() {
        let default = pwd_count(6);
        assert_eq!(default.len(), 6)
    }

    #[test]
    fn validate_certain_number_of_passwords_returned() {
        let default = pwd_count(15);
        assert_eq!(default.len(), 15)
    }

    #[test]
    fn validate_max_passwords_returned() {
        let default = pwd_count(1967);
        assert_eq!(default.len(), 31)
    }

    #[test]
    fn validate_no_passwords_returned() {
        let default = pwd_count(0);
        assert_eq!(default.len(), 31)
    }
}