#[macro_use]
extern crate glium;

mod fluid;

use fluid::*;
use glium::backend::glutin_backend::GlutinFacade;

#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 2],
    uv: [f32; 2],
}
implement_vertex!(Vertex, pos, uv);

static VERT_SRC: &'static str = r#"
    #version 140

    in vec2 pos;
    in vec2 uv;

    out vec2 v_uv;

    void main() {
      v_uv = uv;
      gl_Position = vec4(pos, 0.0, 1.0);
    }
"#;

static FRAG_SRC: &'static str = r#"
    #version 140

    uniform highp sampler2D tex;
    uniform highp sampler2D tex1;
    uniform highp sampler2D tex2;

    in vec2 v_uv;

    out vec4 color;

    void main() {
      color = vec4(texture(tex, v_uv).x, texture(tex1, v_uv).x, texture(tex2, v_uv).x, 1.0);
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

    let vbo_data = vec![
        Vertex {
            pos: [-1.0, -1.0],
            uv: [0.0, 0.0],
        },
        Vertex {
            pos: [1.0, -1.0],
            uv: [1.0, 0.0],
        },
        Vertex {
            pos: [1.0, 1.0],
            uv: [1.0, 1.0],
        },
        Vertex {
            pos: [-1.0, -1.0],
            uv: [0.0, 0.0],
        },
        Vertex {
            pos: [-1.0, 1.0],
            uv: [0.0, 1.0],
        },
        Vertex {
            pos: [1.0, 1.0],
            uv: [1.0, 1.0],
        },
    ];

    let vbo = glium::VertexBuffer::new(&display, &vbo_data).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // Set up fluids

    // Velocity grids
    let mut vx_grid_ = [0_f32; TEX_SIZE * TEX_SIZE];
    let vx_grid = vx_grid_.to_vec();
    let mut vy_grid_ = [0_f32; TEX_SIZE * TEX_SIZE];
    let vy_grid = vy_grid_.to_vec();
    let mut tex_data = [0.0; TEX_SIZE * TEX_SIZE].to_vec();
    // Create texture data buffer for fluid

    let (mut prev_mx, mut prev_my) = (0, 0);

    // 4 tuple, ABXY, AB for position and XY for velocity. Every frame the
    //   velocity cell this correcponds to gets set to XY.
    loop {
        // listing the events produced by the window and waiting to be received
        let mut grid = Grid::new(&mut vx_grid_, &mut vy_grid_);
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return, // the window has been closed by the user
                glium::glutin::Event::MouseMoved(x, y) => {
                    let pos = Pos {
                        x: ((x as f32 / display_w as f32) * TEX_SIZE as f32) as usize,
                        y: (((display_h as f32 - y as f32) / display_h as f32) * TEX_SIZE as f32)
                            as usize,
                    };

                    //println!("{}, {}", x, y);
                    let ix = pos.x + pos.y * TEX_SIZE;
                    if ix >= tex_data.len() {
                        continue;
                    }

                    tex_data[pos.x + pos.y * TEX_SIZE] = 1.0;

                    let mut vel = Vel::default();
                    if prev_mx != 0 && prev_my != 0 {
                        vel.x = (pos.x as f32 - prev_mx as f32) * 2000.0;
                        vel.y = (pos.y as f32 - prev_my as f32) * 2000.0;
                    }
                    prev_mx = pos.x;
                    prev_my = pos.y;

                    grid.set(pos, vel);
                }
                _ => (),
            }
        }

        // Process fluids
        fluid::step_fluid(
            &mut tex_data[..],
            &mut grid,
            TEX_SIZE as u32,
            0.016,
            0.0001,
            true,
        );

        // Re buffer texture
        use std::borrow::Cow;
        let raw_tex_2d = glium::texture::RawImage2d {
            data: Cow::from(tex_data.clone()),
            width: TEX_SIZE as u32,
            height: TEX_SIZE as u32,
            format: glium::texture::ClientFormat::F32,
        };
        let raw_tex_2d1 = glium::texture::RawImage2d {
            data: Cow::from(vx_grid.clone()),
            width: TEX_SIZE as u32,
            height: TEX_SIZE as u32,
            format: glium::texture::ClientFormat::F32,
        };
        let raw_tex_2d2 = glium::texture::RawImage2d {
            data: Cow::from(vy_grid.clone()),
            width: TEX_SIZE as u32,
            height: TEX_SIZE as u32,
            format: glium::texture::ClientFormat::F32,
        };
        let texture = glium::texture::Texture2d::new(&display, raw_tex_2d).unwrap();
        let texture1 = glium::texture::Texture2d::new(&display, raw_tex_2d1).unwrap();
        let texture2 = glium::texture::Texture2d::new(&display, raw_tex_2d2).unwrap();
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
