use clap::Parser;

/// Mandelbrot fractal generator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(allow_negative_numbers = true)]
#[clap(allow_hyphen_values = true)]
pub struct MandelbrotArgs {
    /// Number of iterations
    #[clap(short, long, value_parser, default_value = "500")]
    pub n: usize,

    /// Output image
    #[clap(short, long, value_parser, default_value = "output.png")]
    pub output: String,

    /// Dimensions of generated image
    #[clap(short, long, value_parser, default_value = "1000x1000")]
    pub dimensions: String,

    /// Domain (x axis) min and max value
    #[clap(short, long, value_parser, default_value = "-2.0,0.47")]
    pub x: String,

    /// Range (y axis) min and max value
    #[clap(short, long, value_parser, default_value = "-1.235,1.235")]
    pub y: String,
}
