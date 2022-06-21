use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    // index_out_of_bounds();
    matching_different_errors();
}

/// Unrecoverable index out of bounds error
fn index_out_of_bounds() {
    let v = vec![1, 2, 3];

    v[99];
}

fn matching_different_errors() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // Unwrap or else performing the same logic as above
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn propagating_errors() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s);

    // Above can be simplified to
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s);

    // Or can be replaced with
    fs::read_to_string("hello.txt")
}
