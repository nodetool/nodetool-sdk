use std::{collections::HashMap, mem::discriminant, rc::Rc};

use thiserror::Error;

use crate::node::{AnyNode, Node, NodeParameter};

#[derive(Error, Debug)]
pub enum NodeConnectError {
	#[error("source node not found")]
	SourceNodeNotFound,
	#[error("target node not found")]
	TargetNodeNotFound,
	#[error("source node is the same as the target node")]
	SelfConnect,
	#[error("parameter types do not match")]
	ParameterMismatch,
}

#[derive(Error, Debug)]
pub enum NodeDisconnectError {
	#[error("node not found")]
	NodeNotFound,
}

#[derive(Error, Debug)]
pub enum GetNodeOutputsError {
	#[error("node not found")]
	NodeNotFound,
}

#[derive(Error, Debug)]
pub enum GetNodeInputsError {
	#[error("node not found")]
	NodeNotFound,
}

pub type NodeID = u64;

/// A graph containing nodes.
/// Each node is assigned a unique ID which is used in hashmaps and connections.
pub struct NodeGraph {
	pub nodes: HashMap<NodeID, AnyNode>,
	/// mapping of node connections.
	/// key is the node ID and index of the target node, value is the node ID and index of the source node.
	pub parameters: HashMap<(NodeID, usize), (NodeID, usize)>,
	/// mapping of node IDs and their output indices to connected nodes.
	pub connections: HashMap<(NodeID, usize), Vec<(NodeID, usize)>>,
	/// cache of node outputs.
	pub outputs: HashMap<NodeID, Rc<Vec<NodeParameter>>>,
	/// highest node ID.
	pub max_node_id: NodeID,
}

impl NodeGraph {
	pub fn new() -> Self {
		Self {
			nodes: HashMap::new(),
			parameters: HashMap::new(),
			connections: HashMap::new(),
			outputs: HashMap::new(),
			max_node_id: 0,
		}
	}

	/// add creates a new node and adds it to the graph, generating a new ID.
	pub fn add<T: Node + 'static>(&mut self, node: T) -> u64 {
		let id = self.max_node_id;
		self.nodes.insert(id, Rc::new(node));
		self.max_node_id += 1;
		id
	}

	/// invalidates a node's output in the cache.
	pub fn invalidate_node(&mut self, node_id: u64) {
		self.outputs.remove(&node_id);
	}

	/// gets all of the outputs of a node.
	/// first checks the cache if a node exists.
	/// if not found, it will go through the node graph and build node outputs.
	/// and then evaluate the node with the inputs.
	pub fn get_node_outputs(
		&mut self,
		node_id: u64,
	) -> Result<Rc<Vec<NodeParameter>>, GetNodeOutputsError> {
		// clones the ENTIRE vector of node parameters
		if let Some(outputs) = self.outputs.get(&node_id).cloned() {
			return Ok(outputs);
		}

		let node = Rc::clone(
			&self
				.nodes
				.get(&node_id)
				.ok_or(GetNodeOutputsError::NodeNotFound)?
				.clone(),
		);
		let len = node.inputs().len();
		let mut collected_inputs: Vec<Option<NodeParameter>> = Vec::with_capacity(len);
		for (i, entry) in collected_inputs.iter_mut().enumerate() {
			if let Some((node_id, parameter_index)) = self.parameters.get(&(node_id, i)).cloned() {
				let outputs = self.get_node_outputs(node_id)?;
				*entry = Some(outputs[parameter_index].clone());
			} else {
				*entry = None;
			}
		}

		let result = Rc::new(node.eval(collected_inputs));

		self.outputs.insert(node_id, Rc::clone(&result));

		Ok(result)
	}

	pub fn disconnect(
		&mut self,
		target_node: u64,
		target_index: usize,
	) -> Result<(), NodeDisconnectError> {
		self.parameters
			.remove(&(target_node, target_index))
			.ok_or(NodeDisconnectError::NodeNotFound)?;

		self.invalidate_node(target_node);

		Ok(())
	}

	pub fn connect(
		&mut self,
		from: u64,
		from_index: usize,
		to: u64,
		to_index: usize,
	) -> Result<(), NodeConnectError> {
		if from == to {
			return Err(NodeConnectError::SelfConnect);
		}

		let from_node = self
			.nodes
			.get(&from)
			.ok_or(NodeConnectError::SourceNodeNotFound)?;

		let to_node = self
			.nodes
			.get(&to)
			.ok_or(NodeConnectError::TargetNodeNotFound)?;

		let from_outputs = from_node.outputs();
		let to_inputs = to_node.inputs();

		if discriminant(&to_inputs[to_index].parameter_type)
			!= discriminant(&from_outputs[from_index].parameter_type)
		{
			return Err(NodeConnectError::ParameterMismatch);
		}

		self.parameters
			.entry((to, to_index))
			.or_insert((from, from_index));

		self.connections
			.entry((from, from_index))
			.or_insert(Vec::new())
			.push((to, to_index));

		Ok(())
	}
}

impl Default for NodeGraph {
	fn default() -> Self {
		Self::new()
	}
}