use std::fmt;

use serde::{
    Deserializer,
    de::{Error, SeqAccess, Visitor},
};

/// Reads a job's `needs` entry, which a workflow may write either as a single
/// job id or as a sequence of job ids.
///
/// Both spellings deserialize into the same list, so a workflow author is free
/// to write `needs: build` or `needs: [build, lint]`.
pub struct JobNeedsVisitor;

impl<'de> Visitor<'de> for JobNeedsVisitor {
    type Value = Vec<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a job id or a sequence of job ids")
    }

    fn visit_str<E>(self, job_id: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(vec![job_id.to_string()])
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut job_ids = Vec::new();
        while let Some(job_id) = sequence.next_element::<String>()? {
            job_ids.push(job_id);
        }
        Ok(job_ids)
    }
}

/// Deserializes either spelling of `needs:` into the list of job ids.
///
/// # Errors
///
/// Returns the deserializer's error when the entry is neither a string nor a
/// sequence of strings.
pub fn job_needs_from_yaml<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(JobNeedsVisitor)
}
