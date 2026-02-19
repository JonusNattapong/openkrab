use krabkrab::channels::location::*;

#[test]
fn test_format_location_text_live() {
    let loc = NormalizedLocation {
        latitude: 12.345678,
        longitude: 98.765432,
        accuracy: Some(12.0),
        name: None,
        address: None,
        is_live: Some(true),
        source: None,
        caption: Some("moving".to_string()),
    };
    let s = format_location_text(&loc);
    assert!(s.contains("Live location"));
    assert!(s.contains("moving"));
}

#[test]
fn test_to_location_context() {
    let loc = NormalizedLocation {
        latitude: 1.0,
        longitude: 2.0,
        accuracy: None,
        name: Some("Park".to_string()),
        address: None,
        is_live: None,
        source: None,
        caption: None,
    };
    let ctx = to_location_context(&loc);
    assert_eq!(ctx.LocationLat, 1.0);
    assert_eq!(ctx.LocationLon, 2.0);
}
