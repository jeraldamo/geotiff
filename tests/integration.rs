extern crate rust_geotiff as tiff;

use std::env;
use tiff::reader::TIFFReader;

#[test]
fn test_load() {
    let tiff_reader = TIFFReader;
    match tiff_reader.load("resources/marbles.tif") {
        Ok(x) => println!("Read tiff {:?}", x),
        Err(e) => println!("File I/O Error: {:?}", e),
    }
}