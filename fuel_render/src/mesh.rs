use fuel_core::{ObjectTypes, SceneObject};
use fuel_types::{Transform, EBO, VAO, VBO};
use gl;
use gltf;
use na::{Vector2, Vector3, Vector4};
use shader::Shader;
use std::mem;
use std::os::raw::c_void;
use texture::Texture;

macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        unsafe { &(*(0 as *const $ty)).$field as *const _ as usize }
    };
}

struct Vertex {
    position: Vector3<f32>,
    tex_coord_0: Vector2<f32>,
    tex_coord_1: Vector2<f32>,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vector3::zeros(),
            tex_coord_0: Vector2::zeros(),
            tex_coord_1: Vector2::zeros(),
        }
    }
}

struct Primitive {
    vao: VAO,
    vbo: VBO,
    ebo: Option<EBO>,
    num_indices: u32,
    num_vertices: u32,
}

impl Primitive {
    pub fn new(vertices: &[Vertex], indices: Option<Vec<u32>>) -> Self {
        let num_indices = indices.as_ref().map(|i| i.len()).unwrap_or(0) as u32;
        let mut primitive = Primitive {
            vao: 0,
            vbo: 0,
            ebo: None,
            num_indices,
            num_vertices: vertices.len() as u32,
        };

        unsafe {
            primitive.setup_gl(&vertices, indices);
        }
        primitive
    }

    fn setup_shader() {
        let shader = Shader::new("cube");
        shader.set_color("objectColor", Vector3::new(1., -0.5, 0.31));
    }

    unsafe fn setup_gl(
        &mut self,
        vertices: &[Vertex],
        indices: Option<Vec<u32>>,
    ) {
        gl::Enable(gl::DEPTH_TEST);

        // Generate our array then load it
        gl::GenVertexArrays(1, &mut self.vao);
        gl::BindVertexArray(self.vao);

        // Generate buffer then load it
        gl::GenBuffers(1, &mut self.vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

        if indices.is_some() {
            let mut ebo = 0;
            gl::GenBuffers(1, &mut ebo);
            self.ebo = Some(ebo);
        }

        let size = (vertices.len() * mem::size_of::<u32>()) as isize;
        let data = &vertices[0] as *const Vertex as *const c_void;

        gl::BufferData(
            gl::ARRAY_BUFFER,
            size,
            data,
            // Tell the GPU if our data are likely to change frequently
            gl::STATIC_DRAW,
        );

        if let Some(ebo) = self.ebo {
            let indices = indices.expect("Error GL while setting up ebo.");
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            let size = (indices.len() * mem::size_of::<u32>()) as isize;
            let data = &indices[0] as *const u32 as *const c_void;
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                size,
                data,
                gl::STATIC_DRAW,
            );
        }

        let size = mem::size_of::<Vertex>() as i32;
        // Position
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, position) as *const c_void,
        );

        // Tex_coord_0
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, tex_coord_0) as *const c_void,
        );
        // Tex_coord_1
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, tex_coord_1) as *const c_void,
        );

        gl::BindVertexArray(0);
    }
}

pub struct Mesh {
    primitives: Vec<Primitive>,
}

pub struct Model {
    transform: Transform,
    meshes: Vec<Mesh>,
}

// impl SceneObject for Model {
//     fn set_scale(&mut self, scale: f32) {
//         self.transform.scale.set(scale);
//     }
//     fn get_type(&self) -> ObjectTypes {
//         ObjectTypes::MODEL
//     }
// }

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

                let mut vertices: Vec<Vertex> = vec![];

                if let Some(positions) = reader.read_positions() {
                    positions.enumerate().for_each(|(_, vertex_position)| {
                        vertices.push(Vertex {
                            position: Vector3::from(vertex_position),
                            ..Default::default()
                        });
                    })
                }

                for tex_index in 0..1 {
                    if let Some(tex_coords) = reader.read_tex_coords(tex_index)
                    {
                        tex_coords.into_f32().enumerate().for_each(
                            |(i, coord)| match tex_index {
                                0 => {
                                    vertices[i].tex_coord_0 =
                                        Vector2::from(coord)
                                }
                                1 => {
                                    vertices[i].tex_coord_0 =
                                        Vector2::from(coord)
                                }
                                _ => println!("No tex_coord > 1 is permitted."),
                            },
                        )
                    }
                }

                let mut indices: Option<Vec<u32>> = reader
                    .read_indices()
                    .map(|indices| indices.into_u32().collect());

                Primitive::new(&vertices, indices)
            })
            .collect();

        Mesh { primitives }
    }
}
