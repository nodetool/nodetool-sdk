use crate::node::{Node, NodeParameter, NodeParameterDescriptor, NodeParameterType};

pub struct Constant {
	pub value: f64,
}

impl Constant {
	pub fn new(value: f64) -> Self {
		Self { value }
	}
}

impl Node for Constant {
	fn inputs(&self) -> Vec<crate::node::NodeParameterDescriptor> {
		vec![]
	}

	fn outputs(&self) -> Vec<crate::node::NodeParameterDescriptor> {
		vec![NodeParameterDescriptor {
			name: "value",
			description: "The constant value",
			parameter_type: NodeParameterType::Number,
		}]
	}

	fn eval(
		&self,
		_inputs: Vec<Option<crate::node::NodeParameter>>,
	) -> Vec<crate::node::NodeParameter> {
		vec![NodeParameter::Number(self.value)]
	}
}
