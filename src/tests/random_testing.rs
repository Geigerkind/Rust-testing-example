use proptest::prelude::*;
use crate::tools::parse_date;

proptest! {
    #[test]
    fn doesnt_crash(s in "\\PC*") {
        parse_date(&s);
    }

    #[test]
    fn parses_all_valid_dates(s in "[0-9]{4}-[0-9]{2}-[0-9]{2}") {
        parse_date(&s).unwrap();
    }

    #[test]
    fn parses_date_back_to_original(y in 0u32..10000,
                                    m in 1u32..13, d in 1u32..32) {
        let (y2, m2, d2) = parse_date(
            &format!("{:04}-{:02}-{:02}", y, m, d)).unwrap();
        // prop_assert_eq! is basically the same as assert_eq!, but doesn't
        // cause a bunch of panic messages to be printed on intermediate
        // test failures. Which one to use is largely a matter of taste.
        prop_assert_eq!((y, m, d), (y2, m2, d2));
    }
}