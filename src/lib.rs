#![feature(arbitrary_enum_discriminant)]
mod constants;
pub use self::constants::*;

#[cfg(test)]
mod tests {
    use crate::authentication::auth_consts::{ResponseCodes};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_has_working_flags() {
        let char_name_profane = ResponseCodes::CharNameProfane;
        assert_eq!(char_name_profane, 87);
        assert!(char_name_profane.contains(ResponseCodes::CharNameProfane));
    }
}
