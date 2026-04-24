use std::fs::File;
use std::io::{self, Read, BufRead};
use std::path::Path;
use std::env;



fn zero_pass_check(initial_pos: &mut i32 ,dir: String,number: &i32) -> i32{
	
	let mut  pos: i32 = *initial_pos;
	let mut times:i32 = 0;

	let dir_slice:&str = &dir;
	match dir_slice{
		"L" =>{pos -= number;},
		"R" => {pos += number;},
		_ => panic!("Incorrect direction")
	}

	match pos{
		v if v > 99 => {times += pos /100;
				pos = pos % 100;
				},
		v if v < 0 => {
				pos = pos.abs();
				times += pos/100;
				pos = 100 - pos % 100;	
				},
		v if v == 0 =>{
				times += 1;
				},
		_ => {},
	}
	
	*initial_pos = pos;

	times
}

fn main() {

        let args: Vec<String> = env::args().collect();

        let path = Path::new(&args[1]);
        let file = File::open(&path).unwrap();
        let reader = io::BufReader::new(file);

	let mut initial:i32 = 50;
	let mut count:i32 = 0;


	println!("The dial starts at {}",initial);


	for line in reader.lines() {

		let line = line.unwrap();//now we have the raw string in the line var

		let dir = String::from(line.chars().next().unwrap());//this holds the direction
		let number = &line[1..].parse::<i32>().unwrap();//this holds the rotation amount
		count += zero_pass_check(&mut initial,dir,number )
	
				}
		println!("{}",count);

}


#[cfg(test)]
	mod tests {
		use super::*;

		//checking if function logic can reliably see if the rotations pass trough zero	
		#[test]
		fn right_rotation() {
			let result = zero_pass_check(&50,"R".to_string(),&151);
			assert_eq!(result,2);
		}
		#[test]
		fn left_rotation() {
			let result = zero_pass_check(&50,"L".to_string(),&-350);
			assert_eq!(result,4);
		}

}

