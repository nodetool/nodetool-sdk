use std::{collections::HashMap, mem::discriminant, rc::Rc};

use thiserror::Error;

use crate::node::{AnyNode, NodeParameter};

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
pub enum GetNodeOutputsError {
	#[error("node not found")]
	NodeNotFound,
}

/// A graph containing nodes.
/// Each node is assigned a unique ID which is used in hashmaps and connections.
pub struct NodeGraph {
	nodes: HashMap<u64, AnyNode>,
	/// mapping of node connections.
	/// key is the node ID and index of the target node, value is the node ID and index of the source node.
	links: HashMap<(u64, usize), (u64, usize)>,
	// cache of node outputs.
	outputs: HashMap<u64, Rc<Vec<NodeParameter>>>,
}

impl NodeGraph {
	pub fn get_node_outputs(
		&mut self,
		node_id: u64,
	) -> Result<Rc<Vec<NodeParameter>>, GetNodeOutputsError> {
		if let Some(outputs) = self.outputs.get(&node_id).cloned() {
			return Ok(outputs);
		}

		// TODO: solve lifetime bullshit and make this only get the node once
		let node = self
			.nodes
			.get(&node_id)
			.ok_or(GetNodeOutputsError::NodeNotFound)?
			.clone();
		let len = node.inputs().len();
		let mut collected_inputs: Vec<Option<NodeParameter>> = Vec::with_capacity(len);
		for i in 0..len {
			if let Some((node_id, parameter_index)) = self.links.get(&(node_id, i)).cloned() {
				let outputs = self.get_node_outputs(node_id)?;
				collected_inputs.push(Some(outputs[parameter_index].clone()));
			} else {
				collected_inputs.push(None);
			}
		}

		let result = Rc::new(node.eval(collected_inputs));

		self.outputs.insert(node_id, result.clone());

		Ok(result)
	}

	pub fn connect(
		&mut self,
		from: u64,
		to: u64,
		from_index: usize,
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

		self.links
			.entry((to, to_index))
			.or_insert((from, from_index));

		Ok(())
	}
}
