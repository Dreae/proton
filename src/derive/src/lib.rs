extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

mod entity;

use proc_macro::TokenStream;

#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
  let s = input.to_string();
  let ast = syn::parse_macro_input(&s).unwrap();

  let gen = entity::impl_entity(&ast);

  gen.parse().unwrap()
}