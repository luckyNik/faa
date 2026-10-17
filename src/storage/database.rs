use std::collections::HashMap;

use super::RdTree;

#[derive(Default)]
pub struct Database {
    pub collections: HashMap<String, RdTree>,
}
