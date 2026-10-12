// files management like reading, writing, and deleting files
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn write_primes_to_file(primes: &[u128], filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    for prime in primes {
        writeln!(writer, "{}", prime)?;
    }

    Ok(())
}