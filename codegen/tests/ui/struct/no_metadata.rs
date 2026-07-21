use feignhttp_rs_codegen::feign;

// error: no metadata assign
//    |  #[feign]
//    |  ^^^^^^^^

struct Http;

#[feign]
impl Http {}

fn main() {}
