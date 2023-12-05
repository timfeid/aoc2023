fn main() {
    let input = include_str!("input.txt").trim();
    let mut total = 0;
    for line in input.lines() {
        let (mut first, mut last) = (None, None);

        for char in line.chars() {
            if is_digit(&char) {
                if first == None {
                    first = Some(char);
                }
                last = Some(char);
            }
        }

        let number: i32 = format!("{}{}", first.expect("no first"), last.expect("no last"))
            .parse()
            .expect("couldn't find number");
        total = total + number;
    }
    print!("{:?}", total);
}

fn is_digit(char: &char) -> bool {
    if char.is_numeric() {
        return true;
    }

    return false;
}
