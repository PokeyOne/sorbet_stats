use super::*;

const FULL_METRICS_FILE_EXAMPLE: &str = r#"
{
    "repo": "sorbet/sorbet",
    "sha": "a8f9f8f8f8f8f8f8f8f8f8f8f8f8f8f8f8f8f8f8f8",
    "status": "success",
    "branch": "master",
    "timestamp": "1036546800",
    "uuid": "a8f9f8f8f8f8f8f8f8f8f8f8f8f8f8f8f8f8f8f8f",
    "metrics": [
        {
            "name": "sorbet.types.input.files.sigil.ignore",
            "value": 1
        },
        {
            "name": "sorbet.types.input.files.sigil.false",
            "value": 2
        },
        {
            "name": "sorbet.types.input.files.sigil.true",
            "value": 3
        },
        {
            "name": "sorbet.types.input.files.sigil.strict",
            "value": 4
        },
        {
            "name": "sorbet.types.input.files.sigil.strong",
            "value": 5
        }
    ]
}
"#;

#[test]
fn test_to_sigil_csv() {
    let metrics: RawMetrics = serde_json::from_str(FULL_METRICS_FILE_EXAMPLE).unwrap();
    let project = Project::new(metrics).unwrap();

    let sigil_csv = project.to_sigil_csv();

    let expected = "level,file count\nignore,1\nfalse,2\ntrue,3\nstrict,4\nstrong,5\n";

    assert_eq!(sigil_csv, expected);
}