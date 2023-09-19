use std::collections::HashMap;

// since we are dealing with string literals mapping into data buffers
// we heavily use the lifespan indicators
#[derive(Debug)]
pub struct QString<'z> {
    // query value may be Single or Multiple
    data: HashMap<&'z str, Value<'z>>,
}

#[derive(Debug)]
pub enum Value<'z> {
    Single(&'z str), // single value string literal
    Multiple(Vec<&'z str>), // multiple value a vec of string literals
}

impl<'z> QString<'z> {
    // get value based on the key passed in
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

//a=1&b=2&cz&&d=1&e=5&f=6&2&d=3 - we need to account for query value arrays in query strings
// for example the key d is encountered 3 times with values 1,2,3 correspondingly
impl<'z> From<&'z str> for QString<'z> {
    fn from(s: &'z str) -> Self {
        // each key may have multiple values - use hash map to store multiple values
        let mut data = HashMap::new();
        // split the query string on &
        for sub_str in s.split('&') {
            let mut q_key = sub_str;
            // in case a key is missing a value, store an empty string
            let mut q_val = "";
            // does this key have an associated value?
            if let Some(assign_idx) = sub_str.find('=') { // does it have an actual value via assignment operator?
                q_key = &sub_str[..assign_idx]; // the data to the left of the = operator is the key
                q_val = &sub_str[assign_idx + 1..]; // the data to the right of the = operator is the value
            }
            // get existing value(s) for the given key
            data.entry(q_key)
                // provide in-place mutable access to an occupied entry before any potential inserts into the map.
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        // if the value was signle, replace with multiple...
                        *existing = Value::Multiple(vec![prev_val, q_val]);
                    }
                    Value::Multiple(vec) => vec.push(q_val),
                })
                // or simply insert a single value
                .or_insert(Value::Single(q_val));
        }

        Self {
            data
        }
    }
}
