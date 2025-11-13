use std::collections::HashMap;
use std::rc::Rc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CustomOSC {
    pub identifier: String,

    pub parameters: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct A11yNode {
    pub data: CustomOsc,

    pub 
}
