use num::Complex;
use std::str::FromStr;

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is the character given by
/// the `separator` argument, and <left> and <right> are both strings that can be parsed by
/// `T::from_str`. `separator` must be an ASCII character. If `s` has the proper form, return
/// `Some<(x, y)>`. If it doesn't parse /// correctly, return `None`.
#[allow(unused)]
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

/// Parse a pair of floating-point number separated by a comma as a complex number.
#[allow(unused)]
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    parse_pair(s, ',').map(|(re, im)| Complex { re, im })
}

/// Given the row and column of a pixel in the output image, return the corresponding point on the
/// complex plane. `bounds` is a pair giving the width and height of the image in pixels. `pixel` is
/// a (column, row) pair indicating a particular pixel in that image. The `upper_left` and
/// `lower_right` parameters are points on the complex plane designating the area our image covers.
#[allow(unused)]
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (percentage_across_width, percentage_down_height) = (
        pixel.0 as f64 / bounds.0 as f64,
        pixel.1 as f64 / bounds.1 as f64,
    );
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + width * percentage_across_width,
        im: upper_left.im - height * percentage_down_height,
    }
}

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit` iterations to decide.
/// If `c` is not a member, return `Some(i)`, where `i` is number of iterations to verify.
#[allow(unused)]
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        // If `z` ever leaves circle r=2 centered on origin, then `c` diverges.
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    // `c` seems to be a member of Mandelbrot set, given that we reached iteration limit without
    // proving that `c` diverges.
    None
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}
