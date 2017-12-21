use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use interpreter::value::Value;

macro_rules! runtime_node(
    ($($val:tt)*) => (Rc::new(RefCell::new($($val)*)))
);

pub type RuntimeNode = Rc<RefCell<Runtime>>;
pub struct Runtime {
    parent: Option<RuntimeNode>,
    values: HashMap<String, Value>
}

impl Runtime {
    pub fn new() -> RuntimeNode {
        runtime_node!(Runtime { parent: None, values: HashMap::new() })
    }

    pub fn new_scope(parent: RuntimeNode) -> RuntimeNode {
        runtime_node!(Runtime { parent: Some(parent), values: HashMap::new() })
    }

    pub fn set_var_value(&mut self, key: String, value: Value) {
        self.values.insert(key, value);
    }

    pub fn is_var_defined(&self, key: &String) -> bool {
        self.values.contains_key(key)
    }

    pub fn get_var_value(&self, key: &String) -> Option<Value> {
        if let Some(val) = self.values.get(key) {
            Some(val.clone())
        } else if let Some(ref parent) = self.parent {
            parent.borrow().get_var_value(key)
        } else { None }
    }
}
