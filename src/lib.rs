pub struct Dice {
    up: isize,
    left: isize,
    right: isize,
}

impl Dice {
    pub fn new() -> Self {
        Dice {
            up: 1,
            left: 3,
            right: 5,
        }
    }

    pub fn north(&mut self) {
        let orig_up = self.up;
        self.up = self.right;
        self.right = 7 - orig_up;
    }

    pub fn east(&mut self) {
        let orig_up = self.up;
        self.up = self.left;
        self.left = 7 - orig_up;
    }

    pub fn south(&mut self) {
        self.north();
        self.north();
        self.north();
    }

    pub fn west(&mut self) {
        self.east();
        self.east();
        self.east();
    }
}

pub fn solve(input: &str) -> String {
    let mut result = String::new();

    let mut dice = Dice::new();
    result.push_str(&format!("{}", dice.up));
    for c in input.chars() {
        match c {
            'N' => dice.north(),
            'E' => dice.east(),
            'S' => dice.south(),
            'W' => dice.west(),
            _ => unimplemented!(),
        };
        result.push_str(&format!("{}", dice.up));
    }

    result
}

#[cfg(test)]
mod tests {
    use std::io::{Result, BufReader, BufRead};
    use std::fs::File;

    #[test]
    fn test_solve() -> Result<()> {
        let file = File::open("test.tsv")?;
        let buf_reader = BufReader::new(file);
        for line in buf_reader.lines() {
            let line = line?;
            let cols = line.split('\t').collect::<Vec<&str>>();
            assert_eq!(crate::solve(cols[1]), cols[2]);
        }

        Ok(())
    }
}
