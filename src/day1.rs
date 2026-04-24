use std::fs::File;
use std::io::{self, Read, BufRead};
use std::path::Path;
use std::env;



fn main() {


        let args: Vec<String> = env::args().collect();


        let path = Path::new(&args[1]);
        let file = File::open(&path).unwrap();
        let reader = io::BufReader::new(file);


        let mut initial:i32 = 50;
        let mut count:i32 = 0;

        println!("The dial starts at {}",initial);

        for line in reader.lines() {


                let line = line.unwrap();//this unwraps the result

                let s = String::from(line.chars().next().unwrap());

                let number_string = &line[1..];
                let was_initial = initial;

                if s == "L" {
                        initial -= number_string.parse::<i32>().unwrap();
                } else if  s == "R" {
                        initial += number_string.parse::<i32>().unwrap();
                }

                if initial > 99{
                        initial = initial % 100;
                }
                if initial < 0 {
                        initial = (100 + initial % 100) % 100;
                }

                if initial == 0{
                        count = count + 1;}


                println!("The dial is rotated {} to point at {} count:{}",line, initial,count);
}

}
