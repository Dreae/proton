use syn;
use quote;

pub fn impl_entity(ast: &syn::MacroInput) -> quote::Tokens {
  let name = &ast.ident;
  quote! {
    use proton_shared::tier0::Entity;
    impl Entity for #name {
      fn set_pos(&mut self, pos: [f32; 3]) {
        self.base_entity.set_pos(pos);
      }

      fn get_pos(&self) -> [f32; 3] {
        self.base_entity.get_pos()
      }

      fn set_scale(&mut self, scale: [f32; 3]) {
        self.base_entity.set_scale(scale);
      }
      
      fn get_scale(&self) -> [f32; 3] {
        self.base_entity.get_scale()
      }

      fn _on_spawn_post(&mut self) {
        self.on_spawn_post();
      }
    }
  }
}