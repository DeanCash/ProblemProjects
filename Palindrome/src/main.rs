#![allow(non_snake_case, dead_code, non_upper_case_globals, unused_mut, unused_variables, unused_parens, unused_imports, unused_must_use)]

use std::io;
use yansi::{Paint, Color};

static bottom: i32  = 100;
static top: i32     = 999;

fn main() {
    if !(Paint::is_enabled()) {
        Paint::disable();
    }

    let mut palindromes = Vec::<i32>::new();

    for i in bottom..=top {
        for j in bottom..=top {

            if is_palindrome(i, j) {
                println!(" i:{} * j:{} = {}", i, j, i * j);
                palindromes.push(i * j);
            }
        }
    }

    let largest = palindromes.iter().max().unwrap_or(&-1);
    println!("\nThe biggest {} is: {}",
        Paint::new("palindrome").fg(Color::Magenta).bold(),
        Paint::new(largest).fg(Color::Green).bold()
    );
    let stdin = io::stdin();
    io::stdin().read_line(&mut String::new());
}

fn is_palindrome(num1: i32, num2: i32) -> bool {
    let result = (num1 * num2).to_string();
    let length = result.len();

    let (mut left, mut right) = result.split_at( (length / 2) );

    // if length is uneven
    if length % 2 == 1 {
        let (middle, right) = right.split_at(1);
    }
    // reverse right string
    let right = right.chars().rev().collect::<String>();

    if left == right { true } else { false }
}
