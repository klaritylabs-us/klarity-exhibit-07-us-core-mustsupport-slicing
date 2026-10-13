# US Core Must Support & identifier slicing (FHIR R4) — Rust

Small library and CLI that exercises **US Core profile mechanics** on **Patient** identifier slices (MRN vs SSN systems) and **Observation** lab Must Support fields — the rules mapper generators often get almost right.

Synthetic fixtures only. Not ONC certification, not a substitute for the HL7 FHIR Validator or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 07** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [US Core Must Support](http://hl7.org/fhir/us/core/general-requirements.html#mustsupport)

## What it checks

**Patient (slicing)**

- `meta.profile` includes [US Core Patient](http://hl7.org/fhir/us/core/STU6.1/StructureDefinition-us-core-patient.html)
- MRN slice: `identifier.type` with MRN code and assigning-authority `system` URI
- SSN slice: `identifier.system` is `http://hl7.org/fhir/sid/us-ssn`
- Warnings when MRN or SSN slice paths are missing from test data
- `name` Must Support

**Observation (lab Must Support)**

- Warnings if lab example omits US Core Observation profile URL
- Errors if `status`, `code`, `subject`, `effectiveDateTime`, or `valueQuantity` missing

| Problem | What breaks |
|--------|-------------|
| **SSN on legacy OID only** | Slice system URI wrong for US Core |
| **MRN without system** | Cannot match across assigning authorities |
| **Lab Obs without MS fields** | Valid JSON, fails receiver Must Support expectations |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-us-core-resource -- examples/valid-patient-slices.json
cargo run --bin validate-us-core-resource -- examples/invalid-ssn-slice-system.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or JSON.

## Sample output

One command:

```bash
cargo run --bin validate-us-core-resource -- examples/valid-patient-slices.json
```

**Pass** (`examples/valid-patient-slices.json`) — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-ssn-slice-system.json`) — exit code `1` (issues array; more entries may appear):

```json
[
  {
    "severity": "error",
    "path": "Patient.identifier[0].system",
    "detail": "SSN slice must use http://hl7.org/fhir/sid/us-ssn",
    "rule_id": "ssn-slice-system"
  }
]
```


## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken payloads for tests and demos

## Inferno vs this repo

Inferno exercises your **API** against CMS-style scenarios. This repo is a **focused lab** for slicing and Must Support — profile URLs and slice systems that Inferno scenarios may not stress in your exact mapper output.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
