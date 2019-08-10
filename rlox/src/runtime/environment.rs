use std::collections::HashMap;
use runtime::*;

pub struct Environment {
    values: HashMap<String, LoxObject>
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new()
        }
    }

    pub fn get(&mut self, name: String) -> Option<&LoxObject> {
        self.values.get(&name)
    }

    pub fn define(&mut self, name: String, value: LoxObject) {
        self.values.insert(name, value);
    }
}
