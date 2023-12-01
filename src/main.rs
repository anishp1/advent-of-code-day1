fn main() {
    let input_file = include_str!("input.txt");
    part2(input_file);
}

fn part2(input_file: &str) {
    let mut sum = 0;
    for line in input_file.lines() {
        let mut valid_string: String = String::from("");
        let first = find_first(line);
        if first != 'n' {
            valid_string.push(first);
            let last = find_last(line);
            if last != 'n' {
                valid_string.push(last);
                let result = valid_string.parse::<i32>().unwrap();
                sum += result;
            }
        }
    }
    println!("Sum={}", sum);
}



fn find_first(line: &str) -> char {
    let mut current_word: Vec<char> = Vec::new();
    for (_i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            return c;
        } else {
            current_word.push(c);
            let result = match_word(&mut current_word);
            if result != 'n' {
                return result;
            }
        }
    }
    return 'n'
}

fn find_last(line: &str) -> char {
    let mut current_word: Vec<char> = Vec::new();
    for (_i, c) in line.chars().rev().enumerate() {
        if c.is_digit(10) {
            return c;
        } else {
            current_word.insert(0, c);
            let result = match_word(&mut current_word);
            if result != 'n' {
                return result;
            }
        }
    }
    return 'n';
}

fn match_word(current_word: &mut Vec<char>) -> char {
    let word: String = current_word.iter().collect();
    if word.contains("zero") {
        return '0';
    } else if word.contains("one") {
        return '1';
    } else if word.contains("two") {
        return '2';
    } else if word.contains("three") {
        return '3';
    } else if word.contains("four") {
        return '4';
    } else if word.contains("five") {
        return '5';
    } else if word.contains("six") {
        return '6';
    } else if word.contains("seven") {
        return '7';
    } else if word.contains("eight") {
        return '8';
    } else if word.contains("nine") {
        return '9';
    }
    return 'n';
}

fn part1(input_file: &str) {
    let mut sum = 0;
    for line in input_file.lines() {
        let mut digit_string = String::from("");
        for (_i, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => {
                    digit_string.push(c);
                    break;
                }
                _ => {}
            }
        }

        for (_i, c) in line.chars().rev().enumerate() {
            match c {
                '0'..='9' => {
                    digit_string.push(c);
                    break;
                }
                _ => {}
            }
        }

        let result = digit_string.parse::<i32>().unwrap();

        sum += result;
    }

    println!("Sum={}", sum);
}
