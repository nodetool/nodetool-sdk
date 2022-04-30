use crate::node::{Node, NodeParameter};

#[test]
pub fn test_file_node() -> anyhow::Result<()> {
	let mut node = super::file::File::default();
	let mut values = vec![
		Some(NodeParameter::String("/tmp/test.txt".to_string())),
		Some(NodeParameter::Bool(true)),
		Some(NodeParameter::String("hiii".to_string())),
	];

	node.eval(values.clone()).unwrap();
	values[0] = Some(NodeParameter::String("/tmp/test1.txt".to_string()));
	node.eval(values.clone()).unwrap();

	Ok(())
}
