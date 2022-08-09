use std::rc::Rc;

use crate::{node::NodeParameter, node_graph::NodeGraph};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct NodeParameterWasm {}

impl NodeGraph {
	pub fn connect_wasm(
		&mut self,
		from: u64,
		from_index: usize,
		to: u64,
		to_index: usize,
	) -> Result<(), JsError> {
		Ok(self.connect(from, from_index, to, to_index)?)
	}

	pub fn invalidate_node_wasm(&mut self, node_id: u64) {
		self.invalidate_node(node_id)
	}

	pub fn get_node_outputs_wasm(&mut self, node_id: u64) -> Result<Vec<NodeParameter>, JsError> {
		Ok(self.get_node_outputs(node_id)?.to_vec())
	}
}
