pub const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn caesar(input: &str, shift: i32) -> String {
    input.chars().map(|c| {
        if c.is_ascii_lowercase() {
            let base = b'a' as i32;
            // rem_euclid handles both negative shifts (-1) and large wraps (27) perfectly
            let offset = (c as i32 - base + shift).rem_euclid(26);
            (base + offset) as u8 as char
        } else if c.is_ascii_uppercase() {
            let base = b'A' as i32;
            let offset = (c as i32 - base + shift).rem_euclid(26);
            (base + offset) as u8 as char
        } else {
            c // Leaves spaces, punctuation, and numbers untouched
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_shift_three() {
        assert_eq!(caesar("Hello, World!", 3), "Khoor, Zruog!");
    }

    #[test]
    fn shift_minus_one() {
        assert_eq!(caesar("abc", -1), "zab");
    }

    #[test]
    fn shift_twenty_seven_wraps_to_one() {
        assert_eq!(caesar("xyz", 27), "yza");
    }

    #[test]
    fn empty_input() {
        assert_eq!(caesar("", 5), "");
    }

    #[test]
    fn shift_zero_is_identity() {
        assert_eq!(caesar("Rust!", 0), "Rust!");
    }

    #[test]
    fn shift_twenty_six_is_identity() {
        assert_eq!(caesar("abc", 26), "abc");
    }

    #[test]
    fn non_letters_preserved() {
        assert_eq!(caesar("1 2 3 !", 5), "1 2 3 !");
    }

    #[test]
    fn large_negative_shift_wraps() {
        assert_eq!(caesar("abc", -27), "zab");
    }
}
