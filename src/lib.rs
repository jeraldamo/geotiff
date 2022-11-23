extern crate byteorder;
#[macro_use]
extern crate enum_primitive;
extern crate num;

use num::FromPrimitive;

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};
use std::io::{Read, Seek};
use std::collections::{HashMap, HashSet};
use std::io::Result;
use std::fmt;

mod lowlevel;
mod reader;
mod keys;
pub mod tiff;

use tiff::*;
use reader::*;
use lowlevel::{TIFFTag, TagValue};
pub use tiff::TIFF;
pub use keys::{GeoKeyDirectory, GeoKeyEntry};

/// The GeoTIFF library reads `.tiff` files.
///
/// It is primarily used within a routing application that needs to parse digital elevation models.
/// As such, other use cases are NOT tested (for now).
impl TIFF {
    /// Opens a `.tiff` file at the location indicated by `filename`.
    pub fn open(filename: &str) -> Result<Box<TIFF>> {
        let tiff_reader = TIFFReader;
        tiff_reader.load(filename)
    }

    /// Gets the value at a given coordinate (in pixels).
    pub fn get_value_at(&self, lon: usize, lat: usize) -> usize {
        self.image_data[lon][lat][0]
    }
}

/// Overwrite default display function.
impl fmt::Display for TIFF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TIFF(Image size: [{}, {}, {}], Tag data: {:?})",
               self.image_data.len(), self.image_data[0].len(),
               self.image_data[0][0].len(), self.ifds)
    }
}


impl GeoKeyDirectory {
    pub fn parse(ifd: &IFD) -> Option<Self> {
        for ifd_entry in &ifd.entries {
            if ifd_entry.tag == TIFFTag::GeoKeyDirectoryTag {
                let values = ifd_entry.value.iter()
                    .map(|value| {
                        match value {
                            TagValue::ShortValue(n) => *n,
                            _ => {panic!("GeoKeyDirectory entry value not a ShortValue")}
                        }
                    })
                    .collect::<Vec<u16>>();

                let mut keydirectory = GeoKeyDirectory::new(values[0], values[1], values[2], values[3]);
                for i in (4..ifd_entry.value.len()).step_by(4) {
                    keydirectory.add_key(GeoKeyEntry::new(values[i+0], values[i+1], values[i+2], values[i+3]));
                }
                return Some(keydirectory);
            }
        }

        None
    }
}
