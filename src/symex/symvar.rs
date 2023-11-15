pub struct SymVar {
    pub name: String,
    pub var0: String,
    pub min: i128,
    pub max: u128,
    pub prev: String,
}


impl SymVar {
    pub fn new(s: String, t: String) -> Self {
        SymVar {
            name: s.clone(),
            var0: s.clone(),
            min: Self::set_min(t.clone()),
            max: Self::set_max(t.clone()),
            prev: s.clone()
        }
    }

    pub fn new_assign(s: String, t: String, assign: String) -> Self {
        SymVar {
            name: s.clone(),
            var0: assign.clone(),
            min: Self::set_min(t.clone()),
            max: Self::set_max(t.clone()),
            prev: assign.clone()
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} = {}; range {}..{}", &self.name, &self.var0, &self.min, &self.max)
    }

    fn set_min(t: String) -> i128 {
        if !t.contains("i") {
            return 0;
        }

        else {
            let s = t.replace("i", "");
            let exp: u32 = s.parse().expect("could not resolve to an int");
            let base: i128 = 2;
            return base.pow(exp - 1) * -1;
        }
    }

    fn set_max(t: String) -> u128 {
        let base: u128 = 2;
        if !t.contains("i") {
            let s = t.replace("u", "");
            let exp: u32 = s.parse().expect("could not resolve to an int");
            return base.pow(exp) - 1;
        }

        else {
            let s = t.replace("i", "");
            let exp: u32 = s.parse().expect("could not resolve to an int");
            return base.pow(exp - 1) - 1
        }
    }
}