use darling::{
	util::{path_to_string, PathList},
	FromDeriveInput,
};
use nodetool_sdk_core::node::NodeParameterType;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

#[derive(FromDeriveInput)]
#[darling(attributes(nodetool), forward_attrs(allow, doc, cfg))]
struct NodeDeriveOpts {
	name: String,
	inputs: PathList,
	outputs: PathList,
	ident: Ident,
}

#[cfg(feature = "internal")]
fn nodetool_core() -> TokenStream {
	quote! {::nodetool_sdk_core}
}

#[cfg(not(feature = "internal"))]
fn nodetool_core() -> TokenStream {
	quote! {::nodetool::core}
}

#[proc_macro_derive(Node, attributes(nodetool))]
pub fn node_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let nodetool_core = nodetool_core();

	let input = parse_macro_input!(input);
	let NodeDeriveOpts {
		name,
		inputs,
		outputs,
		ident,
	} = NodeDeriveOpts::from_derive_input(&input).expect("wrong options!");

	let inputs = inputs
		.iter()
		.map(|path| {
			let s = path_to_string(path);
			s.parse::<NodeParameterType>()
				.expect("input must be valid node param type");
			quote! {#nodetool_core::node::NodeParameterType::#path}
		})
		.collect::<Vec<_>>();

	let outputs = outputs
		.iter()
		.map(|path| {
			let s = path_to_string(path);
			s.parse::<NodeParameterType>()
				.expect("output must be valid node param type");
			quote! {#nodetool_core::node::NodeParameterType::#path}
		})
		.collect::<Vec<_>>();

	println!("{:?}", inputs);
	quote! {
		impl #nodetool_core::node::Node for #ident {
			fn inputs(&self) -> &[#nodetool_core::node::NodeParameterType] {
				&[#(#inputs),*]
			}

			fn outputs(&self) -> &[#nodetool_core::node::NodeParameterType] {
				&[#(#outputs),*]
			}
		}
	}
	.into()
}
