use fuel_render::{Mesh, Meshes, Model, Primitive, Vertex};
use fuel_types::Transform;
use gltf;
use na::{Vector2, Vector3};
use Formater;

type TexCoords<'a> = Option<gltf::mesh::util::ReadTexCoords<'a>>;
type GltfPositions<'a> = gltf::accessor::Iter<'a, [f32; 3]>;
type Primitives = Vec<Primitive>;
type Vertices = Vec<Vertex>;
type Indices = Vec<u32>;

/// Format glTF file to fit with the render engine.
pub struct GltfFormater {
    pub meshes: Meshes,
    pub transform: Transform,
}

impl GltfFormater {
    pub fn new(gltf_file_path: &str) -> Self {
        let (document, buffers, _images) = gltf::import(gltf_file_path)
            .expect("glTF file not found or not valid.");

        let meshes: Meshes = document
            .meshes()
            .map(|mesh| {
                let primitives: Primitives = mesh.primitives()
                    .map(|prim| {
                        let reader = prim.reader(|buffer| {
                            Some(&buffers[buffer.index()])
                        });

                        let mut vertices: Vertices = reader
                            .read_positions()
                            .map(|positions| get_vertex(positions))
                            .unwrap_or(vec![]);

                        [0, 1].iter().for_each(|index| {
                            let coords = reader.read_tex_coords(*index);
                            tex_coords_on_vertices(
                                coords,
                                *index,
                                &mut vertices,
                            );
                        });

                        let indices: Option<Indices> = reader
                            .read_indices()
                            .map(|indices| indices.into_u32().collect());

                        Primitive::new(&vertices, indices)
                    })
                    .collect();

                Mesh::new(primitives)
            })
            .collect();

        // TODO: Texture
        // document
        //     .textures()
        //     .for_each(|texture| Texture::from_gltf(texture, &buffers));

        let transform = Transform {
            ..Default::default()
        };

        Self { meshes, transform }
    }
}

impl Formater for GltfFormater {
    fn to_model(self) -> Model {
        Model::new(self.transform, self.meshes)
    }
}

fn get_vertex(positions: GltfPositions) -> Vertices {
    positions
        .map(|position| Vertex {
            position: Vector3::from(position),
            normal: Vector3::from([1., 1., 1.]),
            ..Default::default()
        })
        .collect()
}

fn tex_coords_on_vertices(
    tex_coords: TexCoords,
    index: u32,
    vertices: &mut Vertices,
) {
    tex_coords
        .map(|tex_coords| tex_coords.into_f32().enumerate())
        .map(|tex_coords| {
            tex_coords.for_each(|(i, coord)| match index {
                0 => vertices[i].tex_coord_0 = Vector2::from(coord),
                1 => vertices[i].tex_coord_1 = Vector2::from(coord),
                _ => println!("No tex_coord > 1 is permitted."),
            })
        });
}
