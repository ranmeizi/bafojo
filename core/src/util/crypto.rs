use crypto::{digest::Digest, md5::Md5};
use rand::Rng;

pub fn gen_salt() -> String {
    let mut rng = rand::thread_rng();

    let mut vec = Vec::<u8>::new();
    for i in 0..8 {
        vec.push(rng.gen_range(33..126))
    }

    String::from_utf8(vec).unwrap()
}

pub fn into_md5_psw(psw: &str, salt: &str) -> String {
    let input = format!("{salt}{psw}");
    let mut md5 = Md5::new();
    md5.input_str(&input);
    md5.result_str()
}

#[cfg(test)]
mod tests {

    use std::println;

    use super::*;

    #[test]
    fn test_md5() {
        let res = into_md5_psw("dingzhen", "dingzhen");

        println!("{res}");
        assert_eq!(res, "8b541f7176fb725428a0c37ffb42a027");
    }

    #[test]
    fn test_salt() {
        let salt = gen_salt();
        println!("{salt}");
        assert_eq!(salt.len(), 8);
    }
}
