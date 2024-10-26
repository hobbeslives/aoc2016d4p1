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
    let file = File::open("data/input.txt").expect("Error opening input file.");
    let reader = BufReader::new(file);
    let mut sector_id_sum: u32 = 0;
    for line in reader.lines().map(|l| l.unwrap()) {
        let (_, [encrypted_name, sector_id, checksum]) = re.captures(&line).unwrap().extract();
        let calculated_checksum = get_checksum(encrypted_name);
        // println!("{} {} {} {} {}", &line, encrypted_name, sector_id, checksum, calculated_checksum);
        if calculated_checksum == checksum {
            let sector_id = sector_id.parse::<u16>().unwrap();
            sector_id_sum += sector_id as u32;
            let decrypted_room_name = decrypt_room_name(encrypted_name, sector_id);
            if decrypted_room_name.contains("north") {
                println!("{} {}", decrypted_room_name, sector_id);
            }
        }
    }

    println!("The sum of the sector IDs of the real rooms is {}.", sector_id_sum);

    println!("Done");
}

fn decrypt_room_name(encrypted_name: &str, sector_id: u16) -> String {
    let mut room_name = String::new();
    for c in encrypted_name.chars() {
        if c == '-' {
            room_name.push(' ');
        } else {
            room_name.push(rotate_character(c, sector_id));
        }
    }
    room_name
}

fn rotate_character(c: char, n: u16) -> char {
    (((c as u16 - 'a' as u16 + n) % 26) as u8 + 'a' as u8) as char
}

fn get_checksum(encrypted_name: &str) -> String {
    let mut char_count: HashMap<char, i32> = HashMap::new();
    for char in encrypted_name.chars() {
        if char != '-' {
            char_count.entry(char).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    let mut for_sorting = char_count.iter().map(|(c, v)| format!("{:02}{:02}{}", v, 'z' as u8 - *c as u8, c)).collect::<Vec<String>>();
    for_sorting.sort();
    for_sorting.reverse();
    let mut checksum = String::new();
    for value in for_sorting[..5].iter() {
      checksum.push(value.chars().skip(4).next().unwrap());
    }
    checksum
}