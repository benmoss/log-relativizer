use anyhow::{Context, Result};
use std::io::{self, BufRead, BufReader, Write};
use time::{format_description::well_known, Duration, PrimitiveDateTime};

use regex::Regex;

const ISO8601: &str = r"[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(\.[0-9]+)?([Zz]|([\+-])([01]\d|2[0-3]):?([0-5]\d)?)?";

fn main() -> Result<()> {
    if let Err(e) = run() {
        if e.downcast::<io::Error>()?.kind() == std::io::ErrorKind::BrokenPipe {
            return Ok(());
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    let reader = BufReader::new(io::stdin());
    let mut stdout = io::stdout();
    let mut t0: Option<PrimitiveDateTime> = None;
    let re = Regex::new(ISO8601).unwrap();
    for line in reader.lines() {
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
        writeln!(stdout, "{}", line)?
    }
    Ok(())
}
