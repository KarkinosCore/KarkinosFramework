#![feature(arbitrary_enum_discriminant)]
extern crate core;

mod constants;
mod cryptography;
pub use self::constants::*;
pub use self::cryptography::*;

#[cfg(test)]
mod tests {
    use crate::authentication::auth_consts::{ResponseCodes};
    use crate::cryptography::session_key_generation::{SessionKeyGenerator};

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
    fn test_session_key_generator_new() {
        let input_buff = [32; 40];
        let sk = SessionKeyGenerator::new(&input_buff, 40);
        let o1 = vec![30, 138, 16, 93, 188, 171, 47, 93, 107, 48, 166, 112, 192, 255, 145, 148, 47, 77, 182, 36, 1, 230, 105, 51, 16, 55, 16, 30, 148, 25, 130, 80];
        let o2 = vec![30, 138, 16, 93, 188, 171, 47, 93, 107, 48, 166, 112, 192, 255, 145, 148, 47, 77, 182, 36, 1, 230, 105, 51, 16, 55, 16, 30, 148, 25, 130, 80];
        let o0 = vec![158, 58, 193, 193, 11, 28, 197, 161, 226, 94, 114, 69, 196, 102, 158, 9, 36, 202, 135, 172, 126, 189, 159, 139, 163, 243, 59, 222, 125, 243, 117, 184];

        assert_eq!(o1, sk.o1);
        assert_eq!(o2, sk.o2);
        assert_eq!(o0, sk.o0);
    }

    #[test]
    fn test_generate_key() {
        let expected_pregen_result:[u8; 40] = [32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32];
        let expected_postgen_result: [u8; 40] = [246, 36, 141, 177, 174, 167, 191, 190, 93, 41, 79, 182, 250, 183, 156, 206, 108, 115, 129, 74, 163, 10, 206, 135, 90, 197, 23, 237, 1, 119, 23, 94, 48, 89, 245, 132, 119, 132, 48, 97];
        let input_buff = [32; 32];
        let mut key_buff = [32; 40];
        let mut sk = SessionKeyGenerator::new(&input_buff, 32);

        assert_eq!(key_buff, expected_pregen_result);
        sk.generate_key(&mut key_buff, 40);
        assert_eq!(key_buff, expected_postgen_result);
    }
}
