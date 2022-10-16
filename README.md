# Mandelbrot

Written following the book below. Original code with extensions may be found at
 https://github.com/ProgrammingRust/mandelbrot

## Attribution

*Programming Rust, Second Edition* by Jim Blandy, Jason Orendorff, and Leonora
F. S. Tindall (O’Reilly). Copyright 2021 Jim Blandy, Leonora F. S. Tindall, and
Jason Orendorff, 978-1-492-05259-3.

## Image files:

`mandel1.png` was created using the arguments: `mandel1.png 8000x6000 -1.20,0.35 -1,0.20`.

Ran sequentially in around 15 seconds.
With fork-join concurrency, 12 threads, time decreases to between 3.5 and 4 seconds.
(Ran on 2018 MacBook Pro, 2.6 GHz 6-Core Intel Core i7. Intel's hyperthreading seemed showed some
improvement over runs with 6 threads, which took between 5.5 and 6 seconds.)

Fork-join concurrency in which we naively divide the pixel space into (mostly) uniformly-sized bands
is **suboptimal** because not every pixel takes uniform time to compute. (Pixels that escape radius
2 early, and pixels that stay around until all iterations are complete.)

With variable bounds for the pixels, we cannot a priori optimize how to divide up bands. Using Rayon
for work stealing leads to some improvement.

TODO: Standardized benchmarking...

![mandel1](https://user-images.githubusercontent.com/16626016/195470679-9956e2bd-6019-4184-b42e-d1b9d461ebef.png)
