use super::logic::new_asset;

#[test]
fn creates_asset() {
    let asset = new_asset(1, "test");
    assert_eq!(asset.id, 1);
    assert_eq!(asset.name, "test");
}
