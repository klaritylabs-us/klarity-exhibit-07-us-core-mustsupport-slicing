use serde_json::Value;

use crate::outcome::{ValidationIssue, ValidationReport};
use crate::profile::{
    MRN_TYPE_CODE, US_CORE_OBSERVATION_PROFILE, US_CORE_PATIENT_PROFILE, US_SSN_SYSTEM,
};

pub fn validate_resource_json(json: &str) -> Result<ValidationReport, String> {
    let value: Value = serde_json::from_str(json).map_err(|e| format!("JSON parse: {e}"))?;
    Ok(validate_resource_value(&value))
}

pub fn validate_resource_value(value: &Value) -> ValidationReport {
    match value.get("resourceType").and_then(Value::as_str) {
        Some("Patient") => validate_patient_slices(value),
        Some("Observation") => validate_observation_mustsupport(value),
        other => {
            let mut report = ValidationReport::ok();
            report.push(ValidationIssue::error(
                "resourceType",
                format!(
                    "this validator covers Patient (slicing) or Observation (lab US Core); got {:?}",
                    other
                ),
                "resource-type",
            ));
            report
        }
    }
}

fn validate_patient_slices(patient: &Value) -> ValidationReport {
    let mut report = ValidationReport::ok();

    if !meta_has_profile(patient, US_CORE_PATIENT_PROFILE) {
        report.push(ValidationIssue::error(
            "Patient.meta.profile",
            format!("expected {}", US_CORE_PATIENT_PROFILE),
            "patient-profile",
        ));
    }

    let ids = patient.get("identifier").and_then(Value::as_array);
    if ids.is_none() || ids.is_some_and(|a| a.len() < 2) {
        report.push(ValidationIssue::warning(
            "Patient.identifier",
            "exercise both MRN and SSN slices when testing Must Support identifier paths",
            "identifier-slice-count",
        ));
    }

    if let Some(ids) = ids {
        let mut has_mrn_slice = false;
        let mut has_ssn_slice = false;

        for (idx, id) in ids.iter().enumerate() {
            let path = format!("Patient.identifier[{}]", idx);
            let type_code = id
                .pointer("/type/coding/0/code")
                .and_then(Value::as_str);
            let system = id.get("system").and_then(Value::as_str);

            if type_code == Some(MRN_TYPE_CODE) {
                has_mrn_slice = true;
                if system.is_none_or(|s| s.is_empty()) {
                    report.push(ValidationIssue::error(
                        format!("{path}.system"),
                        "MRN slice needs a assigning authority system URI",
                        "mrn-system",
                    ));
                }
            }

            let is_ssn = type_code == Some("SS")
                || id
                    .pointer("/type/text")
                    .and_then(Value::as_str)
                    .is_some_and(|t| t.to_ascii_lowercase().contains("ssn"));
            if is_ssn {
                has_ssn_slice = true;
                if system != Some(US_SSN_SYSTEM) {
                    report.push(ValidationIssue::error(
                        format!("{path}.system"),
                        format!("SSN slice must use {}", US_SSN_SYSTEM),
                        "ssn-slice-system",
                    ));
                }
            }
        }

        if !has_mrn_slice {
            report.push(ValidationIssue::warning(
                "Patient.identifier",
                "no MRN-type identifier slice detected",
                "mrn-slice-missing",
            ));
        }
        if !has_ssn_slice {
            report.push(ValidationIssue::warning(
                "Patient.identifier",
                "no SSN-type identifier slice detected",
                "ssn-slice-missing",
            ));
        }
    }

    if patient.get("name").and_then(Value::as_array).is_none() {
        report.push(ValidationIssue::error(
            "Patient.name",
            "name is Must Support",
            "name-mustsupport",
        ));
    }

    report
}

fn validate_observation_mustsupport(obs: &Value) -> ValidationReport {
    let mut report = ValidationReport::ok();

    if !meta_has_profile(obs, US_CORE_OBSERVATION_PROFILE) {
        report.push(ValidationIssue::warning(
            "Observation.meta.profile",
            format!(
                "lab example should declare {} when testing lab Must Support",
                US_CORE_OBSERVATION_PROFILE
            ),
            "observation-lab-profile",
        ));
    }

    for field in ["status", "code", "subject", "effectiveDateTime", "valueQuantity"] {
        if obs.get(field).is_none() {
            report.push(ValidationIssue::error(
                format!("Observation.{}", field),
                format!("{} is Must Support on US Core Observation (lab)", field),
                "observation-mustsupport",
            ));
        }
    }

    report
}

fn meta_has_profile(resource: &Value, profile: &str) -> bool {
    resource
        .pointer("/meta/profile")
        .and_then(Value::as_array)
        .is_some_and(|arr| arr.iter().filter_map(Value::as_str).any(|p| p == profile))
}
