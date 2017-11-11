
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use serde_json::{Value, Error};

pub fn cpuinfo() -> Result<Value, io::Error> {
    let f = File::open("/proc/cpuinfo")?;

    let reader = BufReader::new(f);

    let re = Regex::new(r"^(\w+)\t*: (.*)").unwrap();

    let mut vec = Vec::new();
    let mut map = HashMap::new();

    for line in reader.lines() {
        let l = line?;
        if l == "" {
            vec.push(json!(map));
            map.clear();
            continue;
        }

        re.captures(&l).map( |cap| {
            map.insert(cap[1].to_string(), json!(cap[2]));
        });
    }

    Ok(json!({ "cpuinfo": json!(vec)}))
}
