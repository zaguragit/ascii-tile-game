use std::{sync::mpsc::Receiver, fs};

use glam::{Mat4, vec3, vec2};
use glfw::{Action, Context, Key, Window, WindowEvent, Glfw};

use crate::{Scene, UpdateResult, char_buffer::CharBuffer, key::KeyBuffer};
use crate::assets::{mesh::Mesh, shader::Shader, tileset::Tileset};

pub fn game_loop(
    title: &str,
    text_size: (usize, usize),
    tileset_data: (&str, usize),
    first_scene: Box<dyn Scene>,
) {
    let (mut glfw, mut window, events) = start(text_size.0, text_size.1, title);
    let (mut shader, quad, tileset) = make_assets(tileset_data);
    let mut context = crate::Context {
        seconds_per_key_hold_tick: 0.1
    };

    let mut scene = first_scene;
    scene.on_attach(&mut context, &mut glfw);
    let mut chars = CharBuffer::new(text_size.0, text_size.1, scene.as_ref());
    chars.bind();

    let mut key_buffer = KeyBuffer::new();
    let mut last_key_execution = 0.0;

    unsafe {
        quad.bind();
        shader.bind();
        tileset.bind(0);
    }

    let mut post_transform: Mat4 = {
        let (window_width, window_height) = window.get_framebuffer_size() as _;
        make_window_transform_matrix(window_width, window_height, text_size)
    };
    unsafe {
        shader.setm4("window_matrix", post_transform);
        shader.setv2("text_size", vec2(text_size.0 as f32, text_size.1 as f32));
    }
    
    unsafe { gl::ClearColor(0.0, 0.0, 0.0, 1.0) };
    while !window.should_close() {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

        unsafe {
            quad.draw();
        }

        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height) }
                    post_transform = make_window_transform_matrix(width, height, text_size);
                    unsafe {
                        shader.setm4("window_matrix", post_transform);
                    }
                }
                glfw::WindowEvent::Key(key, _, action, _) => match get_key_from_glfw(key) {
                    Some(k) => {
                        key_buffer.on_event(k, action);
                        if action == Action::Press {
                            last_key_execution = glfw.get_time();
                            match scene.on_input(k, &mut context, &mut glfw) {
                                UpdateResult::Update => {
                                    chars = CharBuffer::new(text_size.0, text_size.1, scene.as_ref());
                                    chars.bind();
                                },
                                UpdateResult::NoChange => {},
                                UpdateResult::SwitchScene(s) => {
                                    scene = s;
                                    scene.on_attach(&mut context, &mut glfw);
                                    chars = CharBuffer::new(text_size.0, text_size.1, scene.as_ref());
                                    chars.bind();
                                },
                                UpdateResult::Quit => window.set_should_close(true),
                            }
                        }
                    },
                    _ => {}
                },
                _ => {},
            }
        }
        let now = glfw.get_time();
        if key_buffer.pressed_key_counter != 0 && now - last_key_execution >= context.seconds_per_key_hold_tick {
            last_key_execution = now;
            let mut keys_handled = 0;
            for key in key_buffer.data.keys() {
                if key_buffer.data[key] {
                    keys_handled += 1;
                    match scene.on_input(*key, &mut context, &mut glfw) {
                        UpdateResult::Update => {
                            chars = CharBuffer::new(text_size.0, text_size.1, scene.as_ref());
                            chars.bind();
                        },
                        UpdateResult::NoChange => {},
                        UpdateResult::SwitchScene(s) => {
                            scene = s;
                            scene.on_attach(&mut context, &mut glfw);
                            chars = CharBuffer::new(text_size.0, text_size.1, scene.as_ref());
                            chars.bind();
                        },
                        UpdateResult::Quit => window.set_should_close(true),
                    }
                    if keys_handled == key_buffer.pressed_key_counter {
                        break;
                    }
                }
            }
        }
        match scene.on_loop(&mut context, &mut glfw) {
            UpdateResult::Update => {
                chars = CharBuffer::new(text_size.0, text_size.1, scene.as_ref());
                chars.bind();
            },
            UpdateResult::NoChange => {},
            UpdateResult::SwitchScene(s) => {
                scene = s;
                scene.on_attach(&mut context, &mut glfw);
                chars = CharBuffer::new(text_size.0, text_size.1, scene.as_ref());
                chars.bind();
            },
            UpdateResult::Quit => window.set_should_close(true),
        }
    }
}

fn make_window_transform_matrix(
    window_width: i32,
    window_height: i32,
    text_size: (usize, usize),
) -> Mat4 {
    let char_size_w = window_width as f32 / text_size.0 as f32;
    let char_size_h = window_height as f32 / text_size.1 as f32;
    let char_size = char_size_h.min(char_size_w);

    let xoff = (window_width as f32 - char_size * text_size.0 as f32) as f32 / window_width as f32;
    let yoff = (window_height as f32 - char_size * text_size.1 as f32) as f32 / window_height as f32;

    Mat4::from_translation(vec3(xoff - 1.0, 1.0 - yoff, 0.0)) * Mat4::from_scale(vec3(
        2.0 / window_width as f32 * (char_size * text_size.0 as f32),
        -2.0 / window_height as f32 * (char_size * text_size.1 as f32),
        1.0,
    ))
}

fn make_assets(tileset_data: (&str, usize)) -> (Shader, Mesh, Tileset) {
    let shader = unsafe {
        Shader::new(
            "#version 450 core\nin vec2 vertex;uniform mat4 window_matrix;out vec2 uv;\
            void main(){gl_Position=window_matrix*vec4(vertex,0.0,1.0);uv=vertex;}",
            "#version 450 core\nin vec2 uv;uniform vec2 text_size;\
            layout(binding=0)uniform sampler2DArray tileset;\
            layout(binding=1)uniform sampler2D chars;\
            layout(binding=2)uniform sampler2D bg;\
            layout(binding=3)uniform sampler2D fg;\
            out vec4 out_color;\
            void main(){\
                vec2 local_uv=fract(uv*text_size);\
                float tileset_size=textureSize(tileset, 0).z;\
                out_color=vec4(mix(texture(bg,uv).rgb,texture(fg,uv).rgb,texture(tileset,vec3(local_uv,texture(chars,uv).r*tileset_size)).r),1.0);\
            }",
        )
    };
    
    let quad = unsafe { Mesh::make_quad() };
    let tileset_bytes = fs::read(tileset_data.0).expect("Couldn't open tileset file");
    let tileset = unsafe { Tileset::new(&tileset_bytes.as_slice(), (tileset_data.1, tileset_data.1)) };
    (shader, quad, tileset)
}

fn start(
    text_width: usize,
    text_height: usize,
    title: &str,
) -> (Glfw, Window, Receiver<(f64, WindowEvent)>) {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Window hints
    glfw.default_window_hints();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // Antialiasing
    glfw.window_hint(glfw::WindowHint::StencilBits(Some(4)));
    glfw.window_hint(glfw::WindowHint::Samples(Some(4)));


    glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));

    let (width, height) = glfw.with_primary_monitor(|_, monitor| match monitor {
        Some(monitor) => {
            let (_, _, w, h) = monitor.get_workarea();
            let char_size_w = w as usize / text_width;
            let char_size_h = h as usize / text_height;
            let c = char_size_h.min(char_size_w) * 2 / 3;
            if c == 0 {
                (w as usize * 2 / 3, h as usize * 2 / 3)
            } else {
                (text_width * c, text_height * c)
            }
        }
        None => (480, 360),
    });

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(width as u32, height as u32, title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::Adaptive);
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    unsafe {
        gl::Enable(gl::BLEND);
        gl::Enable(gl::MULTISAMPLE);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    (glfw, window, events)
}

fn get_key_from_glfw(key: Key) -> Option<crate::Key> {
    match key {
        Key::A => Some(crate::Key::A),
        Key::B => Some(crate::Key::B),
        Key::C => Some(crate::Key::C),
        Key::D => Some(crate::Key::D),
        Key::E => Some(crate::Key::E),
        Key::F => Some(crate::Key::F),
        Key::G => Some(crate::Key::G),
        Key::H => Some(crate::Key::H),
        Key::I => Some(crate::Key::I),
        Key::J => Some(crate::Key::J),
        Key::K => Some(crate::Key::K),
        Key::L => Some(crate::Key::L),
        Key::M => Some(crate::Key::M),
        Key::N => Some(crate::Key::N),
        Key::O => Some(crate::Key::O),
        Key::P => Some(crate::Key::P),
        Key::Q => Some(crate::Key::Q),
        Key::R => Some(crate::Key::R),
        Key::S => Some(crate::Key::S),
        Key::T => Some(crate::Key::T),
        Key::U => Some(crate::Key::U),
        Key::V => Some(crate::Key::V),
        Key::W => Some(crate::Key::W),
        Key::X => Some(crate::Key::X),
        Key::Y => Some(crate::Key::Y),
        Key::Z => Some(crate::Key::Z),
        Key::Num0 => Some(crate::Key::Num0),
        Key::Num1 => Some(crate::Key::Num1),
        Key::Num2 => Some(crate::Key::Num2),
        Key::Num3 => Some(crate::Key::Num3),
        Key::Num4 => Some(crate::Key::Num4),
        Key::Num5 => Some(crate::Key::Num5),
        Key::Num6 => Some(crate::Key::Num6),
        Key::Num7 => Some(crate::Key::Num7),
        Key::Num8 => Some(crate::Key::Num8),
        Key::Num9 => Some(crate::Key::Num9),
        Key::Up => Some(crate::Key::Up),
        Key::Down => Some(crate::Key::Down),
        Key::Left => Some(crate::Key::Left),
        Key::Right => Some(crate::Key::Right),
        Key::LeftShift => Some(crate::Key::LeftShift),
        Key::RightShift => Some(crate::Key::RightShift),
        Key::Space => Some(crate::Key::Space),
        Key::Enter => Some(crate::Key::Enter),
        Key::Tab => Some(crate::Key::Tab),
        _ => None
    }
}