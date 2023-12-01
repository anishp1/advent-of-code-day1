fn main() {
    let input_file = include_str!("input.txt");
    part2(input_file);
}

fn part2(input_file: &str) {
    let mut sum = 0;

    for line in input_file.lines() {
        let mut string_list: [char; 2] = ['0', '0'];
        let mut current_word: Vec<char> = Vec::new();
        for (_i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                string_list[0] = c;
                break;
            } else {
                current_word.push(c);
                let result = match_word(&mut current_word);
                if result != 'n' {
                    string_list[0] = result;
                    current_word.clear();
                    break;
                }
            }
        }
        current_word.clear();
        for (_i, c) in line.chars().rev().enumerate() {
            if c.is_digit(10) {
                string_list[1] = c;
                break;
            } else {
                current_word.insert(0, c);
                let result = match_word(&mut current_word);
                if result != 'n' {
                    string_list[1] = result;
                    current_word.clear();
                    break;
                }
            }
        }
        if string_list.len() >= 1 {
            let valid_string: String = string_list.iter().collect();
            let result = valid_string.parse::<i32>().unwrap();
            println!("{}", result);

            sum += result;
        }
    }
    println!("Sum={}", sum);
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

        println!("{}", result);

        sum += result;
    }

    println!("Sum={}", sum);
}
