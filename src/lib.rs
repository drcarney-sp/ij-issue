
#[macro_export]
macro_rules! include_proto {
  ($package: tt) => {
    include!(concat!(env!("OUT_DIR"), concat!("/", $package, ".rs")));
  };
}

pub mod f1 {
  crate::include_proto!("f1");
  // if either crate::include_proto! calls are replaced with include! calls, the problem goes away
  // include!(concat!(env!("OUT_DIR"), "/f1.rs"));
  pub mod f2 {
    // if i uncomment this line, i will not be able to click on Struct1 below, and intellij will hang eventually
    // crate::include_proto!("f2");

    // if either crate::include_proto! calls are replaced with include! calls, the problem goes away
    // include!(concat!(env!("OUT_DIR"), "/f2.rs"));
  }
}

pub fn test () {
  let _ = f1::Struct1::default();
}
