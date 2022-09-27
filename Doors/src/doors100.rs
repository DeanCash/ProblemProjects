#![allow(unused_variables, unused_mut, unused_must_use, dead_code, path_statements)]

// 100 doors
// https://rosettacode.org/wiki/100_doors

use std::fs::{write};

// true  = open
// false = closed

pub fn doors(before_file: &str, after_file: &str) {
    
    let mut doors = [false; 100];

    println!("{:?}\n\n", {doors});
    dump_to_file(&doors, before_file);

    for i in 1..=doors.len() {
        for j in 0..doors.len() {
            let door = doors.get_mut(j).expect("out of bounds");
            if j % i == 0 {
                *door = !*door;
            }
        }
    }

    println!("{:?}", {doors});
    dump_to_file(&doors, after_file);
}












fn dump_to_file(list: &[bool; 100], file_name: &str) {
    let mut res_list: Vec<String> = Vec::new();
    for (i, item) in list.into_iter().enumerate() {
        res_list.push(format!("{} - {}", i+1, item.to_string()));
    }

    write(file_name, res_list.join("\n"));
}


