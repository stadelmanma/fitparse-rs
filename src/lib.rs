extern crate nom;
pub mod objects;
pub use objects::FitFile;
pub mod parser;
pub mod profile;
pub use parser::parse_file;
