mod framebuffer;
mod line;
mod polygon;
mod bmp;

extern crate nalgebra_glm as glm;

use glm::Vec3;
use crate::framebuffer::Framebuffer;
use crate::line::Line;
use crate::polygon::Polygon;
use crate::bmp::WriteBmp;


fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    dibujar_polygon1(&mut framebuffer);
    dibujar_polygon2(&mut framebuffer);
    dibujar_polygon3(&mut framebuffer);
    dibujar_polygon4(&mut framebuffer);
    dibujar_polygon5(&mut framebuffer);

    let _ = framebuffer.render_buffer("out.bmp");
}

fn dibujar_polygon1(framebuffer: &mut Framebuffer) {
    framebuffer.set_current_color(0x00FFFF); 
    let poligono1 = vec![
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
    framebuffer.fill_polygon(&poligono1);

    framebuffer.set_current_color(0xFFFFFF); 
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
    
    framebuffer.polygon(&poly1); 

    
}

fn dibujar_polygon2(framebuffer: &mut Framebuffer) {
    framebuffer.set_current_color(0xFF0000); 
    let poligono2 = vec![
        Vec3::new(321.0, 335.0, 0.0),
        Vec3::new(288.0, 286.0, 0.0), 
        Vec3::new(339.0, 251.0, 0.0),
        Vec3::new(374.0, 302.0, 0.0)
    ];
    framebuffer.fill_polygon(&poligono2);

    framebuffer.set_current_color(0xFFFFFF); 
    let poly2 = vec![
        (321, 335), (288, 286), (339, 251), (374, 302)
    ];

    framebuffer.polygon(&poly2);   
}

fn dibujar_polygon3(framebuffer: &mut Framebuffer) {
    framebuffer.set_current_color(0x0000FF); 
    let poligono3 = vec![
        Vec3::new(377.0, 249.0, 0.0),
        Vec3::new(411.0, 197.0, 0.0), 
        Vec3::new(436.0, 249.0, 0.0)
    ];
    framebuffer.fill_polygon(&poligono3);

    framebuffer.set_current_color(0xFFFFFF); 
    let poly3 = vec![
        (377, 249), (411, 197), (436, 249)
    ];

    framebuffer.polygon(&poly3); 

    
}

fn dibujar_polygon4(framebuffer: &mut Framebuffer) {
    framebuffer.set_current_color(0x00FF00); 
    let poligono4 = vec![
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
    framebuffer.fill_polygon(&poligono4);

    framebuffer.set_current_color(0xFFFFFF); 
    let poly4 = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180)
    ];

    framebuffer.polygon(&poly4); 
}

fn dibujar_polygon5(framebuffer: &mut Framebuffer) {
    framebuffer.set_current_color(0x000000); 
    let poligono5 = vec![
        Vec3::new(682.0, 175.0, 0.0),
        Vec3::new(708.0, 120.0, 0.0), 
        Vec3::new(735.0, 148.0, 0.0),
        Vec3::new(739.0, 170.0, 0.0)
    ];
    framebuffer.fill_polygon(&poligono5);

    framebuffer.set_current_color(0xFFFFFF); 
    let poly5 = vec![
        (682, 175), (708, 120), (735, 148), (739, 170)
    ];

    framebuffer.polygon(&poly5); 

    
}





