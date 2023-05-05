use std::{net::*, io::{ErrorKind, Read}, fs::File};
use ::math::matrix::{Vec3, Vec4};
use visual_debugger::netdata;

use geometric::geom3d;
use graphics::mesh_generate::*;
use raylib::prelude::*;

fn draw_mesh(
    mode: &mut RaylibMode3D<RaylibDrawHandle>,
    position: ::math::matrix::Vec3,
    display_data: &FaceDisplayData,
) {
    for indices in display_data.indices.chunks(3) {
        for i in 0..indices.len() {
            let p1 = display_data.vertices[indices[i] as usize] + position;
            let p2 = display_data.vertices[indices[(i + 1) % 3] as usize] + position;
            mode.draw_line_3D(
                Vector3::new(p1.x(), p1.y(), p1.z()),
                Vector3::new(p2.x(), p2.y(), p2.z()),
                Color::new(
                    (display_data.color.x() * 255.0).clamp(0.0, 255.0) as u8,
                    (display_data.color.y() * 255.0).clamp(0.0, 255.0) as u8,
                    (display_data.color.z() * 255.0).clamp(0.0, 255.0) as u8,
                    (display_data.color.w() * 255.0).clamp(0.0, 255.0) as u8,
                ),
            );
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1024, 720)
        .title("display")
        .msaa_4x()
        .build();

    let mut camera = Camera3D::perspective(
        Vector3::new(4.0, 2.0, 4.0),
        Vector3::new(0.0, 1.8, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        60.0,
    );
    rl.set_camera_mode(&camera, CameraMode::CAMERA_FIRST_PERSON);
    rl.set_target_fps(60);
    rl.show_cursor();

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.set_nonblocking(true).unwrap();

    let mut client: Option<TcpStream> = None;

    let mut file = File::open("./tools/visual-debugger/resources/cylinder.toml").unwrap();
    let content = std::io::read_to_string(&mut file).unwrap();
    let cylinder: netdata::Cylinder = toml::from_str(&content).unwrap();

    while !rl.window_should_close() {
        // net work
        // accept client
        if client.is_none() {
            match listener.accept() {
                Ok(cli) => {
                    client = Some(cli.0);
                    println!("connected!");
                }
                Err(e) => {
                    if e.kind() != ErrorKind::WouldBlock {
                        eprintln!("net error: {}", e.kind());
                    }
                }
            }
        } else {
            let client = client.as_mut().unwrap();
            // read data
            let mut buf = [0u8; 1024];
            match client.read(&mut buf) {
                Ok(num) => {
                    if num > 0 {
                        for i in 0..num {
                            print!("{}", buf[i]);
                        }
                        println!("");
                    }
                },
                Err(e) => {
                    if e.kind() != ErrorKind::WouldBlock {
                        client.shutdown(Shutdown::Both).unwrap();
                        println!("shutdown client, Error: {:?}", e);
                    }
                }
            }
        }

        // render
        let mut d = rl.begin_drawing(&thread);
        d.update_camera(&mut camera);
        let mut m = d.begin_mode3D(camera);
        m.clear_background(Color::DARKGRAY);
        m.draw_grid(100, 1.0);

        let cy: geom3d::Cylinder = cylinder.into();
        let data = cylinder_to_display_data(&cy, Vec4::from_xyzw(0.0, 1.0, 0.0, 1.0), 100);
        draw_mesh(&mut m, Vec3::zeros(), &data);
    }
}