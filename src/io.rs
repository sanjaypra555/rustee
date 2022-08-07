use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use crate::{Args, Error};

pub fn pipe(args: Args) -> Result<(), Error>{
    let file = args.file.ok_or(Error::FileNotProvided)?;
    let mut output_file =
        fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(!args.append)
            .append(args.append)
            .open(file)?;

    let mut unique_lines = get_unique_lines(args.unique, &mut output_file)?;

    let mut stdin = io::stdin().lock();
    let mut line = String::new();

    loop {
        line.clear();
        let n_bytes = stdin.read_line(&mut line)?;
        if n_bytes == 0 { break; }

        let word = trim_newline(&line);
        if args.unique && unique_lines.contains(word) { continue; }
        output_file.write_all(line.as_bytes())?;
        if args.unique {
            unique_lines.insert(String::from(word));
        }
        print!("{}", line);
    }
    Ok(())
}

fn get_unique_lines(is_unique: bool, output_file: &mut File) -> Result<HashSet<String>, Error> {
    let mut unique_lines = HashSet::new();
    if is_unique {
        let reader = BufReader::new(output_file);
        for line in reader.lines() {
            unique_lines.insert(line?);
        }
    }
    return Ok(unique_lines);
}


fn trim_newline(s: &str) -> &str {
    if s.ends_with('\n') {
        if s.ends_with("\r\n") {
            return &s[0..s.len() - 2];
        }
        return &s[0..s.len() - 1];
    }
    return &s[..];
}
