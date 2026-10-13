use std::fs;
use std::path::PathBuf;

use us_core_mustsupport::validate_resource_json;

fn fixture(name: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("examples");
    path.push(name);
    fs::read_to_string(path).expect("fixture")
}

#[test]
fn valid_patient_slices() {
    let report = validate_resource_json(&fixture("valid-patient-slices.json")).unwrap();
    assert!(report.is_valid(), "{:?}", report.issues);
}

#[test]
fn invalid_ssn_slice_system() {
    let report = validate_resource_json(&fixture("invalid-ssn-slice-system.json")).unwrap();
    assert!(!report.is_valid());
}
