use crate::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// `CREATE name;`
    Create { name: String },
    /// `INSERT name {v1, v2, ...};`
    Insert { collection: String, set: Vec<Value> },
    /// `PRINT_TREE name;`
    PrintTree { collection: String },
    /// `CONTAINS name {v1, v2, ...};`
    Contains { collection: String, set: Vec<Value> },
    /// `SEARCH name [WHERE query];`
    Search { collection: String, filter: Option<SearchQuery> },
}

/// The `query` part of `SEARCH ... WHERE query`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchQuery {
    Intersects(Vec<Value>),
    Contains(Vec<Value>),
    ContainedBy(Vec<Value>),
}
