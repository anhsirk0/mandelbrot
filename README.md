# Mandelbrot fractal

[What is a Mandelbrot fractal?](https://en.wikipedia.org/wiki/Mandelbrot_set)

```bash
$ mandelbrot
Generating Mandelbrot fractal with following parameters.
domain:           -2, 0.47
range:            -1.235, 1.235
n (iterations):   500

Output image properties.
name:             output.png
dimensions:       1000x1000

Fractal generated successfully.
```
output.png
![output.png](https://github.com/anhsirk0/mandelbrot/blob/master/output.png)


## Usage
```text
Mandelbrot fractal generator

USAGE:
    mandelbrot [OPTIONS]

OPTIONS:
    -d, --dimensions <DIMENSIONS>    Dimensions of generated image [default: 1000x1000]
    -h, --help                       Print help information
    -n, --n <N>                      Number of iterations [default: 500]
    -o, --output <OUTPUT>            Output image [default: output.png]
    -V, --version                    Print version information
    -x, --x <X>                      Domain (x axis) min and max value [default: -2.0,0.47]
    -y, --y <Y>                      Range (y axis) min and max value [default: -1.235,1.235]
```
