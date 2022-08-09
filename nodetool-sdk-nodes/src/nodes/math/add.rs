use nodetool_sdk_core::{
	extract_inputs,
	node::{Node, NodeParameter, NodeParameterDescriptor, NodeParameterType, NodeResult},
};
use nodetool_sdk_macros::Node;

#[derive(Node)]
#[nodetool(name = "Add", inputs(Number, Number), outputs(Number))]
pub struct Add;

impl Node for Add {
	fn eval(&mut self, inputs: Vec<Option<NodeParameter>>) -> NodeResult {
		let (first, second) = extract_inputs!(inputs, Number, Number);
		Ok(vec![NodeParameter::Number(first + second)])
	}
}
