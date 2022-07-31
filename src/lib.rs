#![feature(arbitrary_enum_discriminant)]
mod constants;
mod cryptography;
pub use self::constants::*;
pub use self::cryptography::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authentication::auth_consts::{ResponseCodes};
    use crate::cryptography::session_key_generation;


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

    #[test]
    fn test_transform_block() {
        let input_buff = [1; 32];
        let out_buff = session_key_generation::transform_block(&input_buff, 0, 9, 9);
        println!("{:?}", out_buff);
    }
}
