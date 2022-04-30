use core::panic;

use nodetool_sdk::{
	node::NodeParameter,
	node_graph::NodeGraph,
	nodes::math::{add::Add, constant::Constant},
};

fn new_const(graph: &mut NodeGraph, value: f64) -> u64 {
	graph.add(Constant::new(value))
}

#[test]
pub fn test_add() -> anyhow::Result<()> {
	let mut node_graph = NodeGraph::new();

	// define two constants
	let const1 = new_const(&mut node_graph, 1.0);
	let const2 = new_const(&mut node_graph, 1.0);

	assert_ne!(const1, const2);

	let add_node = node_graph.add(Add::new());

	node_graph.connect(const1, 0, add_node, 0).unwrap();
	node_graph.connect(const2, 0, add_node, 1).unwrap();

	let output = node_graph.get_node_outputs(add_node)?;
	let data = inner::inner!(output.get(0).unwrap(), if NodeParameter::Number);
	assert_eq!(*data, 2.0);

	let const3 = new_const(&mut node_graph, 3.0);

	node_graph.disconnect(add_node, 0).unwrap();
	node_graph.connect(const3, 0, add_node, 0).unwrap();

	let output = node_graph.get_node_outputs(add_node)?;
	let data = inner::inner!(output.get(0).unwrap(), if NodeParameter::Number);
	assert_eq!(*data, 4.0);

	Ok(())
}
