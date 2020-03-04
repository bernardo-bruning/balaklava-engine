use crate::geometry::Mesh;
use std::path::Path;
use std::io::Result;
use std::fs::File;


pub struct Obj {}

impl Obj {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        unimplemented!();
    }
}

impl From<File> for Obj {
    fn from(file: File) -> Self{
        unimplemented!();
    }
}

impl <'a> Into<Mesh<'a>> for Obj {
    fn into(self) -> Mesh<'a> {
        unimplemented!();
    }
}