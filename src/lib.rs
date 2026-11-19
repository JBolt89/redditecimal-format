pub mod from_redditecimal;
pub mod implementation;

#[cfg(test)]
mod tests {
    use super::*;
    use implementation::*;

    #[test]
    fn does_i8_work() {
        let i = 0x1A_i8;
        assert_eq!(i.as_redditecimal(), "1⬆️");
    }

    #[test]
    fn does_i16_work() {
        let i = 0x1ABC_i16;
        assert_eq!(i.as_redditecimal(), "1⬆️⬇️❓");
    }

    #[test]
    fn does_i32_work() {
        let i = 0x12ABCDEF_i32;
        assert_eq!(i.as_redditecimal(), "12⬆️⬇️❓🔼🔽❔");
    }

    #[test]
    fn does_i64_work() {
        let i = 0x1234567890ABCDEF_i64;
        assert_eq!(i.as_redditecimal(), "1234567890⬆️⬇️❓🔼🔽❔");
    }

    #[test]
    fn does_i128_work() {
        let i = 0x1234567890ABCDEF_i128;
        assert_eq!(i.as_redditecimal(), "1234567890⬆️⬇️❓🔼🔽❔");
    }

    #[test]
    fn does_isize_work() {
        let i = 0x1234567890ABCDEF_isize;
        assert_eq!(i.as_redditecimal(), "1234567890⬆️⬇️❓🔼🔽❔");
    }

    #[test]
    fn does_u8_work() {
        let i = 0x1A_u8;
        assert_eq!(i.as_redditecimal(), "1⬆️");
    }

    #[test]
    fn does_u16_work() {
        let i = 0x1ABC_u16;
        assert_eq!(i.as_redditecimal(), "1⬆️⬇️❓");
    }

    #[test]
    fn does_u32_work() {
        let i = 0x12ABCDEF_u32;
        assert_eq!(i.as_redditecimal(), "12⬆️⬇️❓🔼🔽❔");
    }

    #[test]
    fn does_u64_work() {
        let i = 0x1234567890ABCDEF_u64;
        assert_eq!(i.as_redditecimal(), "1234567890⬆️⬇️❓🔼🔽❔");
    }

    #[test]
    fn does_u128_work() {
        let i = 0x1234567890ABCDEF_u128;
        assert_eq!(i.as_redditecimal(), "1234567890⬆️⬇️❓🔼🔽❔");
    }

    #[test]
    fn does_usize_work() {
        let i = 0x1234567890ABCDEF_usize;
        assert_eq!(i.as_redditecimal(), "1234567890⬆️⬇️❓🔼🔽❔");
    }

    #[test]
    fn does_from_work() {
        let i = from_redditecimal::from_redditecimal("1234567890⬆️⬇️❓🔼🔽❔").expect("oops");

        assert_eq!(i, 0x1234567890ABCDEF_u128);
    }
}
