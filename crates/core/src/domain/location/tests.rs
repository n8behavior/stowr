use super::logic::new_location;

#[test]
fn creates_location() {
    let loc = new_location(1, "warehouse");
    assert_eq!(loc.id, 1);
    assert_eq!(loc.name, "warehouse");
}
