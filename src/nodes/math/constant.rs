use crate::node::{
	Node, NodeDescriptor, NodeParameter, NodeParameterDescriptor, NodeParameterType, NodeResult,
};

#[derive(Debug, Default)]
pub struct Constant {
	pub value: f64,
}

impl Node for Constant {
	fn eval(&mut self, _inputs: &[Option<crate::node::NodeParameter>]) -> NodeResult {
		Ok(vec![NodeParameter::Number(self.value)])
	}
}

pub fn descriptor() -> NodeDescriptor {
	NodeDescriptor {
		name: "constant".to_string(),
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
		node: Box::new(|| Box::new(Constant::default())),
	}
}
