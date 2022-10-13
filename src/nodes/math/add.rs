use crate::{
	extract_inputs,
	node::{
		Node, NodeDescriptor, NodeParameter, NodeParameterDescriptor, NodeParameterType, NodeResult,
	},
};

#[derive(Debug, Default)]
pub struct Add {}

impl Node for Add {
	fn eval(&mut self, inputs: &[Option<crate::node::NodeParameter>]) -> NodeResult {
		let (first, second) = extract_inputs!(inputs, Number, Number);
		Ok(vec![NodeParameter::Number(first + second)])
	}
}

pub fn descriptor() -> NodeDescriptor {
	NodeDescriptor {
		name: "add".to_string(),
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
		node: Box::new(|| Box::new(Add::default())),
	}
}
