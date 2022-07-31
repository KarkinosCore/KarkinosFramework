#![feature(arbitrary_enum_discriminant)]
extern crate core;

mod constants;
mod cryptography;
pub use self::constants::*;
pub use self::cryptography::*;

#[cfg(test)]
mod tests {
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
    fn test_session_key_generator_new() {
        let input_buff = [32; 40];
        let sk = session_key_generation::SessionKeyGenerator::new(&input_buff, 40);
        let o1 = vec![30, 138, 16, 93, 188, 171, 47, 93, 107, 48, 166, 112, 192, 255, 145, 148, 47, 77, 182, 36, 1, 230, 105, 51, 16, 55, 16, 30, 148, 25, 130, 80];
        let o2 = vec![30, 138, 16, 93, 188, 171, 47, 93, 107, 48, 166, 112, 192, 255, 145, 148, 47, 77, 182, 36, 1, 230, 105, 51, 16, 55, 16, 30, 148, 25, 130, 80];
        let o0 = vec![158, 58, 193, 193, 11, 28, 197, 161, 226, 94, 114, 69, 196, 102, 158, 9, 36, 202, 135, 172, 126, 189, 159, 139, 163, 243, 59, 222, 125, 243, 117, 184];

        assert_eq!(o1, sk.o1);
        assert_eq!(o2, sk.o2);
        assert_eq!(o0, sk.o0);
    }
}
