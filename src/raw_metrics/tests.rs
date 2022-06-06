use super::*;

#[test]
fn test_raw_metrics_serialization() {
    let raw = "{
      \"repo\": \"a\",
      \"sha\": \"123\",
      \"status\": \"123\",
      \"branch\": \"123\",
      \"timestamp\": \"123\",
      \"uuid\": \"123\",
      \"metrics\": [
        {
          \"name\": \"test\"
        }
      ]
    }";

    let parsed: RawMetrics = serde_json::from_str(raw).unwrap();

    assert_eq!(
        parsed,
        RawMetrics {
            repo: "a".to_string(),
            sha: "123".to_string(),
            status: "123".to_string(),
            branch: "123".to_string(),
            timestamp: "123".to_string(),
            uuid: "123".to_string(),
            metrics: vec![
                Metric {
                    name: "test".to_string(),
                    value: None
                }
            ]
        }
    )
}