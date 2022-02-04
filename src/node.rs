use std::{error::Error, rc::Rc};

// NodeParameter is the data type for a parameter that a node can take in
#[derive(Clone)]
pub enum NodeParameter {
	IntArray(Vec<i64>),
	Float64Array(Vec<f64>),
	String(String),
	Number(f64),
	Node(AnyNode),
	None,
}

pub enum NodeParameterType {
	IntArray,
	Float64Array,
	String,
	Number,
	Node,
	None,
}

// Descriptor of a node parameter including its type.
pub struct NodeParameterDescriptor<'a> {
	pub name: &'a str,
	pub description: &'a str,
	pub parameter_type: NodeParameterType,
}

pub trait Node {
	/// Returns a list of arguments/parameters the node takes,
	/// with metadata such as names and descriptions.
	fn inputs(&self) -> Vec<NodeParameterDescriptor>;

	/// Returns a list of outputs the node produces,
	/// with metadata such as names and descriptions.
	fn outputs(&self) -> Vec<NodeParameterDescriptor>;

	/// Evaluates the node and returns its output.
	fn eval(&self, inputs: Vec<Option<NodeParameter>>) -> Vec<NodeParameter>;
}

pub type AnyNode = Rc<dyn Node>;
pub type AnyError = Box<dyn Error>;
