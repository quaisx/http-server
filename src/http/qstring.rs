use std::collections::HashMap;

#[derive(Debug)]
pub struct QString<'z> {
    data: HashMap<&'z str, Value<'z>>,
}

#[derive(Debug)]
pub enum Value<'z> {
    Single(&'z str),
    Multiple(Vec<&'z str>),
}

impl<'z> QString<'z> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'z> From<&'z str> for QString<'z> {
    fn from(s: &'z str) -> Self {
        let mut data = HashMap::new();
        // split the query string on &
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            // does this key have an associated value?
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        *existing = Value::Multiple(vec![prev_val, val]);
                    }
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }

        QString { data }
    }
}
