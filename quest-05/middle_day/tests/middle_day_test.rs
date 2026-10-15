use chrono::Weekday;
use middle_day::*;

#[test]
fn test_middle_day() {
    // 公式サンプル
    assert_eq!(middle_day(1022), Some(Weekday::Tue));

    // 平年
    assert_eq!(middle_day(2023), Some(Weekday::Sun));

    // 閏年
    assert_eq!(middle_day(2024), None);
    assert_eq!(middle_day(2000), None);

    // 100で割り切れるが400で割り切れない年は平年
    assert!(middle_day(1900).is_some());
    assert!(middle_day(2100).is_some());

    // chronoが扱えない大きな年
    assert_eq!(middle_day(u32::MAX), None);
}
