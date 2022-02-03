use std::error::Error;

// NodeParameter is the data type for a parameter that a node can take in
#[derive(Clone)]
pub enum NodeParameter {
	IntArray(Option<Vec<i64>>),
	Float64Array(Option<Vec<f64>>),
	String(Option<String>),
	Number(Option<f64>),
	None,
}

// Descriptor of a node parameter including its type.
pub struct NodeParameterDescriptor<'a> {
	pub name: &'a str,
	pub description: &'a str,
	pub parameter_type: NodeParameter,
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

pub type AnyNode = Box<dyn Node>;
pub type AnyError = Box<dyn Error>;
