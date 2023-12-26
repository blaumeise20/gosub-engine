use crate::css3::node::{Node, NodeType};
use crate::css3::{Css3, Error};

impl Css3<'_> {
    fn parse_at_rule_layer_list(&mut self) -> Result<Node, Error> {
        let _children: Vec<Node> = Vec::new();

        todo!();
    }

    fn parse_layer_query(&mut self) -> Result<Node, Error> {
        let _children: Vec<Node> = Vec::new();
        todo!();
    }

    pub fn parse_at_rule_layer_prelude(&mut self) -> Result<Node, Error> {
        log::trace!("parse_at_rule_layer_prelude");

        let loc = self.tokenizer.current_location().clone();

        self.consume_whitespace_comments();

        let mut layers = vec![];

        while !self.tokenizer.eof() {
            let layer = self.parse_layer_query()?;
            layers.push(layer);

            self.consume_whitespace_comments();

            let t = self.consume_any()?;
            if !t.is_comma() {
                self.tokenizer.reconsume();
                break;
            }
        }

        Ok(Node::new(NodeType::LayerList { layers }, loc))
    }
}
