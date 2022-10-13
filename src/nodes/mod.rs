use crate::{node::NodeDescriptor, nodes::math::constant};
pub mod io;
pub mod math;

pub fn get_registered_nodes() -> Vec<NodeDescriptor> {
	vec![constant::descriptor()]
}
