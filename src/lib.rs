use wasm_bindgen::prelude::*;
use web_sys::Worker;
use web_sys::console;
use wasm_bindgen::JsCast;
use serde_json::Value;

mod algo;
mod util;



// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct PathFinder {
    worker: Worker,
}

#[wasm_bindgen]
impl PathFinder {
    pub fn new(worker: JsValue) -> PathFinder {
        let worker = worker.unchecked_into::<Worker>();
        PathFinder{worker}
    }

    fn post_message(&self, s : &str) {
        match self.worker.post_message(&JsValue::from(s)) {
            Ok(()) => (),
            Err(e) => console::error_1(&e)
        }
    }

    pub fn onmessage(&self, data : &str) {
        let v: Value = match serde_json::from_str(data) {
            Ok(v) => v,
            Err(e) => {
                console::error_1(&JsValue::from(format!("{:?}",e)));
                Value::Null
            }
        };

        let function_name = if let Value::String(s)= &v["f"] {
            s.as_ref()
        } else {
            ""
        };

        let n = PathFinder::get_n(&v);

        match function_name {
            "compute_path" => self.compute_path(v),
            _ => console::error_1(&JsValue::from(format!("function {} not supported",function_name)))
        }

        self.end(n)
    }

    fn get_n(v: &Value) -> i64 {
        if let Value::Number(n) = &v["n"] {
            if let Some(n) = n.as_i64() {
                n
            } else {
                -1
            }
        } else {
            -1
        }
    }

    fn end(&self,n: i64) {
        self.post_message(format!("{{\"n\":{},\"end\"=true}}",n).as_ref());
    }

    fn compute_path(&self, v: Value) {
        let n = PathFinder::get_n(&v);
        self.post_message(format!("{{\"n\":{},\"data\":\"{}\"}}",n,"Hello World").as_ref());
    }

    fn load(&self, data: &str) {

    }
}
