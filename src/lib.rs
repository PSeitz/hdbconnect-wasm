mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, hdbconnect-wasm!");
// }





#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;
use hdbconnect::{Connection as HdbCon, HdbResult, IntoConnectParams, ServerCerts};
use hdbconnect::ConnectParams;

// #[wasm_bindgen]
// pub fn statement(stmt: &str) -> String {
//     // alert(&format!("Hello, {}!", name));

//     let params = "hdbsql://HORST:SECRET@hxehost:39013".into_connect_params().unwrap();
//     let mut connection = HdbCon::new(params).unwrap();

//     // let mut insert_stmt = connection.prepare("insert into FOO_SQUARE (f1, f2) values(?,?)")?;
//     // for i in 0..100 {
//     //     insert_stmt.add_batch(&(i, i * i))?;
//     // }
//     // insert_stmt.execute_batch()?;


// // let result: Vec<(usize, usize)> = connection.query(stmt)?.try_into()?;
//     // let res:String = connection.statement(stmt).unwrap().try_into().unwrap();
//     connection.statement(stmt).unwrap();

//     "hello".to_string()
// }



#[wasm_bindgen]
pub struct Connection {
    con: HdbCon
}

#[wasm_bindgen]
impl Connection {
    #[wasm_bindgen(constructor)]
    pub fn new(host: &str, port: usize, user: &str, password: &str, tls: Option<String>) -> Connection {
        let mut builder = ConnectParams::builder();
        builder.hostname(host)
            .port(port as u16)
            .dbuser(user)
            .password(password);
        if let Some(cert) = &tls {
            builder.tls_with(ServerCerts::Direct(cert.to_string()));
        }
        let connect_params  = builder.build().unwrap();

        Connection{con: HdbCon::new(connect_params).unwrap()}
    }

    // pub fn query(&mut self, stmt: &str) -> Result<JsValue, JsValue> {
    //     let n_square: Vec<Vec<String>> =
    //     self.con.query(stmt).unwrap().try_into().unwrap();
    //     serde_wasm_bindgen::to_value(&n_square)
    // }

    pub fn query(&mut self, stmt: &str) -> JsValue {
        let n_square: Vec<Vec<String>> = self.con.query(stmt).unwrap().try_into().unwrap();
        JsValue::from_serde(&n_square).unwrap()
    }
}


