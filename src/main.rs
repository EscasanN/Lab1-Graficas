mod bmp;
mod framebuffer;
mod line;
mod polygon;

use nalgebra_glm::Vec3;

use crate::framebuffer::Framebuffer;
use crate::polygon::Polygon;
use crate::bmp::WriteBmp;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    let poly1 = vec![
        (165, 380), 
        (185, 360), 
        (180, 330), 
        (207, 345), 
        (233, 330),
        (230, 360), 
        (250, 380), 
        (220, 385), 
        (205, 410), 
        (193, 383)
    ];

    let poly2 = vec![
        Vec3::new(165.0, 380.0, 0.0),
        Vec3::new(185.0, 360.0, 0.0),
        Vec3::new(180.0, 330.0, 0.0),
        Vec3::new(207.0, 345.0, 0.0),
        Vec3::new(233.0, 330.0, 0.0),
        Vec3::new(230.0, 360.0, 0.0),
        Vec3::new(250.0, 380.0, 0.0),
        Vec3::new(220.0, 385.0, 0.0),
        Vec3::new(205.0, 410.0, 0.0),
        Vec3::new(193.0, 383.0, 0.0)
    ];

    let poly3 = vec![
        (321, 335),
        (288, 286),
        (339, 251),
        (374, 302)
    ];

    let poly3_f = vec![
        Vec3::new(321.0, 335.0, 0.0),
        Vec3::new(288.0, 286.0, 0.0),
        Vec3::new(339.0, 251.0, 0.0),
        Vec3::new(374.0, 302.0, 0.0)
    ];
    
    framebuffer.set_current_color(0xEAE52D);

    framebuffer.filled_polygon(&poly2);

    framebuffer.set_current_color(0xFFFFFF);

    framebuffer.polygon(&poly1);

    framebuffer.set_current_color(0x1E11D6);
    framebuffer.filled_polygon(&poly3_f);
    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.polygon(&poly3);

    let _ = framebuffer.render_buffer("output.bmp");

    println!("Framebuffer rendered to output.bmp");
}
