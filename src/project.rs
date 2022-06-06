use std::fmt::{Display, Formatter};
use crate::RawMetrics;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Project {
    pub name: String,
    pub repo: String,
    pub sha: String,
    pub branch: String,
    pub timestamp: String,
    pub uuid: String,
    pub types: TypesMetrics
}

#[derive(Debug, PartialEq)]
pub struct TypesMetrics {
    pub input: InputMetrics
}

#[derive(Debug, PartialEq)]
pub struct InputMetrics {
    pub files: FilesMetrics
}

#[derive(Debug, PartialEq)]
pub struct FilesMetrics {
    pub sigil: SigilMetrics
}

/// The levels of Sorbet typing.
#[derive(Debug, PartialEq)]
pub struct SigilMetrics {
    pub ignore_count: u32,
    pub false_count: u32,
    pub true_count: u32,
    pub strict_count: u32,
    pub strong_count: u32
}

#[derive(Debug, PartialEq)]
pub enum ProjectError {
    MissingMetric(String)
}

impl Project {
    pub fn new(raw_metrics: RawMetrics) -> Result<Project, ProjectError> {
        let name = raw_metrics.metrics.first()
            .and_then(|v| Some(v.name_components()))
            .and_then(|v| v.into_iter().next())
            .unwrap();

        let file_sigil_metrics = SigilMetrics {
            ignore_count: raw_metrics.get_metric(&format!("{}.types.input.files.sigil.ignore", name)).unwrap_or_default(),
            false_count: raw_metrics.get_metric(&format!("{}.types.input.files.sigil.false", name)).unwrap_or_default(),
            true_count: raw_metrics.get_metric(&format!("{}.types.input.files.sigil.true", name)).unwrap_or_default(),
            strict_count: raw_metrics.get_metric(&format!("{}.types.input.files.sigil.strict", name)).unwrap_or_default(),
            strong_count: raw_metrics.get_metric(&format!("{}.types.input.files.sigil.strong", name)).unwrap_or_default()
        };

        let result = Project {
            name,
            repo: raw_metrics.repo,
            sha: raw_metrics.sha,
            branch: raw_metrics.branch,
            timestamp: raw_metrics.timestamp,
            uuid: raw_metrics.uuid,
            types: TypesMetrics {
                input: InputMetrics {
                    files: FilesMetrics {
                        sigil: file_sigil_metrics
                    }
                }
            }
        };

        Ok(result)
    }

    /// Generate a CSV string representation of sigil metrics.
    pub fn to_sigil_csv(&self) -> String {
        let mut result = String::new();

        result.push_str("level,file count\n");
        result.push_str("ignore,");
        result.push_str(&self.types.input.files.sigil.ignore_count.to_string());
        result.push_str("\n");
        result.push_str("false,");
        result.push_str(&self.types.input.files.sigil.false_count.to_string());
        result.push_str("\n");
        result.push_str("true,");
        result.push_str(&self.types.input.files.sigil.true_count.to_string());
        result.push_str("\n");
        result.push_str("strict,");
        result.push_str(&self.types.input.files.sigil.strict_count.to_string());
        result.push_str("\n");
        result.push_str("strong,");
        result.push_str(&self.types.input.files.sigil.strong_count.to_string());
        result.push_str("\n");

        result
    }
}

impl Display for ProjectError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingMetric(val) => write!(f, "Missing required metric: {val:?}")
        }
    }
}