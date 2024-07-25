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
    
    let poly4 = vec![
        (377, 249),
        (411, 197),
        (436, 249)
    ];
    
    let poly4_f = vec![
        Vec3::new(377.0, 249.0, 0.0),
        Vec3::new(411.0, 197.0, 0.0),
        Vec3::new(436.0, 249.0, 0.0)
    ];

    let poly5 = vec![
        (413, 177),
        (448, 159),
        (502, 88),
        (553, 53),
        (535, 36),
        (676, 37),
        (660, 52),
        (750, 145),
        (761, 179),
        (672, 192),
        (659, 214),
        (615, 214),
        (632, 230),
        (580, 230),
        (597, 215),
        (552, 214),
        (517, 144),
        (466, 180)
    ];

    let poly5_f = vec![
        Vec3::new(413.0, 177.0, 0.0),
        Vec3::new(448.0, 159.0, 0.0),
        Vec3::new(502.0, 88.0, 0.0),
        Vec3::new(553.0, 53.0, 0.0),
        Vec3::new(535.0, 36.0, 0.0),
        Vec3::new(676.0, 37.0, 0.0),
        Vec3::new(660.0, 52.0, 0.0),
        Vec3::new(750.0, 145.0, 0.0),
        Vec3::new(761.0, 179.0, 0.0),
        Vec3::new(672.0, 192.0, 0.0),
        Vec3::new(659.0, 214.0, 0.0),
        Vec3::new(615.0, 214.0, 0.0),
        Vec3::new(632.0, 230.0, 0.0),
        Vec3::new(580.0, 230.0, 0.0),
        Vec3::new(597.0, 215.0, 0.0),
        Vec3::new(552.0, 214.0, 0.0),
        Vec3::new(517.0, 144.0, 0.0),
        Vec3::new(466.0, 180.0, 0.0)
    ];

    let poly6 = vec![
        (682, 175),
        (708, 120),
        (735, 148),
        (739, 170)
    ];

    let poly6_f = vec![
        Vec3::new(682.0, 175.0, 0.0),
        Vec3::new(708.0, 120.0, 0.0),
        Vec3::new(735.0, 148.0, 0.0),
        Vec3::new(739.0, 170.0, 0.0)
    ];
        
    framebuffer.set_current_color(0xEAE52D);

    framebuffer.filled_polygon(&poly2);

    framebuffer.set_current_color(0xFFFFFF);

    framebuffer.polygon(&poly1);

    framebuffer.set_current_color(0x1E11D6);
    framebuffer.filled_polygon(&poly3_f);
    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.polygon(&poly3);

    framebuffer.set_current_color(0xF20F0F);
    framebuffer.filled_polygon(&poly4_f);
    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.polygon(&poly4);

    framebuffer.set_current_color(0x04941F);
    framebuffer.filled_polygon(&poly5_f);
    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.polygon(&poly5);
    framebuffer.set_current_color(0x000000);
    framebuffer.filled_polygon(&poly6_f);
    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.polygon(&poly6);

    let _ = framebuffer.render_buffer("output.bmp");

    println!("Framebuffer rendered to output.bmp");
}
