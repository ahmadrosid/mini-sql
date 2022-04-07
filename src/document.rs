use std::str::FromStr;

pub enum DataType {
    STRING,
    INT,
}

pub struct Row {
    pub name: String,
    pub data_type: DataType
}

impl Row {
    pub fn new(key: &str, data_type: DataType) -> Self {
        Self {
            name: key.to_string(),
            data_type
        }
    }
}

pub enum DataValue {
    STRING(String),
    INT(u32),
}

impl DataValue {
    pub fn to_string(source: String) -> Self {
        Self::STRING(source)
    }

    pub fn to_int(source: String) -> Self {
        let val: u32 = FromStr::from_str(&source).unwrap();
        Self::INT(val)
    }
}

pub struct Table {
    pub name: String,
    pub schema: Vec<Row>,
    pub pages: Vec<Vec<(String, DataValue)>>,
}

impl Table {
    pub fn new() -> Self {
        let name = String::from("test_table");
        let mut schema = Vec::new();
        schema.push(Row::new("id", DataType::INT));
        schema.push(Row::new("email", DataType::STRING));

        Self {
            name,
            schema,
            pages: Vec::new(),
        }
    }

    pub fn select(&self) {
        for page in &self.pages {
            let mut lines: Vec<String> = Vec::new();
            for (_key, val) in page {
                match val {
                    DataValue::INT(value) => {
                        lines.push(format!("{}", value));
                    }
                    DataValue::STRING(value) => {
                        lines.push(format!("{}", value));
                    }
                }
            }
            println!("( {} )", lines.join(", "));
        }
    }

    pub fn insert(&mut self, values: Vec<&str>) {
        let mut data = Vec::new();
        let mut i = 0;
        for row in &self.schema {
            let value = values[i].to_string();
            let key = row.name.to_string();
            match row.data_type {
                DataType::INT => {
                    data.push((key, DataValue::to_int(value)));
                }
                DataType::STRING => {
                    data.push((key, DataValue::to_string(value)));
                }
            }
            i += 1;
        }
        self.pages.push(data);
    }
}
