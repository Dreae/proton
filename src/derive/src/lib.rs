extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
  let s = input.to_string();
  let ast = syn::parse_macro_input(&s).unwrap();

  let gen = impl_entity(&ast);

  gen.parse().unwrap()
}

fn impl_entity(ast: &syn::MacroInput) -> quote::Tokens {
  let name = &ast.ident;
  quote! {
    use proton_shared::tier0::Entity;
    impl Entity for #name {
      fn set_pos(&mut self, pos: [f32; 3]) {
        self.base_entity.set_pos(pos);
      }
    }
  }
}