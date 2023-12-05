use std::collections::HashMap;

fn main() {
    let color_map = create_map();
    let input = include_str!("input.txt").trim();
    let mut total = 0;

    for line in input.lines() {
        let colon_position = line.find(":").unwrap();
        let game_id: &i32 = &line[5..colon_position].to_string().parse().unwrap();
        let sets = &line[colon_position + 2..]
            .split("; ")
            .collect::<Vec<&str>>();
        let mut valid = true;
        for &set in sets {
            let cubes: Vec<&str> = set.split(", ").collect();
            for cube in cubes {
                let split: Vec<&str> = cube.split(" ").collect();
                let total_cubes: usize = split.first().unwrap().parse().unwrap();
                let color = split.last().unwrap().to_string();
                if let Some(&max_value) = color_map.get(&color) {
                    if total_cubes > max_value {
                        valid = false;
                        continue;
                    }
                }
            }
        }

        println!("{} {}", game_id, valid);
        if valid {
            total = total + game_id;
        }
        println!("{}", total);
    }
}

fn create_map() -> HashMap<String, usize> {
    let mut map = HashMap::new();
    map.insert("blue".to_string(), 14);
    map.insert("green".to_string(), 13);
    map.insert("red".to_string(), 12);

    return map;
}
