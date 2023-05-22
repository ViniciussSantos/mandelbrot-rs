use num::Complex;

fn calculate_mandelbrot(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut rows: Vec<_> = Vec::with_capacity(width);
    for img_y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            let x_percent = img_x as f64 / width as f64;
            let y_percent = img_y as f64 / height as f64;
            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }

        rows.push(row);
    }
    rows
}

fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);

    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            return i;
        }
        z = z * z + c;
    }
    max_iters
}

fn render_mandelbrot_image(width: usize, height: usize, escape_vals: Vec<Vec<usize>>) {
    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.4 * x as f32) as u8;
        let b = (0.4 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    for x in 0..width {
        for y in 0..height {
            let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], escape_vals[x][y] as u8, data[2]]);
        }
    }
    imgbuf.save("fractal.png").unwrap();
}

fn main() {
    let width = 1024;
    let height = 1024;
    let mandelbrot = calculate_mandelbrot(3000, -2.0, 1.0, -1.0, 1.0, width, height);

    render_mandelbrot_image(width, height, mandelbrot);
}
