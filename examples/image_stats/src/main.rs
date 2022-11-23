use geotiff::{TIFF, GeoKeyDirectory};

use std::path::PathBuf;



fn main() {
    let mut pathbuf = PathBuf::from("../../resources");

    pathbuf.push("bogota.tif");
    let tiff = TIFF::open(pathbuf.to_str().unwrap()).unwrap();
    // println!("{}", &tiff.ifds.len());
    // println!("{:#?}", &tiff.ifds[0]);
    let keys = GeoKeyDirectory::parse(&tiff.ifds[0]);

    println!("{:#?}", keys);
}
