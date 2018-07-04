use fuel_camera::{Projection, View};
use fuel_core::{ObjectTypes, SceneObject};
use fuel_types::Transform;
use gltf;
use na::{Vector2, Vector3};
use primitive::Primitive;
use vertex::Vertex;

/// Model contains a list of Mesh that contains
/// a list of Primitive that contains a list of Vertex.
///
/// When we create a new Primitive, we setup automatically
/// openGL with his vertices. We don't keep it in memory.
///
/// Transform contains the position, scale and the rotation
/// of a Model.  When we update a Model, we also update the shader
/// inside the primitives.
pub struct Model {
    transform: Transform,
    meshes: Vec<Mesh>,
}

impl SceneObject for Model {
    fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.transform.position.set(x, y, z);
    }

    fn get_type(&self) -> ObjectTypes {
        ObjectTypes::MODEL
    }

    fn render(&self, proj: Projection, view: View) {
        self.meshes.iter().for_each(|mesh| {
            mesh.draw(proj, view, &self.transform);
        });
    }

    fn set_color(&self, _name: &str, _color: Vector3<f32>) {}
    fn set_scale(&mut self, _scale: f32) {}
}

impl Model {
    pub fn from_gltf(file_path: &str) -> Self {
        let (doc, buffers, _) =
            gltf::import(file_path).expect("Path for glTF file not valid.");

        let meshes = doc.meshes()
            .enumerate()
            .map(|(_, mesh)| Mesh::from_gltf(mesh, &buffers))
            .collect();

        let transform = Transform {
            ..Default::default()
        };
        Model { meshes, transform }
    }
}

pub struct Mesh {
    primitives: Vec<Primitive>,
}

impl Mesh {
    pub fn from_gltf(
        mesh: gltf::Mesh,
        buffers: &Vec<gltf::buffer::Data>,
    ) -> Self {
        let primitives: Vec<Primitive> = mesh.primitives()
            .enumerate()
            .map(|(_, prim)| {
                let reader =
                    prim.reader(|buffer| Some(&buffers[buffer.index()]));

                let mut vertices: Vec<Vertex> = reader
                    .read_positions()
                    .map(|positions| {
                        positions
                            .map(|position| Vertex {
                                position: Vector3::from(position),
                                normal: Vector3::from([1., 1., 1.]),
                                ..Default::default()
                            })
                            .collect()
                    })
                    .unwrap_or(vec![]);

                [0; 1].into_iter().enumerate().for_each(|(tex_index, _)| {
                    reader
                        .read_tex_coords(tex_index as u32)
                        .map(|tex_coords| tex_coords.into_f32().enumerate())
                        .map(|tex_coords| {
                            tex_coords.for_each(|(i, coord)| match tex_index {
                                0 => {
                                    vertices[i].tex_coord_0 =
                                        Vector2::from(coord)
                                }
                                1 => {
                                    vertices[i].tex_coord_1 =
                                        Vector2::from(coord)
                                }
                                _ => println!("No tex_coord > 1 is permitted."),
                            })
                        });
                });

                // reader.read_normals().iter().enumerate().for_each(
                //     |(i, normal)| {
                //         let normlals = normal.enumerate().for_each(|(_, n)| {
                //             println!("{:?}", n);
                //             vertices[i].normal = Vector3::from(n);
                //         });
                //     },
                // );

                let indices: Option<Vec<u32>> = reader
                    .read_indices()
                    .map(|indices| indices.into_u32().collect());

                Primitive::new(&vertices, indices)
            })
            .collect();

        Mesh { primitives }
    }

    fn draw(&self, proj: Projection, view: View, transform: &Transform) {
        self.primitives.iter().for_each(|primitive| {
            primitive.shader_config(proj, view, transform);
        })
    }
}
