use std::fs::File;
use std::io::{self, Read, BufRead};
use std::path::Path;




fn main() {

        let path = Path::new("input");
        let file = File::open(&path).unwrap();
        let reader = io::BufReader::new(file);


        let mut initial:i32 = 50;
        let mut count:i32 = 0;

        for line in reader.lines() {


                let line = line.unwrap();//this unwraps the result

                let s = String::from(line.chars().next().unwrap());

                let number_string = &line[1..];


                if s == "L" {
                        initial -= number_string.parse::<i32>().unwrap();
                } else {
                        initial += number_string.parse::<i32>().unwrap();
                }

                if initial > 99{
                        initial %= 100;
                }
                if initial < 0 {
                        initial = 100 - initial % 100;
                }

                if initial == 0{
                        count = count + 1;}

}
                println!("the answer is : {}", count);

}
