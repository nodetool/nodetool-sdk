use nodetool_sdk_core::node::{
	Node, NodeParameter, NodeParameterDescriptor, NodeParameterType, NodeResult,
};
use nodetool_sdk_macros::Node;

#[derive(Node)]
#[nodetool(name = "Constant", inputs(Number, Number), outputs(String))]
pub struct Constant {
	pub value: f64,
}

impl Constant {
	pub fn new(value: f64) -> Self {
		Self { value }
	}
}

impl Node for Constant {
	fn eval(&mut self, _inputs: Vec<Option<NodeParameter>>) -> NodeResult {
		Ok(vec![NodeParameter::Number(self.value)])
	}
}
