use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bounds {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub text: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenId {
    #[serde(rename = "pageIndex")]
    pub page_index: i32,
    #[serde(rename = "tokenIndex")]
    pub token_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub id: String,
    pub page: i32,
    pub label: Label,
    pub bounds: Bounds,
    pub tokens: Option<Vec<TokenId>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationGroup {
    #[serde(rename = "sourceIds")]
    pub source_ids: Vec<String>,
    #[serde(rename = "targetIds")]
    pub target_ids: Vec<String>,
    pub label: Label,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfAnnotations {
    pub annotations: Vec<Annotation>,
    pub relations: Vec<RelationGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperStatus {
    sha: String,
    name: String,
    annotations: i32,
    relations: i32,
    finished: bool,
    junk: bool,
    comments: String,
    #[serde(rename = "completedAt")]
    completed_at: Option<String>,
}

impl PaperStatus {
    pub fn empty(sha: &str, name: &str) -> PaperStatus {
        PaperStatus {
            sha: sha.to_string(),
            name: name.to_string(),
            annotations: 0,
            relations: 0,
            finished: false,
            junk: false,
            comments: "".into(),
            completed_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Allocation {
    pub papers: Vec<PaperStatus>,
    #[serde(rename = "hasAllocatedPapers")]
    pub has_allocated_papers: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub index: i32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub width: f32,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageTokens {
    pub page: Page,
    pub tokens: Vec<Token>,
}
