use std::{borrow::Borrow, cell::RefCell, collections::HashMap, mem::discriminant, ops::DerefMut};

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
	nodes: HashMap<u64, RefCell<AnyNode>>,
	/// mapping of node connections.
	/// key is the node ID and index of the target node, value is the node ID and index of the source node.
	links: HashMap<(u64, usize), (u64, usize)>,
	// cache of node outputs.
	outputs: HashMap<u64, Vec<NodeParameter>>,
}

impl NodeGraph {
	fn get_node(&mut self, node_id: u64) -> Option<&mut AnyNode> {
		self.nodes.get_mut(&node_id).map(|node| node.get_mut())
	}

	pub fn get_node_outputs(
		&mut self,
		node_id: u64,
	) -> Result<&Vec<NodeParameter>, GetNodeOutputsError> {
		// if let Some(outputs) = self.outputs.get_mut(&node_id) {
		// 	return Ok(outputs);
		// }

		// TODO: solve lifetime bullshit and make this only get the node once
		let len = {
			self.get_node(node_id)
				.ok_or(GetNodeOutputsError::NodeNotFound)?
				.inputs()
				.len()
		};
		let mut collected_inputs: Vec<Option<NodeParameter>> = Vec::with_capacity(len);
		for i in 0..len {
			if let Some(link) = self.links.get(&(node_id, i)) {
				let node_id = link.0;
				let parameter_index = link.1;
				let outputs = self.get_node_outputs(node_id)?;
				collected_inputs.push(Some(outputs[parameter_index].clone()));
			} else {
				collected_inputs.push(None);
			}
		}

		let node = self
			.get_node(node_id)
			.ok_or(GetNodeOutputsError::NodeNotFound)?;

		let result = &node.eval(collected_inputs);

		// self.outputs.insert(node_id, result);

		self.outputs
			.get(&node_id)
			.ok_or(GetNodeOutputsError::NodeNotFound)
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
			.ok_or(NodeConnectError::SourceNodeNotFound)?
			.borrow();

		let to_node = self
			.nodes
			.get(&to)
			.ok_or(NodeConnectError::TargetNodeNotFound)?
			.borrow();

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
