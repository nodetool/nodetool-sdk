use std::io::Write;

use crate::{
	extract_inputs,
	node::{
		Node, NodeDescriptor, NodeParameter, NodeParameterDescriptor, NodeParameterType, NodeResult,
	},
};

#[derive(Debug)]
pub struct File {
	pub current_path: Option<String>,
	pub current_append: Option<bool>,
	file: Option<std::fs::File>,
}

impl File {
	pub fn new() -> Self {
		Self {
			current_append: None,
			current_path: None,
			file: None,
		}
	}
}

impl Default for File {
	fn default() -> Self {
		Self::new()
	}
}

impl Node for File {
	fn eval(&mut self, inputs: &[Option<crate::node::NodeParameter>]) -> NodeResult {
		let (path, append, data) = extract_inputs!(inputs, String, Bool, String);

		let did_path_change = self
			.current_path
			.as_ref()
			.map_or(true, |current| current != path);

		if did_path_change || self.current_append != Some(*append) {
			self.file.replace(
				std::fs::OpenOptions::new()
					.append(*append)
					.create(true)
					.open(path)?,
			);
			self.current_path.replace(path.clone());
			self.current_append.replace(*append);
		}

		if let Some(ref mut file) = self.file {
			file.write_all(data.as_bytes())?;
		}

		Ok(vec![])
	}
}

pub fn descriptor() -> NodeDescriptor {
	NodeDescriptor {
		name: "file".to_string(),
		inputs: vec![
			NodeParameterDescriptor::new(
				"path",
				"The file path to write to",
				NodeParameterType::String,
			),
			NodeParameterDescriptor::new(
				"append",
				"whether to append to the file or to overwrite the contents",
				NodeParameterType::Bool,
			),
			NodeParameterDescriptor::new(
				"data",
				"the data to write to the file",
				NodeParameterType::String,
			),
		],
		outputs: vec![],
		node: Box::new(|| Box::new(File::new())),
	}
}
