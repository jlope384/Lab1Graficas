use raylib::prelude::*;

// Algoritmo de Bresenham para dibujar línea entre (x0, y0) y (x1, y1)
fn draw_line_bresenham(image: &mut Image, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = if dx > dy { dx } else { -dy } / 2;
    loop {
        if x0 >= 0 && x0 < image.width as i32 && y0 >= 0 && y0 < image.height as i32 {
            image.draw_pixel(x0 as i32, y0 as i32, color);
        }

        if x0 == x1 && y0 == y1 { break; }
        let e2 = err;
        if e2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if e2 < dy {
            err += dx;
            y0 += sy;
        }
    }
}

// Dibuja el borde del polígono con la línea de Bresenham
fn draw_polygon_edges(image: &mut Image, points: &[(i32, i32)], color: Color) {
    let n = points.len();
    for i in 0..n {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % n];
        draw_line_bresenham(image, x0, y0, x1, y1, color);
    }
}

// Rellena el polígono usando scanline fill simple
fn fill_polygon(image: &mut Image, points: &[(i32, i32)], fill_color: Color, border_color: Color, holes: &[&[(i32, i32)]]) {
    draw_polygon_edges(image, points, border_color);

    let min_y = points.iter().map(|&(_, y)| y).min().unwrap().max(0);
    let max_y = points.iter().map(|&(_, y)| y).max().unwrap().min(image.height as i32 - 1);

    for y in min_y..=max_y {
        let mut intersections = Vec::new();
        let n = points.len();
        for i in 0..n {
            let (x0, y0) = points[i];
            let (x1, y1) = points[(i + 1) % n];

            if y0 != y1 && ((y0 <= y && y1 > y) || (y1 <= y && y0 > y)) {
                let x = x0 + (y - y0) * (x1 - x0) / (y1 - y0);
                intersections.push(x);
            }
        }
        intersections.sort();

        let mut i = 0;
        while i + 1 < intersections.len() {
            let x_start = intersections[i].max(0);
            let x_end = intersections[i + 1].min(image.width as i32 - 1);

            if x_start > x_end {
                i += 2;
                continue;
            }

            let mut skip_fill = false;
            for hole in holes {
                if point_in_polygon((x_start + (x_end - x_start) / 2, y), hole) {
                    skip_fill = true;
                    break;
                }
            }

            if !skip_fill {
                for x in x_start..=x_end {
                    if x >= 0 && x < image.width as i32 && y >= 0 && y < image.height as i32 {
                        image.draw_pixel(x, y, fill_color);
                    }
                }
            }

            i += 2;
        }
    }
}

// Función para saber si un punto está dentro de un polígono (algoritmo ray casting)
fn point_in_polygon(point: (i32, i32), polygon: &[(i32, i32)]) -> bool {
    let (x, y) = point;
    let mut inside = false;
    let n = polygon.len();

    if n < 3 {
        return false;
    }

    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        if yi != yj && ((yi > y) != (yj > y)) &&
            (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }

    inside
}


fn main() {
    let (width, height) = (800, 450);
    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Polígonos y relleno con Bresenham y Scanline")
        .build();

    let mut image = Image::gen_image_color(width, height, Color::WHITE);


    let polygon2: [(i32, i32); 4] = [
        (321, 335), (288, 286), (339, 251), (374, 302)
    ];



    fill_polygon(&mut image, &polygon2, Color::BLUE, Color::WHITE, &[]);

    image.flip_vertical(); // Las imagenes en Raylib están al revés verticalmente

    image.export_image("out.bmp");

    let texture = rl.load_texture_from_image(&thread, &image).unwrap();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
    }
}