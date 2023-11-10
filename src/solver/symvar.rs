pub struct SymVar {
    pub name: String,
    pub var0: String,
    pub prev: String,
}


impl SymVar {
    pub fn new(s: String) -> Self {
        SymVar {
            name: s.clone(),
            var0: s.clone(),
            prev: s.clone()
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} = {}", &self.name, &self.var0)
    }
}