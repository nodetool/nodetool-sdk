use crate::node::{Node, NodeParameter, NodeParameterDescriptor, NodeParameterType, NodeResult};

pub struct Constant {
	pub value: f64,
	pub inputs: Vec<NodeParameterDescriptor>,
	pub outputs: Vec<NodeParameterDescriptor>,
}

impl Constant {
	pub fn new(value: f64) -> Self {
		Self {
			value,
			inputs: vec![NodeParameterDescriptor {
				name: "value".into(),
				description: "The constant value".into(),
				parameter_type: NodeParameterType::Number,
			}],
			outputs: vec![NodeParameterDescriptor {
				name: "value".into(),
				description: "The constant value".into(),
				parameter_type: NodeParameterType::Number,
			}],
		}
	}
}

impl Node for Constant {
	fn inputs(&self) -> &[crate::node::NodeParameterDescriptor] {
		&self.inputs
	}

	fn outputs(&self) -> &[crate::node::NodeParameterDescriptor] {
		&self.outputs
	}

	fn eval(&mut self, _inputs: Vec<Option<crate::node::NodeParameter>>) -> NodeResult {
		Ok(vec![NodeParameter::Number(self.value)])
	}
}
