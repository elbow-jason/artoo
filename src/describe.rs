#[cfg(test)]
use difference::{Changeset, Difference};

pub struct Describe {
    pub string: String,
    pub depth: usize,
    pub indenation: String,
    _eat_next_indent: bool,
}

impl Describe {
    pub fn new() -> Self {
        Describe {
            string: String::new(),
            depth: 0,
            indenation: String::from("  "),
            _eat_next_indent: false,
        }
    }

    pub fn indent(&mut self) {
        if self._eat_next_indent {
            self._eat_next_indent = false;
            return;
        }
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

    pub fn eat_next_indent(&mut self) {
        self._eat_next_indent = true;
    }

    #[cfg(test)]
    pub fn assert_eq(&self, expected: &str) {
        let cs = Changeset::new(self.as_str().trim(), expected.trim(), "\n");
        match &cs.diffs[..] {
            [Difference::Same(_)] => (),
            _ => panic!("describe was different:\n{}", cs),
        }
    }
}

pub trait Describer {
    fn describe(&self, describe: &mut Describe);
}
