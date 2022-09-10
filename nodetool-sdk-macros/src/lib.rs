use darling::{
	util::{path_to_string, PathList},
	FromDeriveInput, FromMeta,
};
use nodetool_sdk_core::node::NodeParameterType;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ExprArray, Ident, NestedMeta};

#[derive(FromDeriveInput)]
struct NodeParameterDeriveOpts {}

#[derive(FromDeriveInput)]
#[darling(attributes(nodetool), forward_attrs(allow, doc, cfg))]
struct NodeDeriveOpts {
	name: String,
	description: String,
	inputs: NestedMeta,
	outputs: ExprArray,
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
		description,
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

	quote! {
		impl #nodetool_core::node::NodeInfo for #ident {
			fn name(&self) -> &'static str {
				#name
			}

			fn inputs(&self) -> &[#nodetool_core::node::NodeParameterDescriptor] {
				&[#(#inputs),*]
			}

			fn outputs(&self) -> &[#nodetool_core::node::NodeParameterDescriptor] {
				&[#(#outputs),*]
			}
		}
	}
	.into()
}
