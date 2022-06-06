use super::*;

fn example_metrics() -> RawMetrics {
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
            },
            Metric {
                name: "test.test".to_string(),
                value: Some(123)
            }
        ]
    }
}

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
        },
        {
          \"name\": \"test.test\",
          \"value\": 123
        }
      ]
    }";

    let parsed: RawMetrics = serde_json::from_str(raw).unwrap();

    assert_eq!(parsed, example_metrics());
}

#[test]
fn test_get_metric() {
    let test_metric = example_metrics().get_metric("test");
    let test_test_metric = example_metrics().get_metric("test.test");

    assert_eq!(test_metric, None);
    assert_eq!(test_test_metric, Some(123));
}
