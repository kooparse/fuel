use fuel_camera::{Projection, View};
use fuel_types::{Position, Transform, EBO, VAO, VBO};
use gl;
use na::Isometry3;
use shader::Shader;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use vertex::Vertex;

macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(0 as *const $ty)).$field as *const _ as usize
    };
}

pub struct Primitive {
    pub vao: VAO,
    pub vbo: VBO,
    pub shader: Shader,
    pub ebo: Option<EBO>,
    pub num_indices: u32,
    pub num_vertices: u32,
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
            shader: Shader::new("cube"),
        };

        unsafe {
            primitive.setup(&vertices, indices);
        }
        primitive
    }

    pub fn shader_config(
        &self,
        proj: Projection,
        view: View,
        transform: &Transform,
    ) {
        self.shader.use_program();
        // Building model
        let (position, rotation, scale) = transform.get();
        let mut model = Isometry3::new(position, rotation).to_homogeneous();
        model = model.append_scaling(scale);
        self.shader.set_mvp(proj * view * model);

        unsafe {
            gl::BindVertexArray(self.vao);

            if self.ebo.is_some() {
                gl::DrawElements(
                    gl::TRIANGLES,
                    self.num_indices as i32,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            } else {
                gl::DrawArrays(gl::TRIANGLES, 0, self.num_vertices as i32);
            }

            gl::BindVertexArray(self.vao);
            gl::ActiveTexture(gl::TEXTURE0);
        }
    }

    pub unsafe fn setup(
        &mut self,
        vertices: &[Vertex],
        indices: Option<Vec<u32>>,
    ) {
        let mut array: Vec<f32> = vec![];
        vertices.iter().for_each(|vert| {
            let pos = vert.position;
            array.push(pos.x);
            array.push(pos.y);
            array.push(pos.z);
        });

        vertices.iter().for_each(|vertex| {
            println!("{:?}", vertex.position);
        });

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

        let size = (array.len() * mem::size_of::<u32>()) as isize;
        let data = &array[0] as *const f32 as *const c_void;

        println!("size: {}", size);
        println!("data: {:?}", data);

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

        let size = mem::size_of::<Position>() as i32;
        // Position
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, position) as *const c_void,
        );
        gl::EnableVertexAttribArray(0);

        // Normal
        // gl::EnableVertexAttribArray(1);
        // gl::VertexAttribPointer(
        //     0,
        //     3,
        //     gl::FLOAT,
        //     gl::FALSE,
        //     size,
        //     offset_of!(Vertex, normal) as *const c_void,
        // );

        // // Tex_coord_0
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, tex_coord_0) as *const c_void,
        );
        // // Tex_coord_1
        // gl::EnableVertexAttribArray(2);
        // gl::VertexAttribPointer(
        //     2,
        //     2,
        //     gl::FLOAT,
        //     gl::FALSE,
        //     size,
        //     offset_of!(Vertex, tex_coord_1) as *const c_void,
        // );

        gl::BindVertexArray(0);
    }
}
