#![feature(arbitrary_enum_discriminant)]
extern crate core;

pub use self::constants::*;
pub use self::cryptography::*;

mod constants;
pub mod cryptography;

#[cfg(test)]
mod tests {
    use crate::authentication::auth_consts::ResponseCodes;
    use crate::cryptography::session_key_generation::{generate_key};

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
    fn test_generate_key() {
        let expected_postgen_result: [u8; 40] = [246, 36, 141, 177, 174, 167, 191, 190, 93, 41, 79, 182, 250, 183, 156, 206, 108, 115, 129, 74, 163, 10, 206, 135, 90, 197, 23, 237, 1, 119, 23, 94, 48, 89, 245, 132, 119, 132, 48, 97];
        let input_buff = [32; 32];
        let mut output_buff = [32; 40];

        generate_key(&input_buff, 32,  &mut output_buff, 40);
        assert_eq!(output_buff, expected_postgen_result);
    }
}
