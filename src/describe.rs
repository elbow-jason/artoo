pub struct Describe {
    pub string: String,
    pub depth: usize,
    pub indenation: String,
}

impl Describe {
    pub fn new() -> Self {
        Describe {
            string: String::new(),
            depth: 0,
            indenation: String::from("  "),
        }
    }

    pub fn indent(&mut self) {
        for _ in 0..self.depth {
            self.string.push_str(&self.indenation);
        }
    }

    pub fn push_str(&mut self, s: &str) {
        self.string.push_str(s);
    }

    pub fn nest<F: Fn(&mut Describe)>(&mut self, f: F) {
        self.depth += 1;
        f(self);
        self.depth -= 1;
    }

    pub fn as_str(&self) -> &str {
        &self.string
    }
}

pub trait Describer {
    fn describe(&self, describe: &mut Describe);
}

pub struct Describe {
    pub string: String,
    pub depth: usize,
    pub indenation: String,
}

impl Describe {
    pub fn new() -> Self {
        Describe {
            string: String::new(),
            depth: 0,
            indenation: String::from("  "),
        }
    }

    pub fn indent(&mut self) {
        for _ in 0..self.depth {
            self.string.push_str(&self.indenation);
        }
    }

    pub fn push_str(&mut self, s: &str) {
        self.string.push_str(s);
    }

    pub fn nest<F: Fn(&mut Describe)>(&mut self, f: F) {
        self.depth += 1;
        f(self);
        self.depth -= 1;
    }

    pub fn as_str(&self) -> &str {
        &self.string
    }
}
