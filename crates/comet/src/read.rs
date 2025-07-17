use ciborium_ll::{Decoder, Header};
use rustfft::num_complex::Complex64;

pub fn read_float_array<R: std::io::Read>(
    decoder: &mut Decoder<R>,
    len: usize,
) -> Result<Vec<f64>, String> {
    let mut values = Vec::<f64>::with_capacity(len);
    for _ in 0..len {
        match decoder.pull().unwrap() {
            Header::Float(x) => values.push(x),
            header => return Err(format!("array element {:?} is not a float", header)),
        }
    }
    Ok(values)
}

pub fn read_float_array_2d<R: std::io::Read>(
    decoder: &mut Decoder<R>,
    len: usize,
) -> Result<Vec<Vec<f64>>, String> {
    let mut values = Vec::with_capacity(len);

    for _ in 0..len {
        match decoder.pull().unwrap() {
            Header::Array(Some(inner_len)) => {
                values.push(read_float_array(decoder, inner_len)?);
            }
            header => return Err(format!("array element {:?} is not an array", header)),
        }
    }

    Ok(values)
}

pub fn read_complex_array<R: std::io::Read>(
    decoder: &mut Decoder<R>,
    len: usize,
) -> Result<Vec<Complex64>, String> {
    let mut values = vec![Complex64 { re: 0.0, im: 0.0 }; len];

    for value in values.iter_mut() {
        *value = match decoder.pull().unwrap() {
            Header::Array(Some(2)) => {
                let re = match decoder.pull().unwrap() {
                    Header::Float(x) => x,
                    _ => return Err(String::from("Bad input")),
                };

                let im = match decoder.pull().unwrap() {
                    Header::Float(x) => x,
                    _ => return Err(String::from("Bad input")),
                };
                Complex64 { re, im }
            }
            _ => return Err(String::from("Bad input")),
        };
    }
    Ok(values)
}
