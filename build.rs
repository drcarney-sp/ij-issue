use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let out_dir = env::var_os("OUT_DIR").unwrap();
  let out_dir = PathBuf::from(out_dir);
  let f1 = out_dir.join("f1.rs");
  let mut f1 = File::create(f1)?;
  f1.write_all(F1_DATA.as_bytes())?;
  let f2 = out_dir.join("f2.rs");
  let mut f2 = File::create(f2)?;
  f2.write_all(F2_DATA.as_bytes())?;
  Ok(())
}

const F1_DATA : &str = r#"
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Struct1 {
    #[prost(uint32, optional, tag="1")]
    pub val_1: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Struct2 {
   #[prost(message, optional, tag="1")]
   pub val_1: ::core::option::Option<::prost_types::Timestamp>,
   #[prost(message, optional, tag="2")]
   pub val_2: ::core::option::Option<::prost_types::Timestamp>,
}
"#;

const F2_DATA : &str = r#"
pub struct Whatever {
}
"#;

