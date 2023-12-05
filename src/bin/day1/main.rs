use std::{cmp, collections::HashMap};

fn main() {
    let input = include_str!("input.txt").trim();

    let mut total = 0;
    let numbers = number_map();
    let max_word_length = numbers.keys().map(|k| k.len()).max().unwrap() + 1;

    for line in input.lines() {
        let mut numbers_in_line = Vec::new();

        for i in 0..line.len() {
            let char = line.chars().nth(i).unwrap();
            if char.is_numeric() {
                numbers_in_line.push(char.to_digit(10).unwrap() as i32);
            }
            for j in i..cmp::min(line.len() + 1, i + max_word_length) {
                let word = line[i..j].to_string();
                if let Some(&number) = numbers.get(&word) {
                    numbers_in_line.push(number);
                }
            }
        }

        if let (Some(first), Some(last)) = (numbers_in_line.first(), numbers_in_line.last()) {
            // println!("{} {:?} {:?}", line, first, last);
            total += first * 10 + last;
        }
    }

    println!("{:?}", total);
}

fn number_map() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);
    map.insert("three".to_string(), 3);
    map.insert("four".to_string(), 4);
    map.insert("five".to_string(), 5);
    map.insert("six".to_string(), 6);
    map.insert("seven".to_string(), 7);
    map.insert("eight".to_string(), 8);
    map.insert("nine".to_string(), 9);

    return map;
}
