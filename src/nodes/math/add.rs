use crate::node::{Node, NodeParameter, NodeParameterDescriptor, NodeParameterType};

pub struct Add {}

impl Add {
	pub fn new() -> Self {
		Self {}
	}
}

impl Node for Add {
	fn inputs(&self) -> Vec<crate::node::NodeParameterDescriptor> {
		vec![
			NodeParameterDescriptor {
				name: "first",
				description: "The first input value",
				parameter_type: NodeParameterType::Number,
			},
			NodeParameterDescriptor {
				name: "second",
				description: "The second input value",
				parameter_type: NodeParameterType::Number,
			},
		]
	}

	fn outputs(&self) -> Vec<crate::node::NodeParameterDescriptor> {
		vec![NodeParameterDescriptor {
			name: "value",
			description: "The output value",
			parameter_type: NodeParameterType::Number,
		}]
	}

	fn eval(
		&self,
		inputs: Vec<Option<crate::node::NodeParameter>>,
	) -> Vec<crate::node::NodeParameter> {
		let first = inputs.get(0).unwrap().as_ref().unwrap();
		let second = inputs.get(1).unwrap().as_ref().unwrap();

		if let (NodeParameter::Number(first), NodeParameter::Number(second)) = (first, second) {
			return vec![NodeParameter::Number(first + second)];
		} else {
			panic!("invalid parameter types")
		}
	}
}
