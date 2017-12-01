use ::serde_json::ser::to_string as to_json;
use ::serde_json::de::from_str as from_json;
use ::serde_json::Result as SerdeResult;

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "assigned")]               Assigned,
    #[serde(rename = "unassigned")]             Unassigned,
    #[serde(rename = "review_requested")]       ReviewRequested,
    #[serde(rename = "review_request_removed")] ReviewRequestRemoved,
    #[serde(rename = "labeled")]                Labeled,
    #[serde(rename = "unlabeled")]              Unlabeled,
    #[serde(rename = "opened")]                 Opened,
    #[serde(rename = "edited")]                 Edited,
    #[serde(rename = "closed")]                 Closed,
    #[serde(rename = "reopened")]               Reopened,
    #[serde(rename = "synchronize")]            Synchronized,
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
