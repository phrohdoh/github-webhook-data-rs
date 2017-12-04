use ::std::str::FromStr;
use ::serde::de::{value, Deserialize, IntoDeserializer};
use ::serde_json::ser::to_string as to_json;
use ::serde_json::de::from_str as from_json;
use ::serde_json::Result as SerdeResult;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Assigned,
    Unassigned,
    ReviewRequested,
    ReviewRequestRemoved,
    Labeled,
    Unlabeled,
    Opened,
    Edited,
    Closed,
    Reopened,

    #[serde(rename = "synchronize")]
    Synchronized,
}

impl FromStr for Action {
    type Err = value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub action: Action,
    pub number: usize,
    pub pull_request: PullRequest,
    // TODO: Add more fields.
}

impl Event {
    pub fn to_json(&self) -> SerdeResult<String>  {
        to_json(self)
    }

    pub fn from_json(json: &str) -> SerdeResult<Self> {
        from_json(json)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequest {
    pub head: Head,
    // TODO: Add more fields.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Head {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub sha: String,
    // TODO: Add more fields.
}
