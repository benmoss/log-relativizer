use anyhow::{Context, Result};
use std::io::{self, BufRead, BufReader, Write};
use time::{format_description::well_known, Duration, PrimitiveDateTime};

use regex::Regex;

const ISO8601: &str = r"[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(\.[0-9]+)?([Zz]|([\+-])([01]\d|2[0-3]):?([0-5]\d)?)?";

fn main() -> Result<()> {
    let reader = BufReader::new(io::stdin());
    if let Err(e) = run(reader, io::stdout()) {
        if e.downcast::<io::Error>()?.kind() == std::io::ErrorKind::BrokenPipe {
            return Ok(());
        }
    }
    Ok(())
}

fn run<R: BufRead, W: Write>(input: R, mut output: W) -> Result<()> {
    let mut t0: Option<PrimitiveDateTime> = None;
    let re = Regex::new(ISO8601).unwrap();
    for line in input.lines() {
        let line = line.unwrap();
        let result = match re.captures(&line) {
            Some(caps) => caps.get(0).context("no match"),
            None => continue,
        }?;
        let parsed = PrimitiveDateTime::parse(result.as_str(), &well_known::Rfc3339);
        let parsed = parsed?;

        let diff = match t0 {
            Some(t0) => parsed - t0,
            None => {
                t0 = Some(parsed);
                Duration::ZERO
            }
        };
        let diff = format!("{}", diff);
        let line = re.replace(&line, diff);
        writeln!(output, "{}", line)?
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufReader, Cursor},
    };

    #[test]
    fn it_works() {
        let expected = "[0s] starting the tests
[1s928ms] test 1
[2s196ms] some logs and crap
[2s461ms] test 2
[3s53ms] whoa so many logs
[3s458ms] tests complete!
";
        let reader = Box::new(BufReader::new(File::open("examples/test.log").unwrap()));
        let mut output = Cursor::new(vec![0; 15]);

        assert!(crate::run(reader, &mut output).is_ok());
        let actual = std::str::from_utf8(output.get_ref()).expect("should be valid");
        assert_eq!(expected, actual);
    }
}
