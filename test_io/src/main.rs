use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

fn main() -> std::io::Result<()> {
    // Creates or truncates an existing file. Writes data to the file. Data
    // must be flushed or else it will not be written to the file.
    let f = File::create("log.txt")?;
    let mut writer = BufWriter::new(f);
    writer.write_all(b"hello world\nmy name is brian\n")?;
    writer.flush()?;

    // Opens the file in reading mode. It will also be writeable since we are
    // not setting file open options. Uses a buffered read to read data from the
    // file.
    let f = File::open("log.txt")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let len = reader.read_line(&mut line)?;
    println!(
        "First line is {} bytes long and has the value {}",
        len, line
    );

    // Opens the same file with append write only mode and writes an additiona
    // line to the file
    let f = OpenOptions::new()
        .read(false)
        .write(true)
        .append(true)
        .open("log.txt")?;
    let mut writer = BufWriter::new(f);
    writer.write_all(b"next line\n")?;
    writer.flush()?;

    // Open the same file in read-only mode and read 10 bytes from the beginning
    // of the file and output the stuff as as a raw buffer.
    let mut f = OpenOptions::new().read(true).write(false).open("log.txt")?;
    let mut buf = [0; 10];
    f.read(&mut buf)?;
    println!("{:?}", buf);

    // Open the same file in read-only mode and read 8-byte chunks output the
    // 8-byte chunk along the way. We terminate when the last read chunk was
    // less than the recommend chunk size
    let mut f = OpenOptions::new().read(true).write(false).open("log.txt")?;
    let mut buf = [0; 8];
    loop {
        let size = f.read(&mut buf)?;
        let string_val = String::from_utf8(buf.to_vec()).unwrap_or(String::new());
        print!("{}", string_val);
        if (size) < 8 {
            break;
        }
    }

    // convert a u8 slice into hex. You would think this functionality would
    // exist, but I couldn't find it anywhere, which is super annoying.
    let buf: &[u8] = &[104, 101, 108, 11, 32, 119, 111, 114, 208];
    let hex_str = buf.iter().map(|x| format!("{:02x}", x)).collect::<String>();
    println!("{}", hex_str);

    Ok(())
}
