use super::{Amount, TieredMulti};

#[test]
fn summary_works() {
    let notes = TieredMulti::from_iter(vec![
        (Amount::from_sats(1), ()),
        (Amount::from_sats(2), ()),
        (Amount::from_sats(3), ()),
        (Amount::from_sats(3), ()),
        (Amount::from_sats(2), ()),
        (Amount::from_sats(2), ()),
    ]);
    let summary = notes.summary();
    assert_eq!(
        summary.iter().collect::<Vec<_>>(),
        vec![
            (Amount::from_sats(1), 1),
            (Amount::from_sats(2), 3),
            (Amount::from_sats(3), 2),
        ]
    );
    assert_eq!(summary.total_amount(), notes.total_amount());
    assert_eq!(summary.count_items(), notes.count_items());
    assert_eq!(summary.count_tiers(), notes.count_tiers());
}

#[test]
fn checked_total_amount_reports_an_overflowing_tier() {
    // Two notes in one tier is enough: the tier is read from the encoding, not
    // from a federation's configuration, so it can be any `u64`.
    let notes = TieredMulti::from_iter(vec![
        (Amount::from_msats(u64::MAX), ()),
        (Amount::from_msats(u64::MAX), ()),
    ]);
    assert_eq!(notes.checked_total_amount(), None);

    // And across tiers, where each tier on its own fits.
    let notes = TieredMulti::from_iter(vec![
        (Amount::from_msats(u64::MAX), ()),
        (Amount::from_msats(1), ()),
    ]);
    assert_eq!(notes.checked_total_amount(), None);
}

#[test]
fn checked_total_amount_agrees_with_total_amount() {
    let notes = TieredMulti::from_iter(vec![
        (Amount::from_sats(1), ()),
        (Amount::from_sats(2), ()),
        (Amount::from_sats(2), ()),
    ]);
    assert_eq!(notes.checked_total_amount(), Some(Amount::from_sats(5)));
    assert_eq!(notes.checked_total_amount(), Some(notes.total_amount()));
    assert_eq!(
        notes.summary().checked_total_amount(),
        Some(notes.total_amount())
    );

    let empty = TieredMulti::<()>::default();
    assert_eq!(empty.checked_total_amount(), Some(Amount::ZERO));
}

#[test]
#[should_panic(expected = "notes total amount overflow")]
fn total_amount_panics_instead_of_wrapping() {
    let notes = TieredMulti::from_iter(vec![
        (Amount::from_msats(u64::MAX), ()),
        (Amount::from_msats(u64::MAX), ()),
    ]);
    let _ = notes.total_amount();
}

#[test]
#[should_panic(expected = "note counts total amount overflow")]
fn counted_total_amount_panics_instead_of_wrapping() {
    let notes = TieredMulti::from_iter(vec![
        (Amount::from_msats(u64::MAX), ()),
        (Amount::from_msats(u64::MAX), ()),
    ]);
    let _ = notes.summary().total_amount();
}
