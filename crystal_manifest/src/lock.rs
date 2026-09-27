use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct CrystalLock {
    pub name: String,
    pub version: String,
    pub source: Url,
    pub sha256: String,
}
