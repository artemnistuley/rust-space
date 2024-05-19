use std::io::{ BufRead, BufReader, Read, Write, Result };

fn count_lines<R: Read>(reader: R) -> usize {
    let buf_reader = BufReader::new(reader);
    buf_reader.lines().count()
}

fn log<W: Write>(writer: &mut W, msg: &str) -> Result<()> {
    writer.write_all(msg.as_bytes())?;
    writer.write_all("\n".as_bytes())
}

// fn main() -> Result<()> {
//     let slice: &[u8] = b"foo\nbar\n\nbaz\n";
//     println!("lines in slice: {}", count_lines(slice));

//     let file = std::fs::File::open(std::env::current_exe()?)?;
//     println!("lines in file: {}", count_lines(file));
//     Ok(())
// }

fn main() -> Result<()> {
    let mut buffer = Vec::new();
    log(&mut buffer, "Hello")?;
    log(&mut buffer, "World")?;
    println!("Logged: {:?}", buffer);
    Ok(())
}
