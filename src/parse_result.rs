use crate::nodes::Node;

#[derive(Debug, Clone)]
pub struct ParseResult {
    pub error: Option<String>,
    pub node: Option<Node>,
    pub last_registered_advance_count: usize,
    pub advance_count: usize,
    pub to_reverse_count: usize,
}

impl ParseResult {
    pub fn new() -> Self {
        Self {
            error: None,
            node: None,
            last_registered_advance_count: 0,
            advance_count: 0,
            to_reverse_count: 0,
        }
    }

    pub fn register_advancement(&mut self) {
        self.last_registered_advance_count = 1;
        self.advance_count += 1;
    }

    pub fn register(&mut self, res: &ParseResult) -> Option<Node> {
        self.last_registered_advance_count = res.advance_count;
        self.advance_count += res.advance_count;
        if res.error.is_some() {
            self.error = res.error.clone();
        }
        res.node.clone()
    }

    pub fn try_register(&mut self, res: &ParseResult) -> Option<Node> {
        if res.error.is_some() {
            self.to_reverse_count = res.advance_count;
            None
        } else {
            self.register(res)
        }
    }

    pub fn success(mut self, node: Node) -> Self {
        self.node = Some(node);
        self
    }

    pub fn failure(mut self, error: String) -> Self {
        if self.error.is_none() || self.last_registered_advance_count == 0 {
            self.error = Some(error);
        }
        self
    }
}
