use kiss_srt::Timestamp;

#[test]
fn max_timestamp_is_max() {
    assert_eq!(Timestamp::MAX.to_string(), "99:59:59,999");
    // Saturates to max
    assert_eq!(Timestamp::MAX + Timestamp::from_millis(1), Timestamp::MAX);
    assert_eq!(
        Timestamp::from_millis(Timestamp::MAX.total_millis() + 1),
        Timestamp::MAX
    );
}

#[test]
fn min_timestamp_is_min() {
    let min = Timestamp::default();
    assert_eq!(min.to_string(), "00:00:00,000");
    // Saturates to min
    assert_eq!(min - Timestamp::from_millis(1), min);
}

#[test]
fn float_shenanigans() {
    // Floats out of bounds will saturate
    assert_eq!(Timestamp::MAX * 1.1, Timestamp::MAX);
    assert_eq!(Timestamp::from_millis(1) * -1.0, Timestamp::default());
    assert_eq!(Timestamp::from_millis(1) * f32::INFINITY, Timestamp::MAX);
    assert_eq!(
        Timestamp::from_millis(1) * f32::NEG_INFINITY,
        Timestamp::default()
    );

    // Floats in bounds are fine
    assert_eq!(
        Timestamp::from_millis(1) * 100.0,
        Timestamp::from_millis(100)
    );
    assert_eq!(Timestamp::from_millis(10) * 0.1, Timestamp::from_millis(1));
}

#[test]
fn checked_from_millis() {
    assert!(Timestamp::checked_from_millis(Timestamp::MAX.total_millis() + 1).is_none());
}
