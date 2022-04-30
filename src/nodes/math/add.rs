use crate::{
	extract_inputs,
	node::{Node, NodeParameter, NodeParameterDescriptor, NodeParameterType, NodeResult},
};

pub struct Add {
	inputs: Vec<NodeParameterDescriptor>,
	outputs: Vec<NodeParameterDescriptor>,
}

impl Add {
	pub fn new() -> Self {
		Self {
			inputs: vec![
				NodeParameterDescriptor::new(
					"first",
					"The first input value",
					NodeParameterType::Number,
				),
				NodeParameterDescriptor::new(
					"second",
					"The second input value",
					NodeParameterType::Number,
				),
			],
			outputs: vec![NodeParameterDescriptor::new(
				"value",
				"The output value",
				NodeParameterType::Number,
			)],
		}
	}
}

impl Default for Add {
	fn default() -> Self {
		Self::new()
	}
}

impl Node for Add {
	fn inputs(&self) -> &[crate::node::NodeParameterDescriptor] {
		&self.inputs
	}

	fn outputs(&self) -> &[crate::node::NodeParameterDescriptor] {
		&self.outputs
	}

	fn eval(&self, inputs: Vec<Option<crate::node::NodeParameter>>) -> NodeResult {
		let (first, second) = extract_inputs!(inputs, Number, Number);
		Ok(vec![NodeParameter::Number(first + second)])
	}
}
