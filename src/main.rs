#[macro_use]
extern crate glium;

mod fluid;
mod types;
use glium::backend::glutin_backend::GlutinFacade;
use std::time::Instant;
use types::*;

#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 3],
    uv: [f32; 3],
}
implement_vertex!(Vertex, pos, uv);

static VERT_SRC: &'static str = r#"
    #version 140

    in vec3 pos;
    in vec3 uv;

    out vec3 v_uv;

    void main() {
      v_uv = uv;
      gl_Position = vec4(pos, 1.0);
    }
"#;

static FRAG_SRC: &'static str = r#"
    #version 140

    uniform highp sampler3D tex;
    uniform highp sampler3D tex1;
    uniform highp sampler3D tex2;
    uniform highp sampler3D tex3;

    in vec3 v_uv;

    out vec4 color;

    void main() {
      color = vec4(texture(tex, v_uv).x, texture(tex1, v_uv).x, texture(tex2, v_uv).x, texture(tex3, v_uv).x);
    }
"#;

fn setup_display() -> GlutinFacade {
    use glium::DisplayBuild;
    glium::glutin::WindowBuilder::new().build_glium().unwrap()
}

fn setup_shader(display: &GlutinFacade) -> glium::Program {
    glium::Program::from_source(display, VERT_SRC, FRAG_SRC, None).unwrap()
}

fn main() {
    let display = setup_display();
    let shader = setup_shader(&display);
    let (display_w, display_h) = display.get_window().unwrap().get_inner_size().unwrap();
    //println!("Display size: {}, {}", x_size, y_size);

    let vbo_data = vec![
        Vertex {
            pos: [-1.0, 1.0, -1.0],
            uv: [0.0, 1.0, 0.0],
        },
        Vertex {
            pos: [1.0, 1.0, -1.0],
            uv: [1.0, 1.0, 0.0],
        },
        Vertex {
            pos: [1.0, -1.0, 1.0],
            uv: [1.0, 0.0, 1.0],
        },
        Vertex {
            pos: [-1.0, 1.0, -1.0],
            uv: [0.0, 1.0, 0.0],
        },
        Vertex {
            pos: [-1.0, -1.0, 1.0],
            uv: [0.0, 0.0, 1.0],
        },
        Vertex {
            pos: [1.0, -1.0, 1.0],
            uv: [1.0, 0.0, 1.0],
        },
    ];

    let vbo = glium::VertexBuffer::new(&display, &vbo_data).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // Create texture data buffer for fluid

    let mut wind_grid = WindGrid {
        x_vel: vec![0_f32; SIZE_1D].into_boxed_slice(),
        y_vel: vec![0_f32; SIZE_1D].into_boxed_slice(),
        z_vel: vec![0_f32; SIZE_1D].into_boxed_slice(),
        density: vec![0_f32; SIZE_1D].into_boxed_slice(),
    };

    let mut last_t = Instant::now();

    let mut fps_list = Vec::<u128>::new();
    // let mut t = Duration::default();
    let mut mouse_x = 0.0;
    let mut mouse_y = 0.0;
    loop {
        let new_now = Instant::now();
        let dt = new_now.duration_since(last_t);
        let ms = dt.as_millis();
        if ms > 0 {
            let fps = 1000 / ms;
            fps_list.push(fps);
            let avg_fps: u128 =
                (fps_list.iter().fold(0, |x, y| (x + y) as u128) / fps_list.len() as u128) as u128;
            println!("{} (total avg: {})", fps, avg_fps);
        }
        last_t = new_now;
        // listing the events produced by the window and waiting to be received

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return, // the window has been closed by the user
                glium::glutin::Event::MouseMoved(x, y) => {
                    mouse_x = (x as f32 / display_w as f32) * X_SIZE as f32;
                    mouse_y = ((display_h as f32 - y as f32) / display_h as f32) * Y_SIZE as f32;

                    wind_grid.add_velocity_source(
                        Pos {
                            x: mouse_x as usize,
                            y: mouse_y as usize,
                            z: 2,
                        },
                        Vel {
                            x: 200.0,
                            y: 200.0,
                            z: 20.0,
                        },
                    );
                }
                glium::glutin::Event::MouseInput(
                    glium::glutin::ElementState::Pressed,
                    glium::glutin::MouseButton::Left,
                ) => wind_grid.add_density_source(
                    Pos {
                        x: mouse_x as usize,
                        y: mouse_y as usize,
                        z: 2,
                    },
                    330.0,
                ),
                _ => (),
            }
        }

        //Adding point velocity sources to the grid

        // Process fluids
        fluid::step_fluid(
            &mut wind_grid.density,
            &mut wind_grid.x_vel,
            &mut wind_grid.y_vel,
            &mut wind_grid.z_vel,
            0.001,
            0.1,
            true,
        );

        // Re buffer texture
        use std::borrow::Cow;
        let raw_tex_3d = glium::texture::RawImage3d {
            data: Cow::from(wind_grid.density[..].to_vec()),
            width: X_SIZE as u32 + 2,
            height: Y_SIZE as u32 + 2,
            depth: Z_SIZE as u32 + 2,
            format: glium::texture::ClientFormat::F32,
        };

        let raw_tex_3d1 = glium::texture::RawImage3d {
            data: Cow::from(wind_grid.x_vel[..].to_vec()),
            width: X_SIZE as u32 + 2,
            height: Y_SIZE as u32 + 2,
            depth: Z_SIZE as u32 + 2,
            format: glium::texture::ClientFormat::F32,
        };

        let raw_tex_3d2 = glium::texture::RawImage3d {
            data: Cow::from(wind_grid.y_vel[..].to_vec()),
            width: X_SIZE as u32 + 2,
            height: Y_SIZE as u32 + 2,
            depth: Z_SIZE as u32 + 2,
            format: glium::texture::ClientFormat::F32,
        };
        let raw_tex_3d3 = glium::texture::RawImage3d {
            data: Cow::from(wind_grid.z_vel[..].to_vec()),
            width: X_SIZE as u32 + 2,
            height: Y_SIZE as u32 + 2,
            depth: Z_SIZE as u32 + 2,
            format: glium::texture::ClientFormat::F32,
        };

        let texture = glium::texture::Texture3d::new(&display, raw_tex_3d).unwrap();
        let texture1 = glium::texture::Texture3d::new(&display, raw_tex_3d1).unwrap();
        let texture2 = glium::texture::Texture3d::new(&display, raw_tex_3d2).unwrap();
        let texture3 = glium::texture::Texture3d::new(&display, raw_tex_3d3).unwrap();
        // Load texture into uniforms

        let uniforms = uniform! {
          tex: texture.sampled()
            .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
            tex1: texture1.sampled()
              .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
              .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
              tex2: texture2.sampled()
                .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
                tex3: texture3.sampled()
                .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear)
        };

        // Draw
        use glium::Surface;
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target
            .draw(&vbo, &indices, &shader, &uniforms, &Default::default())
            .unwrap();
        target.finish().unwrap();
    }
}
