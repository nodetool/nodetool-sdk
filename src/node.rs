use std::{cell::RefCell, error::Error, fmt::Debug, rc::Rc};

use smol_str::SmolStr;

#[derive(Clone, Debug)]
pub enum NodeParameterType {
	IntArray,
	Float64Array,
	String,
	Number,
	Bool,
	Node,
	None,
}

// NodeParameter is the data type for a parameter that a node can take in
#[derive(Clone)]
pub enum NodeParameter {
	String(String),
	Number(f64),
	Bool(bool),
	Array(Vec<NodeParameter>),
	Node(AnyNode),
	None,
}

impl Debug for NodeParameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			NodeParameter::Node(n) => write!(f, "Node({:?})", n.as_ref().borrow().inputs()),
			NodeParameter::String(s) => write!(f, "String({:?})", s),
			NodeParameter::Number(n) => write!(f, "Number({:?})", n),
			NodeParameter::Bool(b) => write!(f, "Bool({:?})", b),
			NodeParameter::Array(a) => write!(f, "Array({:?})", a),
			NodeParameter::None => write!(f, "None"),
		}
	}
}

/// Descriptor of a node parameter including its type.
#[derive(Debug)]
pub struct NodeParameterDescriptor {
	pub name: SmolStr,
	pub description: SmolStr,
	pub parameter_type: NodeParameterType,
}

impl NodeParameterDescriptor {
	pub fn new(
		name: impl Into<SmolStr>,
		description: impl Into<SmolStr>,
		parameter_type: NodeParameterType,
	) -> Self {
		Self {
			name: name.into(),
			description: description.into(),
			parameter_type,
		}
	}
}

pub type NodeResult = Result<Vec<NodeParameter>, AnyError>;

pub trait Node {
	/// Returns a list of arguments/parameters the node takes,
	/// with metadata such as names and descriptions.
	fn inputs(&self) -> &[NodeParameterDescriptor];

	/// Returns a list of outputs the node produces,
	/// with metadata such as names and descriptions.
	fn outputs(&self) -> &[NodeParameterDescriptor];

	/// Evaluates the node and returns its output.
	fn eval(&mut self, inputs: Vec<Option<NodeParameter>>) -> NodeResult;
}

pub type NodeID = u64;
pub type AnyNode = Rc<RefCell<dyn Node>>;
pub type AnyError = Box<dyn Error + Send + Sync>;
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct NodeParamIndex(pub NodeID, pub usize);

impl Debug for NodeParamIndex {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("NodeParamIndex")
			.field("id", &self.0)
			.field("index", &self.1)
			.finish()
	}
}
