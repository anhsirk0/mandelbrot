use image::ImageBuffer;
use std::f64::consts::PI;

pub struct Mandelbrot {
    iterations: usize,
    width: u32,
    height: u32,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    output: String,
}

impl Mandelbrot {
    pub fn new(
        iterations: usize,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        width: u32,
        height: u32,
        output: String,
    ) -> Self {
        let info = format!(
            "Generating Mandelbrot fractal with following parameters.\n\
             domain:           {}, {}\n\
             range:            {}, {}\n\
             n (iterations):   {}\n\n\
             Output image properties.\n\
             name:             {}\n\
             dimensions:       {}x{}\n",
            x_min, x_max, y_min, y_max, iterations, output, width, height
        );
        println!("{}", info,);

        Mandelbrot {
            iterations,
            x_min,
            x_max,
            y_min,
            y_max,
            width,
            height,
            output,
        }
    }

    fn stays_in_region(&self, a: f64, b: f64) -> (bool, usize) {
        let mut a0: f64 = 0.0;
        let mut b0: f64 = 0.0;
        for i in 0..self.iterations {
            if a0.powf(2.0) + b0.powf(2.0) >= 4.0 {
                return (false, i);
            }

            let a1: f64 = a0.powf(2.0) - b0.powf(2.0) + a;
            let b1: f64 = 2.0 * a0 * b0 + b;

            a0 = a1;
            b0 = b1;
        }

        (true, self.iterations)
    }

    pub fn generate_image(&self) {
        let mut imgbuf = ImageBuffer::new(self.width, self.height);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let x_length: f64 = self.x_max - self.x_min;
            let y_length: f64 = self.y_max - self.y_min;
            let a = (x_length / self.width as f64) * x as f64 - self.x_min.abs();
            let b =
                (y_length / self.height as f64) * (self.height - y - 1) as f64 - self.y_min.abs();

            let (stays_in_region, i) = self.stays_in_region(a, b);
            if stays_in_region {
                // Need to implement better coloring algorithm
                let s: f64 = i as f64 / self.iterations as f64;
                let v: f64 = 1.0 - (PI * s).powf(2.0);
                let r: u8 = 75 - (75 as f64 * v) as u8;
                let g: u8 = r + 28;
                let b: u8 = ((s * 360.0).powf(1.5) % 360.0) as u8;
                *pixel = image::Rgb([r, g, b]);
            }
        }

        match imgbuf.save(&self.output) {
            Ok(_t) => {
                println!("Fractal generated successfully.")
            }
            Err(e) => {
                println!("Error occured {}", e)
            }
        }
    }
}
