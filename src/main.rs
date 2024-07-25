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





