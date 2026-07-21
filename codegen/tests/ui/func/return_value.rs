use feignhttp_rs_codegen::get;

// error: function must have a return value
//    |  pub async fn send_get() {}
//    |      ^^^^^^^^^^^^^^^^^^^

#[get("http://xxx")]
pub async fn send_get() {}

fn main() {}
