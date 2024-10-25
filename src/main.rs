/*
--- Day 4: Security Through Obscurity ---
Finally, you come across an information kiosk with a list of rooms. Of course, the list is encrypted and full of decoy 
data, but the instructions to decode the list are barely hidden nearby. Better remove the decoy data first.

Each room consists of an encrypted name (lowercase letters separated by dashes) followed by a dash, a sector ID, and a 
checksum in square brackets.

A room is real (not a decoy) if the checksum is the five most common letters in the encrypted name, in order, with ties 
broken by alphabetization. For example:

  - aaaaa-bbb-z-y-x-123[abxyz] is a real room because the most common letters are a (5), b (3), and then a tie between 
    x, y, and z, which are listed alphabetically.
  - a-b-c-d-e-f-g-h-987[abcde] is a real room because although the letters are all tied (1 of each), the first five are 
    listed alphabetically.
  - not-a-real-room-404[oarel] is a real room.
  - totally-real-room-200[decoy] is not.

Of the real rooms from the list above, the sum of their sector IDs is 1514.

What is the sum of the sector IDs of the real rooms?
*/

use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};
use regex::Regex;

fn main() {
    let re = Regex::new(r"^([a-z-]*)([0-9]*)\[([^\]]*)]$").unwrap();
    let file = File::open("data/test_input.txt").expect("Error opening input file.");
    let reader = BufReader::new(file);
    for line in reader.lines().map(|l| l.unwrap()) {
        let (_, [encrypted_name, sector_id, checksum]) = re.captures(&line).unwrap().extract();
        let calculated_checksum = get_checksum(encrypted_name);
        println!("{} {} {} {}", &line, encrypted_name, sector_id, checksum);
    }

    println!("Done");
}

fn get_checksum(encrypted_name: &str) -> String {
    let mut char_count: HashMap<char, i32> = HashMap::new();
    for char in encrypted_name.chars() {
        if char != '-' {
            char_count.entry(char).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    println!("{:?}", char_count);
    " ".to_owned()
}