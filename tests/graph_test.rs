use core::panic;
use std::borrow::Borrow;

use nodetool_sdk::{
	node::NodeParameter,
	node_graph::NodeGraph,
	nodes::math::{add::Add, constant::Constant},
};

fn new_const(graph: &mut NodeGraph, value: f32, id: u64) {
	graph.add(Constant::new(1.0), id);
}

#[test]
pub fn test_add() -> anyhow::Result<()> {
	let mut node_graph = NodeGraph::new();

	// define two constants
	new_const(&mut node_graph, 1.0, 0);
	new_const(&mut node_graph, 1.0, 1);

	node_graph.add(Add::new(), 2);
	node_graph.connect(0, 0, 2, 0).unwrap();
	node_graph.connect(1, 0, 2, 1).unwrap();

	let output = node_graph.get_node_outputs(2)?;
	let data = inner::inner!(output.get(0).unwrap(), if NodeParameter::Number);
	assert_eq!(*data, 2.0);
	Ok(())
}
