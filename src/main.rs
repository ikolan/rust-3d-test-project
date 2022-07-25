mod colors;
mod mesh;
mod triangle;
mod windowing;

use crate::{
    mesh::Mesh,
    triangle::ScreenTriangle,
    triangle::Triangle,
    windowing::{Frame, Window},
};
use nalgebra::{Isometry3, Perspective3, Point3, Rotation3, Translation3, UnitQuaternion, Vector3};
use std::ffi::CString;

pub const SCREEN_SIZE: (i32, i32) = (1000, 640);
pub const ASPECT_RATIO: f64 = SCREEN_SIZE.0 as f64 / SCREEN_SIZE.1 as f64;
pub const FOV: f64 = 60.;
pub const Z_NEAR: f64 = 0.0001;
pub const Z_FAR: f64 = 10000.;
pub const MOVE_SPEED: f64 = 3.;

fn draw_triangle(frame: &Frame, points: &ScreenTriangle, color: raylib_sys::Color) {
    frame.draw_line(
        points.vertices[0].x as i32,
        points.vertices[0].y as i32,
        points.vertices[1].x as i32,
        points.vertices[1].y as i32,
        color,
    );
    frame.draw_line(
        points.vertices[1].x as i32,
        points.vertices[1].y as i32,
        points.vertices[2].x as i32,
        points.vertices[2].y as i32,
        color,
    );
    frame.draw_line(
        points.vertices[2].x as i32,
        points.vertices[2].y as i32,
        points.vertices[0].x as i32,
        points.vertices[0].y as i32,
        color,
    );
}

fn process_input(camera: &mut Isometry3<f64>, show_normal: &mut bool) {
    use raylib_sys::{GetFrameTime, IsKeyDown, IsKeyPressed, KeyboardKey};

    let mut movement: Vector3<f64> = Vector3::new(0., 0., 0.);
    let mut cam_rot_vel = (0., 0.);
    let speed = MOVE_SPEED * unsafe { GetFrameTime() as f64 };

    unsafe {
        if IsKeyDown(KeyboardKey::KEY_W as i32) {
            movement.z = -speed;
        }
        if IsKeyDown(KeyboardKey::KEY_S as i32) {
            movement.z = speed;
        }
        if IsKeyDown(KeyboardKey::KEY_D as i32) {
            movement.x = speed;
        }
        if IsKeyDown(KeyboardKey::KEY_A as i32) {
            movement.x = -speed;
        }
        if IsKeyDown(KeyboardKey::KEY_R as i32) {
            movement.y = speed;
        }
        if IsKeyDown(KeyboardKey::KEY_F as i32) {
            movement.y = -speed;
        }

        if IsKeyDown(KeyboardKey::KEY_Q as i32) {
            cam_rot_vel.0 = -speed;
        }
        if IsKeyDown(KeyboardKey::KEY_E as i32) {
            cam_rot_vel.0 = speed;
        }

        if IsKeyDown(KeyboardKey::KEY_G as i32) {
            cam_rot_vel.1 = -speed;
        }
        if IsKeyDown(KeyboardKey::KEY_T as i32) {
            cam_rot_vel.1 = speed;
        }

        if IsKeyPressed(KeyboardKey::KEY_N as i32) {
            *show_normal = !*show_normal;
        }
    }

    camera.append_translation_mut(&Translation3::new(movement.x, movement.y, movement.z));
    camera.append_rotation_wrt_center_mut(&UnitQuaternion::from_rotation_matrix(
        &Rotation3::from_euler_angles(cam_rot_vel.1, cam_rot_vel.0, 0.),
    ));
}

fn main() {
    let window = Window::new(SCREEN_SIZE.0, SCREEN_SIZE.1);
    let perspective = Perspective3::new(ASPECT_RATIO, FOV.to_radians(), Z_NEAR, Z_FAR);
    let mesh = Mesh::from_file();
    let mesh_triangles_count = mesh.triangles_count();
    let mut camera = Isometry3::new(Vector3::new(0., 0., 5.), Vector3::new(0., 0., 0.));
    let mut show_normal = false;

    while !window.should_close() {
        let frame = window.begin_drawing();
        let mut triangle_render_count = 0u64;

        process_input(&mut camera, &mut show_normal);

        let mut triangles_to_draw = Vec::new();

        for (index, triangle) in mesh.triangles_resolved().iter().enumerate() {
            let translated_triangle = triangle.apply_camera_transform(&camera);
            let screen_triangle = translated_triangle.project_on_screen_space(&perspective);
            let normal = mesh.normals[index];

            if translated_triangle.drawable(&camera, Some(normal)) && screen_triangle.on_screen() {
                triangles_to_draw.push(translated_triangle);
            }
        }

        for triangle in triangles_to_draw.iter() {
            let normal = triangle.normal();
            let screen_triangle = triangle.project_on_screen_space(&perspective);
            draw_triangle(&frame, &screen_triangle, colors::WHITE);
            triangle_render_count += 1;
            if show_normal {
                draw_triangle(
                    &frame,
                    &Triangle::new(
                        triangle.vertices[0],
                        Point3::new(
                            triangle.vertices[0].x + (normal.x / 5.),
                            triangle.vertices[0].y + (normal.y / 5.),
                            triangle.vertices[0].z + (normal.z / 5.),
                        ),
                        triangle.vertices[0],
                    )
                    .project_on_screen_space(&perspective),
                    colors::RED,
                );
            }
        }

        //
        // DEBUG INFORMATIONS
        //
        unsafe {
            raylib_sys::DrawText(
                CString::new(format!(
                    "Triangles rendered : {} / {}",
                    triangle_render_count, mesh_triangles_count
                ))
                .unwrap()
                .into_raw(),
                0,
                0,
                9,
                colors::GRAY,
            );
            raylib_sys::DrawText(
                CString::new(format!(
                    "Position : {} x {} x {}",
                    camera.translation.x, camera.translation.y, camera.translation.z
                ))
                .unwrap()
                .into_raw(),
                0,
                10,
                9,
                colors::GRAY,
            );
            raylib_sys::DrawText(
                CString::new(format!(
                    "Rotation : {} x {} x {}",
                    camera.rotation.euler_angles().0.to_degrees(),
                    camera.rotation.euler_angles().1.to_degrees(),
                    camera.rotation.euler_angles().2.to_degrees()
                ))
                .unwrap()
                .into_raw(),
                0,
                20,
                9,
                colors::GRAY,
            );
        }
    }
}
