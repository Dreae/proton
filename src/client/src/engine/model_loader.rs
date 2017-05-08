use assimp;
use engine::{Model, Mesh, Vertex};

pub fn load_model(path: &str) -> Model {
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

  Model::new(meshes)
}