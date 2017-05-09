use assimp;
use tier0::{Model, Mesh, Vertex};
use std::collections::HashMap;
use std::cell::Cell;

pub struct ModelLoader {
  model_map: HashMap<String, *mut Model>,
}

impl ModelLoader {
  pub fn new() -> ModelLoader {
    ModelLoader {
      model_map: HashMap::new(),
    }
  }
  
  pub fn load_model(&mut self, path: &str) -> *mut Model {
    let queried_model = self.model_map.get(path).map(|&m| m);
    match queried_model {
      Some(model) => model,
      None => {
        let mut importer = assimp::Importer::new();
        importer.triangulate(true);
        importer.generate_normals(|x| x.enable = true);
        importer.pre_transform_vertices(|x| {
            x.enable = true;
            x.normalize = true
        });
        let scene = importer.read_file(path).unwrap();

        let meshes: Vec<Mesh> = scene.mesh_iter().map(|mesh| {
          let verts: Vec<Vertex> = mesh.vertex_iter().zip(mesh.normal_iter()).map(|(v, n)| {
            Vertex {
              position: v.into(),
              normal: n.into(),
              texcoord: [0.0, 0.0],
            }
          }).collect();

          let mut indices = Vec::with_capacity(mesh.num_faces() as usize * 3);
          for face in mesh.face_iter() {
            indices.push(face[0]);
            indices.push(face[1]);
            indices.push(face[2]);
          }

          Mesh::new(verts, indices)
        }).collect();

        let boxed = Box::new(Model::new(meshes));
        let raw_ptr = Box::into_raw(boxed);
        self.model_map.insert(path.to_owned(), raw_ptr);

        raw_ptr
      },
    }
  }
}
