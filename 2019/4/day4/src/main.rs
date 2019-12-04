// 6 digit password
// value within range given in the input
// two adjacent digits are the same
// digits never decrease

// input
// 158126-624574
// **how many** valid passwords are there
 
// part 1 : 1665
// part 2 : 1131

use std::vec;

fn main() {
    let min = 158126;
    let max = 624574;

    let mut count = 0;

    assert!(is_valid(vec![1, 1, 1, 1, 1, 1]));
    assert!(!is_valid(vec![2, 2, 3, 4, 5, 0]));
    assert!(!is_valid(vec![1, 2, 3, 4, 5, 6]));

    assert!(is_valid_part2(vec![1, 1, 2, 2, 3, 3]));
    assert!(!is_valid_part2(vec![1, 2, 3, 4, 4, 4]));
    assert!(is_valid_part2(vec![1, 1, 1, 1, 4, 4]));
    assert!(is_valid_part2(vec![2, 2, 3, 4, 5, 6]));
    assert!(is_valid_part2(vec![2, 2, 3, 4, 5, 5]));
    assert!(is_valid_part2(vec![1, 2, 3, 4, 5, 5]));

    for password in min..max {
        let d = digits(password);
        if is_valid(d)
        {
            // println!("valid: {}", password);
            count += 1;
        }
    }

    println!("part 1: {}", count);
    count = 0;

    // part 2
    for password in min..max {
        let d = digits(password);
        if is_valid_part2(d) {
            // println!("valid: {}", password);
            count += 1;
        }
    }
    println!("part 2: {}", count);
}

fn digits(mut input: isize) -> Vec<isize> {
    let mut digits = Vec::new();
    while input > 0 {
        digits.insert(0, input % 10);
        input /= 10;
    }
    return digits;
}

fn is_valid(digits: Vec<isize>) -> bool {
    let is_right_length = digits.len() == 6; // 6 digits
                                             // don't have to check range, this is handled already
    let mut has_adjacent = false; // false until proven true
    let mut prev = -1;
    for digit in digits {
        // as long as this digit is not the first digit
        if prev != -1 {
            // compare the next digit to the prev digit
            if digit < prev {
                return false;
            } else if digit == prev {
                has_adjacent = true;
            }
        }
        prev = digit;
    }
    return is_right_length && has_adjacent;
}

fn is_valid_part2(digits: Vec<isize>) -> bool {
    let is_right_length = digits.len() == 6; // 6 digits
                                             // don't have to check range, this is handled already
    let mut has_adjacent = false; // false until proven true
    let mut adjacent_counter = 1; // default 1, every 1 value is adjacent to itself

    let mut prev = -1;
    for digit in digits {
        // as long as this digit is not the first digit
        if prev != -1 {
            // compare the next digit to the prev digit
            if digit < prev {
                return false;
            } else if digit == prev {
                adjacent_counter += 1;
            }
            // non adjacent values
            else {
                if adjacent_counter == 2 {
                    // a group of two values were found
                    has_adjacent = true;
                }
                adjacent_counter = 1;
            }
        }
        prev = digit;
    }

    // iterated through all of the digits, at the end
    if adjacent_counter == 2 {
        has_adjacent = true;
    }

    return is_right_length && has_adjacent;
}
