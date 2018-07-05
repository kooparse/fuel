extern crate fuel_render;
extern crate fuel_types;
extern crate gltf;
extern crate nalgebra as na;

use fuel_render::{Mesh, Meshes, Model, Primitive, Vertex};
use fuel_types::Transform;
use na::{Vector2, Vector3};

pub struct Importer;

impl Importer {
    pub fn from_gltf(gltf_file_path: &str) -> Model {
        let (doc, buffers, _images) = gltf::import(gltf_file_path)
            .expect("Path for glTF file not valid.");

        let meshes: Meshes = doc.meshes()
            .map(|mesh| {
                let primitives: Vec<Primitive> = mesh.primitives()
                    .map(|prim| {
                        let reader = prim.reader(|buffer| {
                            Some(&buffers[buffer.index()])
                        });

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

                        [0; 1].into_iter().enumerate().for_each(
                            |(tex_index, _)| {
                                reader
                                    .read_tex_coords(tex_index as u32)
                                    .map(|tex_coords| {
                                        tex_coords.into_f32().enumerate()
                                    })
                                    .map(|tex_coords| {
                                        tex_coords.for_each(|(i, coord)| {
                                            match tex_index {
                                0 => {
                                    vertices[i].tex_coord_0 =
                                        Vector2::from(coord)
                                }
                                1 => {
                                    vertices[i].tex_coord_1 =
                                        Vector2::from(coord)
                                }
                                _ => println!("No tex_coord > 1 is permitted."),
                            }
                                        })
                                    });
                            },
                        );

                        let indices: Option<Vec<u32>> = reader
                            .read_indices()
                            .map(|indices| indices.into_u32().collect());

                        Primitive::new(&vertices, indices)
                    })
                    .collect();

                Mesh::new(primitives)
            })
            .collect();

        let transform = Transform {
            ..Default::default()
        };

        // TODO: Texture...
        // doc.textures()
        //     .for_each(|texture| Texture::from_gltf(texture, &buffers));

        Model::new(transform, meshes)
    }
    pub fn to_model(&self) {}
}
