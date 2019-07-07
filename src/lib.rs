pub fn solve(input: &str) -> String {
    input.to_string()
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
