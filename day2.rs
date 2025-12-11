use std::fs::File;
use std::io::{self, Read, BufRead};
use std::path::Path;
use std::env;


fn main() -> io::Result<()> {

        let args: Vec<String> = env::args().collect();

        let path = Path::new(&args[1]);
        let file = File::open(&path)?;
        let mut reader = io::BufReader::new(file);

        let mut buffer = Vec::new();


        while reader.read_until(b',',&mut buffer)? != 0 {
                //gotta convert the bytes to a string
                if let Ok(s) = String::from_utf8(buffer.clone()) {
                        let token = s.trim_end_matches(',');
                        println!("Got a string: {}", token);
                }
                buffer.clear();
        }

        Ok(())
}
