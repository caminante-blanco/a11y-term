use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CustomOSC {
    pub identifier: String,

    pub parameters: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct A11yNode {
    pub data: CustomOSC,

    #[serde(skip)]
    pub parent: Option<Rc<A11yNode>>,
}
