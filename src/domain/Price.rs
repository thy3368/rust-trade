pub struct Price {
    pub order_no: String,
}

impl Price {
    // 构造函数
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
    // getter
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn age(&self) -> u32 {
        self.age
    }
    // setter
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_age(&mut self, age: u32) {
        self.age = age;
    }
}
